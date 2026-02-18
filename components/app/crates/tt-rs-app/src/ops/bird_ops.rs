//! Bird and Nest operations: message passing.
//!
//! ToonTalk Bird/Nest semantics:
//! - Birds are created paired with a nest (via "hatching")
//! - Drop a widget ON a bird to give it a message
//! - The bird delivers the message to its paired nest
//! - Messages queue on the nest until retrieved (click nest to take top)

use tt_rs_core::WidgetId;
use tt_rs_drag::{DropEvent, Position};
use tt_rs_hit_test::find_widget_at_excluding;
use tt_rs_robot::Action;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle when any widget is dropped - check if it landed on a bird.
/// This is called from the general drop handler to check for bird delivery.
pub fn handle_drop_on_bird(state: &mut AppState, id: WidgetId, mx: f64, my: f64) -> bool {
    // Get the widget being dropped
    let dropped = match state.widgets.get(&id) {
        Some(w) if !w.is_copy_source() => w.clone(),
        _ => return false,
    };

    // Check if dropped on a bird
    if let Some((target_id, _is_box)) = find_widget_at_excluding(mx, my, id) {
        if let Some(WidgetItem::Bird(bird)) = state.widgets.get(&target_id) {
            // Can't deliver birds, nests, or tools
            if matches!(
                dropped,
                WidgetItem::Bird(_)
                    | WidgetItem::Nest(_)
                    | WidgetItem::Vacuum(_)
                    | WidgetItem::Wand(_)
            ) {
                return false;
            }

            // Bird must be paired with a nest
            let nest_id = match bird.nest_id() {
                Some(id) => id,
                None => {
                    log::info!("Bird {} has no nest to deliver to", target_id);
                    return false;
                }
            };

            // Record training actions if a robot is being trained
            // Record PickUp: where did widget come from?
            let widget_type = dropped.type_name();
            state.record_action(Action::PickUp {
                path: format!("workspace:{}", widget_type),
            });
            // Record Drop to bird
            state.record_action(Action::Drop {
                path: format!("bird:{}", target_id),
            });

            // Remove the dropped widget from workspace (bird consumes it)
            state.widgets.remove(&id);
            state.positions.remove(&id);

            // Add message to the nest's queue (message goes INTO nest, not as separate widget)
            if let Some(WidgetItem::Nest(nest)) = state.widgets.get_mut(&nest_id) {
                nest.receive(dropped.to_boxed_widget());
            }

            // Update bird to flying state (animation would go here)
            if let Some(WidgetItem::Bird(b)) = state.widgets.get_mut(&target_id) {
                b.start_flying();
            }

            log::info!("Bird {} delivered message to nest {}", target_id, nest_id);
            return true;
        }
    }
    false
}

/// Handle bird drop - just positioning, birds don't do anything special when dropped.
/// (In ToonTalk, you drop things ON birds, not the bird on things)
pub fn handle_bird_drop(_state: &mut AppState, _id: WidgetId, _mx: f64, _my: f64) -> bool {
    // Birds don't have special drop behavior - they just get repositioned
    false
}

/// Handle nest drop - just positioning.
/// (Nests receive messages when birds deliver, not via dropping)
pub fn handle_nest_drop(_state: &mut AppState, _id: WidgetId, _mx: f64, _my: f64) -> bool {
    // Nests don't have special drop behavior - they just get repositioned
    false
}

/// Handle nest click: take the top message if nest has any.
/// A "click" is detected when the widget barely moved (< 10 pixels).
pub fn handle_nest_click(state: &mut AppState, id: WidgetId, event: &DropEvent) -> bool {
    // Check if this is a nest
    let is_nest = state
        .widgets
        .get(&id)
        .map(|w| matches!(w, WidgetItem::Nest(_)))
        .unwrap_or(false);

    if !is_nest {
        return false;
    }

    // Check if this is a click (not much movement) using start_position from event
    let start_pos = event.start_position;
    let final_pos = event.position;
    let dist = ((start_pos.x - final_pos.x).powi(2) + (start_pos.y - final_pos.y).powi(2)).sqrt();

    if dist >= 10.0 {
        return false;
    }

    // Take the top message from the nest
    let taken = if let Some(WidgetItem::Nest(nest)) = state.widgets.get_mut(&id) {
        nest.take()
    } else {
        None
    };

    if let Some(message) = taken {
        // Convert Box<dyn Widget> back to WidgetItem and add to workspace
        let widget_item = WidgetItem::from_boxed_widget(message);
        let new_id = widget_item.id();

        // Position the extracted widget near the nest
        let new_pos = Position::new(start_pos.x + 80.0, start_pos.y);
        state.positions.insert(new_id, new_pos);
        state.widgets.insert(new_id, widget_item);

        log::info!("Took message {} from nest {}", new_id, id);

        // Restore nest to its original position (since it was a click, not a drag)
        state.positions.insert(id, start_pos);

        true
    } else {
        // No messages to take, restore position
        state.positions.insert(id, start_pos);
        false
    }
}
