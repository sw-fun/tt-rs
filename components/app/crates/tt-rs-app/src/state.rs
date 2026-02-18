//! Application state management.

use std::collections::HashMap;
use tt_rs_core::WidgetId;
use tt_rs_drag::Position;
use tt_rs_robot::Action;
use tt_rs_scales::Scales;
use tt_rs_ui::UserLevel;

use crate::box_state::BoxState;
use crate::demo;
use crate::widget_item::WidgetItem;
use crate::workspace::WidgetData;

/// Default workspace notes for tt1 (Basic) mode.
pub const TT1_DEFAULT_NOTES: &str = r#"Welcome to tt-rs Basic Mode!

NUMBERS: Click and drag number stacks to create copies. Numbers support +, -, *, / operations.

BOXES: Containers with holes. Drag items into holes. Drop a box on another's edge to join them. Drop on a number to split.

SCALES: Compare values. Drop numbers in adjacent box holes to see which is larger.

ROBOT: Train by demonstration. Click to start training, perform actions, click again to stop.

TOOLS: Wand copies items. Vacuum removes items."#;

/// Default workspace notes for tt2 (Messaging) mode.
pub const TT2_DEFAULT_NOTES: &str = r#"Welcome to tt-rs Messaging Mode!

This level introduces Birds and Nests for message passing.

BIRDS: Carriers that deliver items to their paired nest. Drop an item on a bird to send it.

NESTS: Receivers where birds deliver items. Items accumulate in the nest until removed.

BIRD-NEST PAIRS: Each bird is bonded to exactly one nest. Create pairs from the palette.

MESSAGE PASSING: Use birds to send data between different parts of your program - this enables asynchronous communication.

All Basic Mode features (numbers, boxes, scales, robot, tools) are also available."#;

/// Default workspace notes for tt3 (Sensors) mode.
pub const TT3_DEFAULT_NOTES: &str = r#"Welcome to tt-rs Sensors Mode!

This level introduces Sensors for generating time and random values.

TIME SENSOR (Clock): Click to produce the current time as a number (epoch milliseconds). Useful for timestamps or measuring durations.

RANDOM SENSOR (Die): Click to produce a random number (0-999,999). Useful for games, simulations, or random choices.

USING WITH ROBOTS: Train robots to click sensors and process the numbers - create programs that react to time or make random decisions.

All previous features (numbers, boxes, scales, robot, tools, birds, nests) are also available."#;

/// Get default notes content for a user level.
pub fn default_notes_for_level(level: UserLevel) -> &'static str {
    match level {
        UserLevel::Tt1 => TT1_DEFAULT_NOTES,
        UserLevel::Tt2 => TT2_DEFAULT_NOTES,
        UserLevel::Tt3 => TT3_DEFAULT_NOTES,
    }
}

/// Application state.
#[derive(Clone)]
pub struct AppState {
    pub widgets: HashMap<WidgetId, WidgetItem>,
    pub boxes: HashMap<WidgetId, BoxState>,
    pub positions: HashMap<WidgetId, Position>,
    pub widget_in_box: HashMap<WidgetId, (WidgetId, usize)>,
    pub training_robot_id: Option<WidgetId>,
    /// Workspace notes/documentation content.
    pub text_pane_content: String,
    /// Text pane size (width, height).
    pub text_pane_size: (f64, f64),
    /// Text pane position.
    pub text_pane_position: Position,
    /// Expected patterns for drop zones (dropzone_id -> pattern).
    pub dropzone_patterns: HashMap<WidgetId, WidgetData>,
    /// Demo steps for "Show Me" animation (for tutorials).
    pub demo_steps: Vec<crate::workspace::DemoStep>,
    /// Map of widget names to their IDs (for semantic demo targeting).
    pub widget_names: HashMap<String, WidgetId>,
    /// Map of box names to their IDs (for semantic demo targeting).
    pub box_names: HashMap<String, WidgetId>,
    /// Map of dropzone roles to their IDs (e.g., "example" -> dropzone_id).
    pub dropzone_roles: HashMap<String, WidgetId>,
    /// ID of robot currently executing (for held widget tracking).
    pub executing_robot_id: Option<WidgetId>,
    /// Status message to display (e.g., robot execution errors).
    pub status_message: Option<String>,
    /// Inspection modal content (set by magnifier tool).
    pub inspection_modal: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        let (widgets, mut positions) = demo::init_widgets();
        let (boxes, box_positions) = demo::init_boxes();
        positions.extend(box_positions);

        Self {
            widgets,
            boxes,
            positions,
            widget_in_box: HashMap::new(),
            training_robot_id: None,
            // Initialize with tt1 content (default level)
            text_pane_content: TT1_DEFAULT_NOTES.to_string(),
            // Default size for sandbox mode (tutorials may override this)
            text_pane_size: (475.0, 500.0),
            // Position on right side of screen
            text_pane_position: Position::new(550.0, 60.0),
            dropzone_patterns: HashMap::new(),
            demo_steps: Vec::new(),
            // Empty for sandbox mode (populated from puzzle JSON)
            widget_names: HashMap::new(),
            box_names: HashMap::new(),
            dropzone_roles: HashMap::new(),
            executing_robot_id: None,
            status_message: None,
            inspection_modal: None,
        }
    }

    pub fn record_action(&mut self, action: Action) {
        if let Some(rid) = self.training_robot_id {
            if let Some(WidgetItem::Robot(r)) = self.widgets.get_mut(&rid) {
                r.record_action(action);
            }
        }
    }

    pub fn update_scales_in_box(&mut self, box_id: WidgetId) {
        let contents = match self.boxes.get(&box_id) {
            Some(b) => b.contents.clone(),
            None => return,
        };

        for (&hole, &wid) in &contents {
            if let Some(WidgetItem::Scales(s)) = self.widgets.get(&wid) {
                let updated = update_scales(s, &contents, hole, &self.widgets);
                self.widgets.insert(wid, WidgetItem::Scales(updated));
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn update_scales(
    scales: &Scales,
    contents: &HashMap<usize, WidgetId>,
    hole: usize,
    widgets: &HashMap<WidgetId, WidgetItem>,
) -> Scales {
    let mut s = scales.clone();
    if hole > 0 {
        if let Some(&lid) = contents.get(&(hole - 1)) {
            if let Some(WidgetItem::Number(n)) = widgets.get(&lid) {
                s.set_left(n.numerator());
            }
        }
    }
    if let Some(&rid) = contents.get(&(hole + 1)) {
        if let Some(WidgetItem::Number(n)) = widgets.get(&rid) {
            s.set_right(n.numerator());
        }
    }
    s
}
