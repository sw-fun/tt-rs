//! Sensor operations: produce numbers on click.

use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::{DropEvent, Position};
use tt_rs_robot::Action;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle sensor click: produce a Number widget.
pub fn handle_sensor_click(state: &mut AppState, id: WidgetId, event: &DropEvent) -> bool {
    let sensor = match state.widgets.get(&id) {
        Some(WidgetItem::Sensor(s)) => s.clone(),
        _ => return false,
    };

    // Check if this was a click (minimal drag distance) not a drag
    let old_pos = state.positions.get(&id).copied();
    let dist = old_pos
        .map(|p| ((p.x - event.position.x).powi(2) + (p.y - event.position.y).powi(2)).sqrt())
        .unwrap_or(0.0);

    if dist >= 10.0 {
        // This was a drag, not a click - just update position
        return false;
    }

    // Record training action if a robot is in training mode
    state.record_action(Action::ClickSensor {
        path: "workspace:sensor".to_string(),
    });

    // Produce a new number widget
    let number = sensor.produce_number();
    let number_id = number.id();

    // Position the new number slightly offset from the sensor
    let sensor_pos = old_pos.unwrap_or_default();
    let number_pos = Position::new(sensor_pos.x + 90.0, sensor_pos.y);

    state.widgets.insert(number_id, WidgetItem::Number(number));
    state.positions.insert(number_id, number_pos);

    // Restore sensor position (it was a click, not a drag)
    if let Some(pos) = old_pos {
        state.positions.insert(id, pos);
    }

    log::info!("Sensor {} produced number at {:?}", id, number_pos);
    true
}
