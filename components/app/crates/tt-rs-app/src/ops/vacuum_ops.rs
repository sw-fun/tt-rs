//! Vacuum tool operations: erase widgets.

use tt_rs_core::WidgetId;
use tt_rs_drag::DropEvent;
use tt_rs_hit_test::{find_box_hole_at, find_widget_at_excluding};
use tt_rs_robot::Action;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle vacuum drop: erase from hole or delete widget.
pub fn handle_vacuum_drop(
    state: &mut AppState,
    id: WidgetId,
    mx: f64,
    my: f64,
    event: &DropEvent,
) -> bool {
    if !state
        .widgets
        .get(&id)
        .map(|w| w.is_vacuum())
        .unwrap_or(false)
    {
        return false;
    }

    if try_vacuum_hole(state, id, mx, my, event) {
        return true;
    }
    if try_vacuum_widget(state, id, mx, my, event) {
        return true;
    }

    state.positions.insert(id, event.position);
    true
}

fn try_vacuum_hole(
    state: &mut AppState,
    vac_id: WidgetId,
    mx: f64,
    my: f64,
    event: &DropEvent,
) -> bool {
    let (box_id, hole) = match find_box_hole_at(mx, my) {
        Some(h) => h,
        None => return false,
    };

    let erased = state
        .boxes
        .get_mut(&box_id)
        .and_then(|b| b.clear_hole(hole));
    if let Some(wid) = erased {
        state.record_action(Action::Remove {
            path: format!("box:{}:hole:{}", box_id, hole),
        });
        state.widget_in_box.remove(&wid);
        state.widgets.remove(&wid);
        state.positions.insert(vac_id, event.position);
        return true;
    }
    false
}

fn try_vacuum_widget(
    state: &mut AppState,
    vac_id: WidgetId,
    mx: f64,
    my: f64,
    event: &DropEvent,
) -> bool {
    let (target_id, is_box) = match find_widget_at_excluding(mx, my, vac_id) {
        Some(t) => t,
        None => return false,
    };

    if is_box || !is_deletable(state, target_id) {
        return false;
    }

    state.widgets.remove(&target_id);
    state.positions.remove(&target_id);
    state.positions.insert(vac_id, event.position);
    true
}

fn is_deletable(state: &AppState, id: WidgetId) -> bool {
    state
        .widgets
        .get(&id)
        .map(|w| {
            // Tools are never deletable
            if matches!(
                w,
                WidgetItem::Vacuum(_) | WidgetItem::Wand(_) | WidgetItem::Robot(_)
            ) {
                return false;
            }
            // Copy sources (stacks) are never deletable - they're palette items
            if w.is_copy_source() {
                return false;
            }
            true
        })
        .unwrap_or(false)
}
