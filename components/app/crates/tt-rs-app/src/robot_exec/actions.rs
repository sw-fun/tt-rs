//! Individual robot action handlers.

use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::Position;
use tt_rs_number::{ArithOperator, Number};

use super::path_parse::{
    parse_bird_path, parse_box_hole_path, parse_sensor_path, parse_widget_path,
    parse_workspace_type_path,
};
use crate::state::AppState;
use crate::widget_item::WidgetItem;

pub fn execute_arithmetic(state: &mut AppState, op: char, num: i64, den: i64, path: &str) {
    // Try parsing as workspace type first (e.g., "workspace:number")
    let target_id = if let Some(widget_type) = parse_workspace_type_path(path) {
        // Find first widget of this type in workspace (not a copy source)
        state
            .positions
            .keys()
            .find(|&id| {
                // Must not be in a box
                if state.widget_in_box.contains_key(id) {
                    return false;
                }
                // Must match the type and not be a copy source
                state
                    .widgets
                    .get(id)
                    .map(|w| w.type_name() == widget_type && !w.is_copy_source())
                    .unwrap_or(false)
            })
            .copied()
    } else {
        // Try parsing as specific widget ID
        parse_widget_path(path)
    };

    let target_id = match target_id {
        Some(id) => id,
        None => {
            state.status_message = Some("Robot error: No number found in workspace".to_string());
            log::warn!("Robot: Cannot find arithmetic target '{}'", path);
            return;
        }
    };

    let operator = char_to_op(op);
    let tool = Number::rational(num, den as u64).with_operator(operator);

    if let Some(WidgetItem::Number(n)) = state.widgets.get(&target_id) {
        let mut target = n.clone();
        let before_value = target.numerator();
        if target.apply(&tool).is_some() {
            let after_value = target.numerator();
            state.widgets.insert(target_id, WidgetItem::Number(target));
            log::info!(
                "Robot: {} {} {} = {}",
                before_value,
                operator.symbol(),
                num,
                after_value
            );
        } else {
            state.status_message = Some(format!(
                "Robot error: Cannot apply {} to number",
                operator.symbol()
            ));
            log::warn!("Robot: Arithmetic operation failed");
        }
    } else {
        state.status_message = Some("Robot error: Target is not a number".to_string());
        log::warn!("Robot: Target {} is not a number", target_id);
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
            } else {
                state.status_message = Some(format!("Robot error: Box hole {} is empty", hole));
                log::warn!("Robot: Box hole {} is empty", hole);
            }
        } else {
            state.status_message = Some("Robot error: Cannot find box".to_string());
            log::warn!("Robot: Cannot find box {}", box_id);
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
            state.status_message = Some(format!(
                "Robot error: No '{}' found in workspace",
                widget_type
            ));
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
        } else {
            state.status_message = Some("Robot error: Cannot find widget to pick up".to_string());
            log::warn!("Robot: Widget {} not found", widget_id);
        }
    } else {
        state.status_message = Some(format!("Robot error: Invalid path '{}'", path));
        log::warn!("Robot: Cannot parse path '{}'", path);
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
            state.status_message = Some("Robot error: Nothing to drop (empty hands)".to_string());
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
        } else {
            state.status_message = Some("Robot error: Cannot find box to drop into".to_string());
            log::warn!("Robot: Cannot find box {}", box_id);
        }
        return;
    }

    // Try parsing as bird path (deliver to bird)
    if let Some(bird_id) = parse_bird_path(path) {
        // Deliver widget to bird
        if let Some(WidgetItem::Bird(bird)) = state.widgets.get(&bird_id) {
            if let Some(nest_id) = bird.nest_id() {
                // Get the widget to deliver
                let dropped = state.widgets.remove(&widget_id);
                if let Some(widget) = dropped {
                    // Add to nest's queue
                    if let Some(WidgetItem::Nest(nest)) = state.widgets.get_mut(&nest_id) {
                        nest.receive(widget.to_boxed_widget());
                        log::info!(
                            "Robot delivered widget {} to bird {} -> nest {}",
                            widget_id,
                            bird_id,
                            nest_id
                        );
                    }
                }
            } else {
                state.status_message = Some("Robot error: Bird has no paired nest".to_string());
                log::warn!("Robot: Bird {} has no nest", bird_id);
            }
        } else {
            state.status_message = Some("Robot error: Cannot find bird".to_string());
            log::warn!("Robot: Cannot find bird {}", bird_id);
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
    } else {
        state.status_message = Some(format!("Robot error: Invalid drop target '{}'", path));
        log::warn!("Robot: Cannot parse drop path '{}'", path);
    }
}

/// Execute a ClickSensor action: click a sensor to produce a number.
pub fn execute_click_sensor(state: &mut AppState, path: &str) {
    // Try parsing as workspace type (e.g., "workspace:sensor")
    let sensor_id = if let Some(widget_type) = parse_workspace_type_path(path) {
        // Find first sensor of this type in workspace
        state
            .positions
            .keys()
            .find(|&id| {
                state
                    .widgets
                    .get(id)
                    .map(|w| w.type_name() == widget_type)
                    .unwrap_or(false)
            })
            .copied()
    } else {
        // Try parsing as specific sensor ID
        parse_sensor_path(path)
    };

    let sensor_id = match sensor_id {
        Some(id) => id,
        None => {
            state.status_message = Some("Robot error: No sensor found".to_string());
            log::warn!("Robot: Cannot find sensor '{}'", path);
            return;
        }
    };

    // Get the sensor and produce a number
    let sensor = match state.widgets.get(&sensor_id) {
        Some(WidgetItem::Sensor(s)) => s.clone(),
        _ => {
            state.status_message = Some("Robot error: Target is not a sensor".to_string());
            log::warn!("Robot: Target {} is not a sensor", sensor_id);
            return;
        }
    };

    // Produce a new number widget
    let number = sensor.produce_number();
    let number_id = number.id();
    let number_value = number.numerator(); // Get actual value for logging

    // Position the new number near the sensor
    let sensor_pos = state.positions.get(&sensor_id).copied().unwrap_or_default();
    let number_pos = Position::new(sensor_pos.x + 90.0, sensor_pos.y);

    state.widgets.insert(number_id, WidgetItem::Number(number));
    state.positions.insert(number_id, number_pos);

    log::info!(
        "Robot: Clicked sensor, produced {} (widget {})",
        number_value,
        number_id
    );
}

fn char_to_op(c: char) -> ArithOperator {
    match c {
        '+' => ArithOperator::Add,
        '-' => ArithOperator::Subtract,
        '*' => ArithOperator::Multiply,
        '/' => ArithOperator::Divide,
        '%' => ArithOperator::Modulo,
        _ => ArithOperator::Add,
    }
}

fn copy_item(w: &WidgetItem) -> WidgetItem {
    match w {
        WidgetItem::Number(n) => WidgetItem::Number(n.copy_number()),
        WidgetItem::Text(t) => WidgetItem::Text(t.copy_text()),
        WidgetItem::Scales(s) => WidgetItem::Scales(s.copy_scales()),
        WidgetItem::Sensor(s) => WidgetItem::Sensor(s.copy_sensor()),
        WidgetItem::Vacuum(v) => WidgetItem::Vacuum(v.copy_vacuum()),
        WidgetItem::Wand(w) => WidgetItem::Wand(w.copy_wand()),
        WidgetItem::Magnifier(m) => WidgetItem::Magnifier(m.copy_magnifier()),
        WidgetItem::Robot(r) => WidgetItem::Robot(r.copy_robot()),
        WidgetItem::Nest(n) => WidgetItem::Nest(n.copy_nest()),
        WidgetItem::Bird(b) => WidgetItem::Bird(b.copy_bird()),
        WidgetItem::DropZone(dz) => WidgetItem::DropZone(dz.copy_dropzone()),
    }
}
