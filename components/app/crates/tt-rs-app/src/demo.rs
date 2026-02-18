//! Demo scene initialization.
//!
//! Layout: Vertical columns from left to right:
//! - Col 1: Number stacks (+1, +5, -1, *2, /2)
//! - Col 2: Boxes (2-hole, 3-hole)
//! - Col 3: tt1 tools (0, Scales, Vacuum, Wand, Robot)
//! - Col 4: tt2 tools (Nest) - only visible in tt2+ mode
//! - Col 5: tt3 tools (Sensors) - only visible in tt3 mode

use std::collections::HashMap;
use tt_rs_core::WidgetId;
use tt_rs_drag::Position;
use tt_rs_magnifier::Magnifier;
use tt_rs_nest::Nest;
use tt_rs_number::{ArithOperator, Number};
use tt_rs_robot::Robot;
use tt_rs_scales::Scales;
use tt_rs_sensor::Sensor;
use tt_rs_vacuum::Vacuum;
use tt_rs_wand::Wand;

use crate::box_state::BoxState;
use crate::widget_item::WidgetItem;

// Layout constants
const START_Y: f64 = 60.0;
const ROW_SPACING: f64 = 100.0; // Increased for clear separation between icons
const COL_NUMBERS: f64 = 20.0;
const COL_BOXES: f64 = 130.0;
const COL_TT1_TOOLS: f64 = 300.0;
const COL_TT2_TOOLS: f64 = 400.0;
// tt3 sensors go in same column as tt2, below the nest
const TT3_START_ROW: usize = 1; // Start below first tt2 tool (nest)

/// Initialize demo widgets and positions.
pub fn init_widgets() -> (HashMap<WidgetId, WidgetItem>, HashMap<WidgetId, Position>) {
    let mut widgets = HashMap::new();
    let mut positions = HashMap::new();

    // Column 1: Number stacks (copy sources)
    for (i, w) in number_stacks().into_iter().enumerate() {
        let pos = Position::new(COL_NUMBERS, START_Y + (i as f64) * ROW_SPACING);
        positions.insert(w.id(), pos);
        widgets.insert(w.id(), w);
    }

    // Column 3: tt1 tools
    for (i, w) in tt1_tools().into_iter().enumerate() {
        let pos = Position::new(COL_TT1_TOOLS, START_Y + (i as f64) * ROW_SPACING);
        positions.insert(w.id(), pos);
        widgets.insert(w.id(), w);
    }

    // Column 4: tt2 tools (Nest - only visible in tt2+ mode)
    for (i, w) in tt2_tools().into_iter().enumerate() {
        let pos = Position::new(COL_TT2_TOOLS, START_Y + (i as f64) * ROW_SPACING);
        positions.insert(w.id(), pos);
        widgets.insert(w.id(), w);
    }

    // tt3 tools (Sensors) - same column as tt2 but below the nest
    for (i, w) in tt3_tools().into_iter().enumerate() {
        let row = TT3_START_ROW + i;
        let pos = Position::new(COL_TT2_TOOLS, START_Y + (row as f64) * ROW_SPACING);
        positions.insert(w.id(), pos);
        widgets.insert(w.id(), w);
    }

    (widgets, positions)
}

/// Initialize demo boxes and positions.
pub fn init_boxes() -> (HashMap<WidgetId, BoxState>, HashMap<WidgetId, Position>) {
    let mut boxes = HashMap::new();
    let mut positions = HashMap::new();

    // Column 2: Boxes
    for (i, b) in demo_boxes().into_iter().enumerate() {
        let pos = Position::new(COL_BOXES, START_Y + (i as f64) * ROW_SPACING);
        positions.insert(b.id(), pos);
        boxes.insert(b.id(), b);
    }
    (boxes, positions)
}

fn number_stacks() -> Vec<WidgetItem> {
    vec![
        WidgetItem::Number(Number::new(1).as_copy_source()),
        WidgetItem::Number(Number::new(5).as_copy_source()),
        WidgetItem::Number(arith_tool(1, ArithOperator::Subtract)),
        WidgetItem::Number(arith_tool(2, ArithOperator::Multiply)),
        WidgetItem::Number(arith_tool(2, ArithOperator::Divide)),
        WidgetItem::Number(arith_tool(10, ArithOperator::Modulo)),
    ]
}

fn tt1_tools() -> Vec<WidgetItem> {
    vec![
        WidgetItem::Number(Number::new(0)),
        WidgetItem::Scales(Scales::new()),
        WidgetItem::Vacuum(Vacuum::new()),
        WidgetItem::Wand(Wand::new()),
        WidgetItem::Robot(Robot::new()),
    ]
}

fn tt2_tools() -> Vec<WidgetItem> {
    // Note: Bird is NOT a copy source - birds are created by "hatching" (copying) a nest
    vec![WidgetItem::Nest(Nest::new().as_copy_source())]
}

fn tt3_tools() -> Vec<WidgetItem> {
    // Sensors: click to produce a number with time or random value
    // Magnifier: inspect widgets to see their internal state (e.g., robot training)
    vec![
        WidgetItem::Sensor(Sensor::new_time().as_copy_source()),
        WidgetItem::Sensor(Sensor::new_random().as_copy_source()),
        WidgetItem::Magnifier(Magnifier::new()),
    ]
}

fn arith_tool(v: i64, op: ArithOperator) -> Number {
    Number::new(v).with_operator(op).as_copy_source()
}

fn demo_boxes() -> Vec<BoxState> {
    vec![BoxState::new(2), BoxState::new(3)]
}
