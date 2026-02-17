//! Box hole drop operations.

use tt_rs_core::WidgetId;
use tt_rs_drag::{DropEvent, Position};
use tt_rs_hit_test::find_box_hole_at;
use tt_rs_robot::Action;

use super::robot_ops::copy_widget;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle dropping widget into box hole.
pub fn handle_box_hole_drop(
    state: &mut AppState,
    id: WidgetId,
    mx: f64,
    my: f64,
    event: &DropEvent,
) -> bool {
    let (box_id, hole) = match find_box_hole_at(mx, my) {
        Some(h) => h,
        None => return false,
    };

    if !state.widgets.contains_key(&id) {
        return false;
    }

    eject_existing(state, box_id, hole, event);

    // Record PickUp action: where did widget come from?
    // Check if it was in a box hole
    if let Some((src_box, src_hole)) = state.widget_in_box.get(&id).copied() {
        state.record_action(Action::PickUp {
            path: format!("box:{}:hole:{}", src_box, src_hole),
        });
    } else {
        // Widget was in workspace - use type-based path for generalization
        let widget_type = state
            .widgets
            .get(&id)
            .map(|w| w.type_name())
            .unwrap_or("unknown");
        state.record_action(Action::PickUp {
            path: format!("workspace:{}", widget_type),
        });
    }

    // Record Drop action
    state.record_action(Action::Drop {
        path: format!("box:{}:hole:{}", box_id, hole),
    });

    let place_id = get_widget_to_place(state, id);
    if let Some(b) = state.boxes.get_mut(&box_id) {
        b.place_in_hole(hole, place_id);
    }
    state.widget_in_box.insert(place_id, (box_id, hole));
    state.update_scales_in_box(box_id);
    true
}

fn eject_existing(state: &mut AppState, box_id: WidgetId, hole: usize, event: &DropEvent) {
    let existing = state
        .boxes
        .get(&box_id)
        .and_then(|b| b.widget_in_hole(hole));
    if let Some(old_id) = existing {
        if let Some(b) = state.boxes.get_mut(&box_id) {
            b.clear_hole(hole);
        }
        state.widget_in_box.remove(&old_id);
        state.positions.insert(
            old_id,
            Position::new(event.mouse_position.x + 50.0, event.mouse_position.y),
        );
    }
}

fn get_widget_to_place(state: &mut AppState, id: WidgetId) -> WidgetId {
    let is_tool = matches!(
        state.widgets.get(&id),
        Some(
            WidgetItem::Scales(_)
                | WidgetItem::Vacuum(_)
                | WidgetItem::Wand(_)
                | WidgetItem::Robot(_)
        )
    );

    if is_tool {
        if let Some(w) = state.widgets.get(&id) {
            let copied = copy_widget(w);
            let copy_id = copied.id();
            state.widgets.insert(copy_id, copied);
            return copy_id;
        }
    }
    state.positions.remove(&id);
    id
}
