//! Robot click and training operations.

use tt_rs_core::WidgetId;
use tt_rs_drag::DropEvent;
use tt_rs_robot::RobotState;

use crate::robot_exec::execute_robot;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Copy a widget to create a new instance.
pub fn copy_widget(widget: &WidgetItem) -> WidgetItem {
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

/// Handle robot click: toggle training or execute.
pub fn handle_robot_click(state: &mut AppState, id: WidgetId, event: &DropEvent) -> bool {
    if !state
        .widgets
        .get(&id)
        .map(|w| w.is_robot())
        .unwrap_or(false)
    {
        return false;
    }

    let old_pos = state.positions.get(&id).copied();
    let dist = old_pos
        .map(|p| ((p.x - event.position.x).powi(2) + (p.y - event.position.y).powi(2)).sqrt())
        .unwrap_or(0.0);

    if dist >= 10.0 {
        return false;
    }

    let (robot_state, has_actions) = get_robot_info(state, id);
    handle_state_change(state, id, robot_state, has_actions);

    if let Some(pos) = old_pos {
        state.positions.insert(id, pos);
    }
    true
}

fn get_robot_info(state: &AppState, id: WidgetId) -> (RobotState, bool) {
    state
        .widgets
        .get(&id)
        .and_then(|w| match w {
            WidgetItem::Robot(r) => Some((r.state(), !r.actions().is_empty())),
            _ => None,
        })
        .unwrap_or((RobotState::Idle, false))
}

fn handle_state_change(state: &mut AppState, id: WidgetId, rs: RobotState, has_actions: bool) {
    match rs {
        RobotState::Training => stop_robot_training(state, id),
        RobotState::Idle if has_actions => execute_robot(state, id),
        RobotState::Idle => start_robot_training(state, id),
        RobotState::Working => log::info!("Robot {} is working", id),
    }
}

fn stop_robot_training(state: &mut AppState, id: WidgetId) {
    if let Some(WidgetItem::Robot(robot)) = state.widgets.get_mut(&id) {
        robot.stop_training();
    }
    state.training_robot_id = None;
}

fn start_robot_training(state: &mut AppState, id: WidgetId) {
    if let Some(old_id) = state.training_robot_id {
        stop_robot_training(state, old_id);
    }
    if let Some(WidgetItem::Robot(r)) = state.widgets.get_mut(&id) {
        r.start_training();
    }
    state.training_robot_id = Some(id);
}
