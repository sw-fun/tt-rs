//! Robot accessor methods.

use tt_rs_core::WidgetId;

use super::{Action, Robot, RobotState};

impl Robot {
    /// Returns the robot's state.
    pub fn state(&self) -> RobotState {
        self.state
    }

    /// Returns the recorded actions.
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Returns the pattern this robot expects.
    pub fn pattern(&self) -> Option<WidgetId> {
        self.pattern
    }

    /// Returns what the robot is currently holding during execution.
    pub fn held_widget(&self) -> Option<WidgetId> {
        self.held_widget_id
    }

    /// Creates a copy of this robot with a new ID.
    pub fn copy_robot(&self) -> Robot {
        Robot::new_with(self.pattern, self.actions.clone(), self.next_robot)
    }

    /// Returns human-readable descriptions of all recorded actions.
    pub fn action_descriptions(&self) -> Vec<String> {
        self.actions.iter().map(|a| a.description()).collect()
    }

    /// Returns a formatted summary of the robot's training.
    pub fn training_summary(&self) -> String {
        if self.actions.is_empty() {
            "No training recorded".to_string()
        } else {
            let steps: Vec<String> = self
                .actions
                .iter()
                .enumerate()
                .map(|(i, a)| format!("{}. {}", i + 1, a.description()))
                .collect();
            steps.join("\n")
        }
    }
}
