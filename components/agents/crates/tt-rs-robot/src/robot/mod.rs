//! Robot struct and operations.

mod mutators;
mod ops;
mod types;

use tt_rs_core::WidgetId;

pub use types::{Action, RobotState};

/// A robot widget that can be trained to automate tasks.
///
/// Robots watch user demonstrations and learn patterns.
/// When given matching input, they replay the recorded actions.
#[derive(Debug, Clone)]
pub struct Robot {
    pub(crate) id: WidgetId,
    /// Current state of the robot.
    pub(crate) state: RobotState,
    /// The pattern this robot expects (optional - starts empty).
    pub(crate) pattern: Option<WidgetId>,
    /// Recorded actions to perform.
    pub(crate) actions: Vec<Action>,
    /// Next robot in chain (optional).
    pub(crate) next_robot: Option<WidgetId>,
    /// Widget currently being held by robot during execution.
    pub(crate) held_widget_id: Option<WidgetId>,
}

impl Robot {
    /// Creates a new idle robot.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            state: RobotState::Idle,
            pattern: None,
            actions: Vec::new(),
            next_robot: None,
            held_widget_id: None,
        }
    }

    /// Creates a robot with specified values.
    pub(crate) fn new_with(
        pattern: Option<WidgetId>,
        actions: Vec<Action>,
        next: Option<WidgetId>,
    ) -> Self {
        Self {
            id: WidgetId::new(),
            state: RobotState::Idle,
            pattern,
            actions,
            next_robot: next,
            held_widget_id: None,
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}
