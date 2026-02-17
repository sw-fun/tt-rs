//! Individual robot action handlers.

use tt_rs_core::WidgetId;
use tt_rs_drag::Position;
use tt_rs_number::{ArithOperator, Number};

use super::path_parse::{parse_box_hole_path, parse_widget_path, parse_workspace_type_path};
use crate::state::AppState;
use crate::widget_item::WidgetItem;

pub fn execute_arithmetic(state: &mut AppState, op: char, num: i64, den: i64, path: &str) {
    let target_id = match parse_widget_path(path) {
        Some(id) => id,
        None => return,
    };
    let operator = char_to_op(op);
    let tool = Number::rational(num, den as u64).with_operator(operator);

    if let Some(WidgetItem::Number(n)) = state.widgets.get(&target_id) {
        let mut target = n.clone();
        if target.apply(&tool).is_some() {
            state.widgets.insert(target_id, WidgetItem::Number(target));
        }
    }
}

pub fn execute_copy(state: &mut AppState, path: &str) {
    let target_id = match parse_widget_path(path) {
        Some(id) => id,
        None => return,
    };
    let widget = match state.widgets.get(&target_id) {
        Some(w) => w.clone(),
        None => return,
    };
    let copy = copy_item(&widget);
    let pos = state.positions.get(&target_id).copied().unwrap_or_default();
    state
        .positions
        .insert(copy.id(), Position::new(pos.x + 30.0, pos.y + 30.0));
    state.widgets.insert(copy.id(), copy);
}

pub fn execute_remove(state: &mut AppState, path: &str) {
    let (box_id, hole) = match parse_box_hole_path(path) {
        Some(p) => p,
        None => return,
    };
    if let Some(b) = state.boxes.get_mut(&box_id) {
        if let Some(wid) = b.clear_hole(hole) {
            state.widget_in_box.remove(&wid);
            state.widgets.remove(&wid);
        }
    }
}

/// Execute a PickUp action: remove widget from source and store in robot.
pub fn execute_pickup(state: &mut AppState, robot_id: WidgetId, path: &str) {
    // Try parsing as box hole first
    if let Some((box_id, hole)) = parse_box_hole_path(path) {
        // Pick up from box hole
        if let Some(b) = state.boxes.get_mut(&box_id) {
            if let Some(widget_id) = b.clear_hole(hole) {
                state.widget_in_box.remove(&widget_id);
                // Store in robot's held widget
                if let Some(WidgetItem::Robot(r)) = state.widgets.get_mut(&robot_id) {
                    r.pick_up(widget_id);
                    log::info!("Robot picked up widget {} from box hole", widget_id);
                }
            }
        }
        return;
    }

    // Try parsing as workspace type (e.g., "workspace:number")
    if let Some(widget_type) = parse_workspace_type_path(path) {
        // Find first widget of this type in workspace (has position, not in box)
        let widget_id = state
            .positions
            .keys()
            .find(|&id| {
                // Must not be in a box
                if state.widget_in_box.contains_key(id) {
                    return false;
                }
                // Must match the type
                state
                    .widgets
                    .get(id)
                    .map(|w| w.type_name() == widget_type && !w.is_copy_source())
                    .unwrap_or(false)
            })
            .copied();

        if let Some(widget_id) = widget_id {
            state.positions.remove(&widget_id);
            if let Some(WidgetItem::Robot(r)) = state.widgets.get_mut(&robot_id) {
                r.pick_up(widget_id);
                log::info!(
                    "Robot picked up {} widget {} from workspace",
                    widget_type,
                    widget_id
                );
            }
        } else {
            log::warn!("No {} widget found in workspace", widget_type);
        }
        return;
    }

    // Try parsing as specific widget ID (fallback)
    if let Some(widget_id) = parse_widget_path(path) {
        if state.widgets.contains_key(&widget_id) {
            state.positions.remove(&widget_id);
            if let Some(WidgetItem::Robot(r)) = state.widgets.get_mut(&robot_id) {
                r.pick_up(widget_id);
                log::info!("Robot picked up widget {} from workspace", widget_id);
            }
        }
    }
}

/// Execute a Drop action: place robot's held widget at target.
pub fn execute_drop(state: &mut AppState, robot_id: WidgetId, path: &str) {
    // Get held widget from robot
    let held_id = {
        if let Some(WidgetItem::Robot(r)) = state.widgets.get_mut(&robot_id) {
            r.drop_held()
        } else {
            None
        }
    };

    let widget_id = match held_id {
        Some(id) => id,
        None => {
            log::warn!("Robot has no held widget to drop");
            return;
        }
    };

    // Try parsing as box hole
    if let Some((box_id, hole)) = parse_box_hole_path(path) {
        // Place in box hole
        if let Some(b) = state.boxes.get_mut(&box_id) {
            b.place_in_hole(hole, widget_id);
            state.widget_in_box.insert(widget_id, (box_id, hole));
            state.update_scales_in_box(box_id);
            log::info!(
                "Robot dropped widget {} into box {} hole {}",
                widget_id,
                box_id,
                hole
            );
        }
        return;
    }

    // Try parsing as workspace position (widget path used as position reference)
    if let Some(ref_id) = parse_widget_path(path) {
        // Place near the reference widget
        let pos = state.positions.get(&ref_id).copied().unwrap_or_default();
        state
            .positions
            .insert(widget_id, Position::new(pos.x + 30.0, pos.y + 30.0));
        log::info!("Robot dropped widget {} near widget {}", widget_id, ref_id);
    }
}

fn char_to_op(c: char) -> ArithOperator {
    match c {
        '+' => ArithOperator::Add,
        '-' => ArithOperator::Subtract,
        '*' => ArithOperator::Multiply,
        '/' => ArithOperator::Divide,
        _ => ArithOperator::Add,
    }
}

fn copy_item(w: &WidgetItem) -> WidgetItem {
    match w {
        WidgetItem::Number(n) => WidgetItem::Number(n.copy_number()),
        WidgetItem::Text(t) => WidgetItem::Text(t.copy_text()),
        WidgetItem::Scales(s) => WidgetItem::Scales(s.copy_scales()),
        WidgetItem::Vacuum(v) => WidgetItem::Vacuum(v.copy_vacuum()),
        WidgetItem::Wand(w) => WidgetItem::Wand(w.copy_wand()),
        WidgetItem::Robot(r) => WidgetItem::Robot(r.copy_robot()),
        WidgetItem::Nest(n) => WidgetItem::Nest(n.copy_nest()),
        WidgetItem::Bird(b) => WidgetItem::Bird(b.copy_bird()),
        WidgetItem::DropZone(dz) => WidgetItem::DropZone(dz.copy_dropzone()),
    }
}
