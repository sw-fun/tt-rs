//! Magnifier tool operations: inspect widgets.

use tt_rs_core::WidgetId;
use tt_rs_drag::DropEvent;
use tt_rs_hit_test::find_widget_at_excluding;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle magnifier drop: inspect target widget.
pub fn handle_magnifier_drop(
    state: &mut AppState,
    id: WidgetId,
    mx: f64,
    my: f64,
    event: &DropEvent,
) -> bool {
    if !state
        .widgets
        .get(&id)
        .map(|w| w.is_magnifier())
        .unwrap_or(false)
    {
        return false;
    }

    if let Some((target_id, _is_box)) = find_widget_at_excluding(mx, my, id) {
        inspect_widget(state, target_id);
    }

    state.positions.insert(id, event.position);
    true
}

fn inspect_widget(state: &mut AppState, target_id: WidgetId) {
    let widget = match state.widgets.get(&target_id) {
        Some(w) => w,
        None => return,
    };

    let content = match widget {
        WidgetItem::Robot(r) => {
            let state_str = match r.state() {
                tt_rs_robot::RobotState::Idle => "Idle",
                tt_rs_robot::RobotState::Training => "Training",
                tt_rs_robot::RobotState::Working => "Working",
            };

            let actions = r.actions();
            let action_list = if actions.is_empty() {
                "No training recorded.".to_string()
            } else {
                actions
                    .iter()
                    .enumerate()
                    .map(|(i, a)| format!("{}. {}", i + 1, a.description()))
                    .collect::<Vec<_>>()
                    .join("\n")
            };

            let held = r
                .held_widget()
                .map(|id| format!("\n\nCurrently holding: widget {}", id))
                .unwrap_or_default();

            format!(
                "Robot Inspection\n\nState: {}\nActions: {}\n\n{}\n{}",
                state_str,
                actions.len(),
                action_list,
                held
            )
        }
        WidgetItem::Number(n) => {
            format!(
                "Number Inspection\n\nValue: {}\nOperator: {}\nCopy source: {}",
                n.display_value(),
                n.operator().symbol(),
                n.is_copy_source()
            )
        }
        WidgetItem::Nest(nest) => {
            let queue_count = nest.message_count();
            let mut content = format!(
                "Nest Inspection\n\nMessages queued: {}\nCopy source: {}\n",
                queue_count,
                nest.is_copy_source()
            );

            if queue_count > 0 {
                content.push_str("\nQueue (oldest first, FIFO):\n");
                // Display all items in the queue (up to 10)
                for (i, item) in nest.iter_messages().enumerate().take(10) {
                    if i == 0 {
                        content.push_str(&format!("1. {} (next out)\n", item.description()));
                    } else {
                        content.push_str(&format!("{}. {}\n", i + 1, item.description()));
                    }
                }
                if queue_count > 10 {
                    content.push_str(&format!("... and {} more\n", queue_count - 10));
                }
            }
            content
        }
        WidgetItem::Sensor(s) => {
            let sensor_type = match s.sensor_type() {
                tt_rs_sensor::SensorType::EpochMillis => "Time (epoch milliseconds)",
                tt_rs_sensor::SensorType::Random => "Random (0-999,999)",
            };
            format!(
                "Sensor Inspection\n\nType: {}\nCopy source: {}",
                sensor_type,
                s.is_copy_source()
            )
        }
        _ => {
            format!(
                "Widget Inspection\n\nType: {}\nID: {}",
                widget.type_name(),
                target_id
            )
        }
    };

    state.inspection_modal = Some(content);
}
