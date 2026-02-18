//! Demo operations for "Show Me" feature.
//!
//! Helper functions for performing actual widget operations during demo playback.

use tt_rs_core::WidgetId;
use tt_rs_hit_test::find_widget_at_excluding;
use yew::UseStateHandle;

use crate::ops::{handle_drop_on_bird, handle_dropzone_drop, handle_number_on_number};
use crate::state::AppState;
use crate::widget_item::WidgetItem;
use crate::workspace::DemoTarget;

/// Widget dimensions for center calculation.
const WIDGET_WIDTH: f64 = 50.0;
const WIDGET_HEIGHT: f64 = 50.0;
const BOX_HOLE_WIDTH: f64 = 50.0;
const BOX_HEIGHT: f64 = 80.0;
const DROPZONE_WIDTH: f64 = 200.0;
const DROPZONE_HEIGHT: f64 = 80.0;

/// Resolve a semantic DemoTarget to (x, y) center coordinates.
/// Returns None if the target cannot be found.
pub fn resolve_target(target: &DemoTarget, state: &AppState) -> Option<(f64, f64)> {
    match target {
        DemoTarget::Widget { name } => {
            // Look up widget by name
            let widget_id = state.widget_names.get(name)?;
            let pos = state.positions.get(widget_id)?;
            // Return center of widget
            Some((pos.x + WIDGET_WIDTH / 2.0, pos.y + WIDGET_HEIGHT / 2.0))
        }
        DemoTarget::Box { name } => {
            // Look up box by name
            let box_id = state.box_names.get(name)?;
            let pos = state.positions.get(box_id)?;
            let box_state = state.boxes.get(box_id)?;
            // Return center of box
            let box_width = box_state.num_holes as f64 * BOX_HOLE_WIDTH;
            Some((pos.x + box_width / 2.0, pos.y + BOX_HEIGHT / 2.0))
        }
        DemoTarget::BoxHole { name, hole } => {
            // Look up box by name, then find specific hole center
            let box_id = state.box_names.get(name)?;
            let pos = state.positions.get(box_id)?;
            let box_state = state.boxes.get(box_id)?;
            // Verify hole index is valid
            if *hole >= box_state.num_holes {
                return None;
            }
            // Calculate hole center (holes are arranged horizontally)
            let hole_x = pos.x + (*hole as f64 + 0.5) * BOX_HOLE_WIDTH;
            let hole_y = pos.y + BOX_HEIGHT / 2.0;
            Some((hole_x, hole_y))
        }
        DemoTarget::DropZone { role } => {
            // Look up dropzone by role
            let dropzone_id = state.dropzone_roles.get(role)?;
            let pos = state.positions.get(dropzone_id)?;
            // Return center of dropzone
            Some((pos.x + DROPZONE_WIDTH / 2.0, pos.y + DROPZONE_HEIGHT / 2.0))
        }
    }
}

/// Find a widget at the given screen coordinates.
/// Returns (widget_id, is_box) if found.
pub fn find_widget_at(x: f64, y: f64) -> Option<(WidgetId, bool)> {
    // Use a dummy WidgetId to not exclude anything
    let dummy_id = WidgetId::new();
    find_widget_at_excluding(x, y, dummy_id)
}

/// Perform a drop operation at the given coordinates.
/// This handles dropping widgets into box holes or onto drop zones.
/// Also records training actions if a robot is in training mode.
pub fn perform_drop(
    app_state: &UseStateHandle<AppState>,
    dirty: &UseStateHandle<bool>,
    dragged_id: WidgetId,
    _is_box: bool,
    x: f64,
    y: f64,
) {
    use tt_rs_robot::Action;

    // Check what's at the drop location
    if let Some((target_id, target_is_box)) = find_widget_at_excluding(x, y, dragged_id) {
        log::info!(
            "Demo drop: found target {:?} (is_box={})",
            target_id,
            target_is_box
        );

        let mut new_state = (**app_state).clone();

        if target_is_box {
            // Dropping onto a box - try to find which hole
            if let Some(hole_index) = find_box_hole_at(&new_state, target_id, x, y) {
                // Check if hole is empty before proceeding
                let hole_empty = new_state
                    .boxes
                    .get(&target_id)
                    .map(|b| b.widget_in_hole(hole_index).is_none())
                    .unwrap_or(false);

                if hole_empty {
                    // Record training actions if a robot is in training mode
                    // Record PickUp: where did widget come from?
                    if let Some((src_box, src_hole)) =
                        new_state.widget_in_box.get(&dragged_id).copied()
                    {
                        new_state.record_action(Action::PickUp {
                            path: format!("box:{}:hole:{}", src_box, src_hole),
                        });
                    } else {
                        // Widget was in workspace - use type-based path
                        let widget_type = new_state
                            .widgets
                            .get(&dragged_id)
                            .map(|w| w.type_name())
                            .unwrap_or("unknown");
                        new_state.record_action(Action::PickUp {
                            path: format!("workspace:{}", widget_type),
                        });
                    }
                    // Record Drop action
                    new_state.record_action(Action::Drop {
                        path: format!("box:{}:hole:{}", target_id, hole_index),
                    });

                    // Now place the widget
                    if let Some(box_state) = new_state.boxes.get_mut(&target_id) {
                        box_state.place_in_hole(hole_index, dragged_id);
                    }
                    new_state
                        .widget_in_box
                        .insert(dragged_id, (target_id, hole_index));
                    // Remove from free positions since it's now in a box
                    new_state.positions.remove(&dragged_id);
                    log::info!(
                        "Demo: placed widget {:?} in box {:?} hole {}",
                        dragged_id,
                        target_id,
                        hole_index
                    );
                    app_state.set(new_state);
                    dirty.set(true);
                }
            }
        } else {
            // Try arithmetic operation (number on number)
            if handle_number_on_number(&mut new_state, dragged_id, x, y) {
                log::info!("Demo: arithmetic operation completed");
                app_state.set(new_state);
                dirty.set(true);
                return;
            }

            // Try bird delivery (widget on bird)
            if handle_drop_on_bird(&mut new_state, dragged_id, x, y) {
                log::info!("Demo: bird delivery completed");
                app_state.set(new_state);
                dirty.set(true);
                return;
            }

            // Check if target is a drop zone
            if let Some(crate::widget_item::WidgetItem::DropZone(_)) =
                new_state.widgets.get(&target_id)
            {
                // Use the actual dropzone verification logic
                log::info!(
                    "Demo: attempting drop on dropzone {:?} at ({}, {})",
                    target_id,
                    x,
                    y
                );
                handle_dropzone_drop(&mut new_state, dragged_id, x, y);
                app_state.set(new_state);
                dirty.set(true);
            }
        }
    }
}

/// Perform a click operation at the given coordinates.
/// Handles robot clicks (toggle training/execute) and sensor clicks for demo playback.
pub fn perform_click(
    app_state: &UseStateHandle<AppState>,
    dirty: &UseStateHandle<bool>,
    x: f64,
    y: f64,
) {
    // Find widget at the click location
    if let Some((target_id, _is_box)) = find_widget_at(x, y) {
        let mut new_state = (**app_state).clone();

        // Check if it's a robot
        if let Some(crate::widget_item::WidgetItem::Robot(_)) = new_state.widgets.get(&target_id) {
            log::info!("Demo click: clicking on robot {:?}", target_id);
            handle_robot_click_demo(&mut new_state, target_id);
            app_state.set(new_state);
            dirty.set(true);
            return;
        }

        // Check if it's a sensor
        let sensor_clone = match new_state.widgets.get(&target_id) {
            Some(crate::widget_item::WidgetItem::Sensor(s)) => Some(s.clone()),
            _ => None,
        };
        if let Some(sensor) = sensor_clone {
            log::info!("Demo click: clicking on sensor {:?}", target_id);
            handle_sensor_click_demo(&mut new_state, target_id, sensor);
            app_state.set(new_state);
            dirty.set(true);
            return;
        }

        log::info!(
            "Demo click: clicked on non-interactive widget {:?}",
            target_id
        );
    }
}

/// Handle sensor click for demo: produce a number.
fn handle_sensor_click_demo(state: &mut AppState, id: WidgetId, sensor: tt_rs_sensor::Sensor) {
    use tt_rs_core::Widget;
    use tt_rs_drag::Position;
    use tt_rs_robot::Action;

    // Record training action if a robot is training
    state.record_action(Action::ClickSensor {
        path: "workspace:sensor".to_string(),
    });

    // Produce a new number widget
    let number = sensor.produce_number();
    let number_id = number.id();

    // Position the new number near the sensor
    let sensor_pos = state.positions.get(&id).copied().unwrap_or_default();
    let number_pos = Position::new(sensor_pos.x + 90.0, sensor_pos.y);

    state
        .widgets
        .insert(number_id, crate::widget_item::WidgetItem::Number(number));
    state.positions.insert(number_id, number_pos);

    log::info!(
        "Demo: Sensor {} produced number {} at {:?}",
        id,
        number_id,
        number_pos
    );
}

/// Handle robot click for demo: toggle training or execute.
/// Simplified version without distance checks.
fn handle_robot_click_demo(state: &mut AppState, id: WidgetId) {
    use crate::robot_exec::execute_robot;
    use crate::widget_item::WidgetItem;
    use tt_rs_robot::RobotState;

    // Get robot state and action info
    let (robot_state, has_actions) = state
        .widgets
        .get(&id)
        .and_then(|w| match w {
            WidgetItem::Robot(r) => Some((r.state(), !r.actions().is_empty())),
            _ => None,
        })
        .unwrap_or((RobotState::Idle, false));

    log::info!(
        "Demo robot click: state={:?}, has_actions={}",
        robot_state,
        has_actions
    );

    match robot_state {
        RobotState::Training => {
            // Stop training
            if let Some(WidgetItem::Robot(robot)) = state.widgets.get_mut(&id) {
                robot.stop_training();
                log::info!("Demo: stopped robot training");
            }
            state.training_robot_id = None;
        }
        RobotState::Idle if has_actions => {
            // Execute the robot
            log::info!("Demo: executing trained robot");
            execute_robot(state, id);
        }
        RobotState::Idle => {
            // Start training
            if let Some(old_id) = state.training_robot_id {
                if let Some(WidgetItem::Robot(robot)) = state.widgets.get_mut(&old_id) {
                    robot.stop_training();
                }
                state.training_robot_id = None;
            }
            if let Some(WidgetItem::Robot(r)) = state.widgets.get_mut(&id) {
                r.start_training();
                log::info!("Demo: started robot training");
            }
            state.training_robot_id = Some(id);
        }
        RobotState::Working => {
            log::info!("Demo: robot is working");
        }
    }
}

/// Find which hole of a box is at the given coordinates.
fn find_box_hole_at(state: &AppState, box_id: WidgetId, x: f64, _y: f64) -> Option<usize> {
    let box_pos = state.positions.get(&box_id)?;
    let box_state = state.boxes.get(&box_id)?;

    // Box holes are approximately 50px wide
    const HOLE_WIDTH: f64 = 50.0;
    const BOX_PADDING: f64 = 10.0;

    // Calculate relative position within box
    let rel_x = x - box_pos.x - BOX_PADDING;

    if rel_x < 0.0 {
        return None;
    }

    let hole_index = (rel_x / HOLE_WIDTH) as usize;

    // Verify hole index is valid
    if hole_index < box_state.num_holes {
        Some(hole_index)
    } else {
        None
    }
}

/// Copy a widget (for copy source dragging in demos).
/// Returns a new WidgetItem with a fresh ID.
pub fn copy_widget(widget: &WidgetItem, _source_id: &WidgetId) -> WidgetItem {
    match widget {
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
