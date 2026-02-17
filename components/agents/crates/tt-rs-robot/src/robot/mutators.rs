//! Robot mutation methods.

use tt_rs_core::WidgetId;

use super::{Action, Robot, RobotState};

impl Robot {
    /// Starts training mode - robot begins recording actions.
    pub fn start_training(&mut self) {
        self.state = RobotState::Training;
        self.actions.clear();
    }

    /// Stops training mode - robot saves recorded actions.
    pub fn stop_training(&mut self) {
        self.state = RobotState::Idle;
    }

    /// Records an action during training.
    pub fn record_action(&mut self, action: Action) {
        if self.state == RobotState::Training {
            self.actions.push(action);
        }
    }

    /// Sets the pattern this robot expects.
    pub fn set_pattern(&mut self, pattern_id: WidgetId) {
        self.pattern = Some(pattern_id);
    }

    /// Starts working mode - robot executes actions.
    pub fn start_working(&mut self) {
        if !self.actions.is_empty() {
            self.state = RobotState::Working;
        }
    }

    /// Stops working mode.
    pub fn stop_working(&mut self) {
        self.state = RobotState::Idle;
        self.held_widget_id = None;
    }

    /// Picks up a widget (stores its ID).
    pub fn pick_up(&mut self, widget_id: WidgetId) {
        self.held_widget_id = Some(widget_id);
    }

    /// Drops the held widget and returns its ID.
    pub fn drop_held(&mut self) -> Option<WidgetId> {
        self.held_widget_id.take()
    }
}
