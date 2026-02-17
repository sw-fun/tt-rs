//! Workspace serialization and deserialization.

use std::collections::HashMap;

use tt_rs_bird::Bird;
use tt_rs_drag::Position;
use tt_rs_dropzone::DropZone;
use tt_rs_nest::Nest;
use tt_rs_number::{ArithOperator, Number};
use tt_rs_robot::Robot;
use tt_rs_scales::Scales;
use tt_rs_text::Text;
use tt_rs_vacuum::Vacuum;
use tt_rs_wand::Wand;

use crate::box_state::BoxState;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

use super::data::*;

/// Convert AppState to a serializable Workspace.
pub fn to_workspace(state: &AppState, metadata: WorkspaceMetadata) -> Workspace {
    let mut widgets = Vec::new();
    let mut boxes = Vec::new();

    // Serialize widgets (skip copy sources - they're part of the palette)
    for (id, widget) in &state.widgets {
        if widget.is_copy_source() {
            continue; // Skip palette items
        }

        // Skip widgets that are in boxes (they'll be serialized with the box)
        if state.widget_in_box.contains_key(id) {
            continue;
        }

        if let Some(pos) = state.positions.get(id) {
            if let Some(data) = widget_to_data(widget, pos) {
                widgets.push(data);
            }
        }
    }

    // Serialize boxes
    for (id, box_state) in &state.boxes {
        if let Some(pos) = state.positions.get(id) {
            let data = box_to_data(box_state, pos, state);
            boxes.push(data);
        }
    }

    Workspace {
        metadata,
        widgets,
        boxes,
        notes: state.text_pane_content.clone(),
        notes_position: Some(PositionData::new(
            state.text_pane_position.x,
            state.text_pane_position.y,
        )),
        notes_size: Some(state.text_pane_size),
        demo_steps: state.demo_steps.clone(),
    }
}

/// Extract name from widget data if present.
fn get_widget_name(data: &WidgetData) -> Option<String> {
    match data {
        WidgetData::Number(n) => n.name.clone(),
        WidgetData::DropZone(dz) => dz.role.clone(),
        // Other widget types don't have names yet (add as needed)
        _ => None,
    }
}

/// Convert a Workspace to AppState.
pub fn from_workspace(workspace: &Workspace) -> AppState {
    let mut widgets = HashMap::new();
    let mut positions = HashMap::new();
    let mut boxes = HashMap::new();
    let mut widget_in_box = HashMap::new();
    let mut dropzone_patterns = HashMap::new();
    let mut widget_names = HashMap::new();
    let mut box_names = HashMap::new();
    let mut dropzone_roles = HashMap::new();

    // Deserialize standalone widgets
    for widget_data in &workspace.widgets {
        // Extract name for semantic targeting
        let name = get_widget_name(widget_data);

        // Extract dropzone expected patterns and roles before converting
        if let WidgetData::DropZone(dz_data) = widget_data {
            if let Some((item, pos)) = data_to_widget(widget_data) {
                let id = item.id();
                positions.insert(id, pos);
                widgets.insert(id, item);

                // Register dropzone role for semantic targeting
                if let Some(ref role) = dz_data.role {
                    dropzone_roles.insert(role.clone(), id);
                }

                // Store expected pattern if present
                if let Some(ref expected) = dz_data.expected {
                    dropzone_patterns.insert(id, expected.as_ref().clone());
                }
            }
            continue;
        }

        if let Some((item, pos)) = data_to_widget(widget_data) {
            let id = item.id();
            positions.insert(id, pos);
            widgets.insert(id, item);

            // Register widget name for semantic targeting
            if let Some(n) = name {
                widget_names.insert(n, id);
            }
        }
    }

    // Deserialize boxes and their contents
    for box_data in &workspace.boxes {
        let (box_state, box_pos, contents) = data_to_box(box_data);
        let box_id = box_state.id();
        positions.insert(box_id, box_pos);

        // Register box name for semantic targeting
        if let Some(ref name) = box_data.name {
            box_names.insert(name.clone(), box_id);
        }

        // Add box contents
        for (hole, item) in contents {
            let item_id = item.id();
            widgets.insert(item_id, item);
            widget_in_box.insert(item_id, (box_id, hole));
        }

        boxes.insert(box_id, box_state);
    }

    // Use custom notes position if provided, otherwise default to right side
    let text_pane_position = workspace
        .notes_position
        .as_ref()
        .map(|p| Position::new(p.x, p.y))
        .unwrap_or_else(|| Position::new(550.0, 60.0));

    // Use custom notes size if provided, otherwise default
    let text_pane_size = workspace.notes_size.unwrap_or((475.0, 500.0));

    AppState {
        widgets,
        boxes,
        positions,
        widget_in_box,
        training_robot_id: None,
        text_pane_content: workspace.notes.clone(),
        text_pane_size,
        text_pane_position,
        dropzone_patterns,
        demo_steps: workspace.demo_steps.clone(),
        widget_names,
        box_names,
        dropzone_roles,
        executing_robot_id: None,
    }
}

/// Convert a WidgetItem to WidgetData.
fn widget_to_data(widget: &WidgetItem, pos: &Position) -> Option<WidgetData> {
    let position = PositionData::new(pos.x, pos.y);

    match widget {
        WidgetItem::Number(n) => Some(WidgetData::Number(NumberData {
            name: None, // Name is only loaded from puzzle files
            numerator: n.numerator(),
            denominator: n.denominator(),
            operator: operator_to_string(n.operator()),
            position,
            is_copy_source: n.is_copy_source(),
        })),
        WidgetItem::Text(t) => Some(WidgetData::Text(TextData {
            content: t.value().to_string(),
            position,
        })),
        WidgetItem::Scales(s) => Some(WidgetData::Scales(ScalesData {
            position,
            left_value: s.left_value(),
            right_value: s.right_value(),
        })),
        WidgetItem::Robot(r) => Some(WidgetData::Robot(RobotData {
            position,
            actions: actions_to_data(r),
            is_trained: !r.actions().is_empty(),
        })),
        WidgetItem::Vacuum(_) => Some(WidgetData::Vacuum(VacuumData { position })),
        WidgetItem::Wand(_) => Some(WidgetData::Wand(WandData { position })),
        WidgetItem::Nest(nest) => Some(WidgetData::Nest(NestData {
            position,
            is_copy_source: nest.is_copy_source(),
            contents: vec![], // TODO: serialize nest contents
        })),
        WidgetItem::Bird(bird) => Some(WidgetData::Bird(BirdData {
            position,
            is_copy_source: bird.is_copy_source(),
            paired_nest_index: None, // TODO: track paired nest
        })),
        WidgetItem::DropZone(dz) => Some(WidgetData::DropZone(DropZoneData {
            label: dz.label().to_string(),
            position,
            role: None,     // Role is only loaded from puzzle files
            expected: None, // Expected pattern is only loaded from puzzle files
            on_success_url: dz.on_success_url().map(|s| s.to_string()),
            on_success_message: dz.on_success_message().map(|s| s.to_string()),
        })),
    }
}

/// Convert WidgetData to WidgetItem and Position.
fn data_to_widget(data: &WidgetData) -> Option<(WidgetItem, Position)> {
    match data {
        WidgetData::Number(n) => {
            let mut num = if n.denominator == 1 {
                Number::new(n.numerator)
            } else {
                Number::rational(n.numerator, n.denominator)
            };
            num = num.with_operator(string_to_operator(&n.operator));
            if n.is_copy_source {
                num = num.as_copy_source();
            }
            Some((
                WidgetItem::Number(num),
                Position::new(n.position.x, n.position.y),
            ))
        }
        WidgetData::Text(t) => {
            let text = Text::new(&t.content);
            Some((
                WidgetItem::Text(text),
                Position::new(t.position.x, t.position.y),
            ))
        }
        WidgetData::Scales(s) => {
            let mut scales = Scales::new();
            if let Some(left) = s.left_value {
                scales.set_left(left);
            }
            if let Some(right) = s.right_value {
                scales.set_right(right);
            }
            Some((
                WidgetItem::Scales(scales),
                Position::new(s.position.x, s.position.y),
            ))
        }
        WidgetData::Robot(r) => {
            let robot = Robot::new();
            // TODO: restore actions
            Some((
                WidgetItem::Robot(robot),
                Position::new(r.position.x, r.position.y),
            ))
        }
        WidgetData::Vacuum(v) => Some((
            WidgetItem::Vacuum(Vacuum::new()),
            Position::new(v.position.x, v.position.y),
        )),
        WidgetData::Wand(w) => Some((
            WidgetItem::Wand(Wand::new()),
            Position::new(w.position.x, w.position.y),
        )),
        WidgetData::Nest(n) => {
            let mut nest = Nest::new();
            if n.is_copy_source {
                nest = nest.as_copy_source();
            }
            Some((
                WidgetItem::Nest(nest),
                Position::new(n.position.x, n.position.y),
            ))
        }
        WidgetData::Bird(b) => {
            let mut bird = Bird::new();
            if b.is_copy_source {
                bird = bird.as_copy_source();
            }
            Some((
                WidgetItem::Bird(bird),
                Position::new(b.position.x, b.position.y),
            ))
        }
        WidgetData::DropZone(dz) => {
            let mut dropzone = DropZone::new(&dz.label);
            if let Some(ref url) = dz.on_success_url {
                dropzone = dropzone.with_success_url(url);
            }
            if let Some(ref msg) = dz.on_success_message {
                dropzone = dropzone.with_success_message(msg);
            }
            Some((
                WidgetItem::DropZone(dropzone),
                Position::new(dz.position.x, dz.position.y),
            ))
        }
        WidgetData::Box(_) => {
            // Box patterns are only used inside expected patterns, not as standalone widgets
            None
        }
        WidgetData::ShowMe(_) => {
            // ShowMe buttons are UI elements in the notes pane, not standalone widgets
            None
        }
    }
}

/// Convert BoxState to BoxData.
fn box_to_data(box_state: &BoxState, pos: &Position, state: &AppState) -> BoxData {
    let position = PositionData::new(pos.x, pos.y);
    let mut contents = Vec::new();

    for hole in 0..box_state.num_holes {
        if let Some(widget_id) = box_state.widget_in_hole(hole) {
            if let Some(widget) = state.widgets.get(&widget_id) {
                // Use a dummy position since it's inside a box
                let dummy_pos = Position::new(0.0, 0.0);
                if let Some(widget_data) = widget_to_data(widget, &dummy_pos) {
                    contents.push(BoxHoleContent {
                        hole,
                        widget: widget_data,
                    });
                }
            }
        }
    }

    BoxData {
        name: None, // Name is only loaded from puzzle files
        num_holes: box_state.num_holes,
        position,
        contents,
        erased: box_state.erased,
        is_copy_source: false, // TODO: add is_copy_source to BoxState
    }
}

/// Convert BoxData to BoxState and contents.
fn data_to_box(data: &BoxData) -> (BoxState, Position, Vec<(usize, WidgetItem)>) {
    let mut box_state = if data.erased {
        BoxState::erased()
    } else {
        BoxState::new(data.num_holes)
    };

    let pos = Position::new(data.position.x, data.position.y);
    let mut contents = Vec::new();

    for hole_content in &data.contents {
        if let Some((item, _)) = data_to_widget(&hole_content.widget) {
            let item_id = item.id();
            box_state.place_in_hole(hole_content.hole, item_id);
            contents.push((hole_content.hole, item));
        }
    }

    (box_state, pos, contents)
}

fn operator_to_string(op: ArithOperator) -> String {
    match op {
        ArithOperator::Add => "+".to_string(),
        ArithOperator::Subtract => "-".to_string(),
        ArithOperator::Multiply => "*".to_string(),
        ArithOperator::Divide => "/".to_string(),
    }
}

fn string_to_operator(s: &str) -> ArithOperator {
    match s {
        "-" => ArithOperator::Subtract,
        "*" => ArithOperator::Multiply,
        "/" => ArithOperator::Divide,
        _ => ArithOperator::Add,
    }
}

fn actions_to_data(_robot: &Robot) -> Vec<ActionData> {
    // TODO: serialize robot actions
    vec![]
}
