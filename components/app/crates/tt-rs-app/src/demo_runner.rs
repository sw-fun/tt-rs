//! Demo animation runner for "Show Me" feature.
//!
//! Processes demo steps with proper timing to animate a cursor
//! through the demonstration, showing users how to perform actions.

use tt_rs_core::WidgetId;

use crate::demo_ops::resolve_target;
use crate::state::AppState;
use crate::workspace::DemoStep;

/// State of the demo animation.
#[derive(Clone, PartialEq)]
pub struct DemoState {
    /// Whether the demo is currently playing.
    pub is_playing: bool,
    /// Current cursor X position (screen coordinates).
    pub cursor_x: f64,
    /// Current cursor Y position (screen coordinates).
    pub cursor_y: f64,
    /// Whether the cursor is currently dragging.
    pub is_dragging: bool,
    /// Current transition duration for smooth movement.
    pub transition_ms: u32,
    /// Current step index in the demo.
    pub step_index: usize,
    /// The demo steps being played.
    pub steps: Vec<DemoStep>,
    /// Widget being dragged (set on DragStart, cleared on DragEnd).
    pub dragged_widget_id: Option<WidgetId>,
    /// Whether dragged widget is a box (vs regular widget).
    pub dragged_is_box: bool,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            is_playing: false,
            cursor_x: 100.0, // Start at a visible position
            cursor_y: 100.0,
            is_dragging: false,
            transition_ms: 0,
            step_index: 0,
            steps: Vec::new(),
            dragged_widget_id: None,
            dragged_is_box: false,
        }
    }
}

/// Offset for workspace content area (below header).
pub const WORKSPACE_OFFSET_Y: f64 = 60.0;

/// Resolve semantic targets in demo steps to concrete coordinates.
/// Call this before starting demo playback to pre-resolve all MoveToTarget steps.
pub fn resolve_steps(steps: &[DemoStep], app_state: &AppState) -> Vec<DemoStep> {
    steps
        .iter()
        .filter_map(|step| match step {
            DemoStep::MoveToTarget { target, duration } => {
                // Resolve semantic target to coordinates
                resolve_target(target, app_state).map(|(x, y)| DemoStep::MoveTo {
                    x,
                    y,
                    duration: *duration,
                })
            }
            // Keep other steps as-is
            other => Some(other.clone()),
        })
        .collect()
}

/// Process the next demo step and return the new state.
/// Returns None if the demo is complete.
pub fn process_next_step(state: &DemoState) -> Option<DemoState> {
    if !state.is_playing || state.step_index >= state.steps.len() {
        return None;
    }

    let step = &state.steps[state.step_index];
    let mut new_state = state.clone();
    new_state.step_index += 1;

    match step {
        DemoStep::Wait { .. } => {
            // No state change, just advance the index
        }
        DemoStep::MoveTo { x, y, duration } => {
            new_state.cursor_x = *x;
            new_state.cursor_y = *y + WORKSPACE_OFFSET_Y;
            new_state.transition_ms = *duration;
        }
        DemoStep::MoveToTarget { .. } => {
            // Should be resolved before playback - skip if not resolved
            log::warn!("MoveToTarget step not resolved - skipping");
        }
        DemoStep::DragStart => {
            new_state.is_dragging = true;
        }
        DemoStep::DragEnd => {
            new_state.is_dragging = false;
        }
        DemoStep::Click => {
            // Click is handled by the app's demo handler - just mark state
            // The actual click logic happens in the app based on cursor position
        }
    }

    // Check if demo is complete
    if new_state.step_index >= new_state.steps.len() {
        new_state.is_playing = false;
        new_state.is_dragging = false;
    }

    Some(new_state)
}

/// Get the delay for the current step (how long to wait before processing).
pub fn get_step_delay(state: &DemoState) -> u32 {
    if state.step_index >= state.steps.len() {
        return 0;
    }

    match &state.steps[state.step_index] {
        DemoStep::Wait { duration } => *duration,
        DemoStep::MoveTo { duration, .. } => *duration + 200, // Extra time for animation
        DemoStep::MoveToTarget { duration, .. } => *duration + 200, // Should be resolved
        DemoStep::DragStart => 400,
        DemoStep::DragEnd => 400,
        DemoStep::Click => 500, // Brief pause for click animation
    }
}
