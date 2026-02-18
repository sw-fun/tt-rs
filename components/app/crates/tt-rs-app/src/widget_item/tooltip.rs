//! Tooltip information for widget items.

use tt_rs_number::ArithOperator;
use tt_rs_sensor::SensorType;

use super::WidgetItem;

/// Tooltip information for a widget.
pub struct TooltipInfo {
    pub title: &'static str,
    pub description: &'static str,
    pub hint: &'static str,
}

const TOOLTIP_NUMBER_ADD: TooltipInfo = TooltipInfo {
    title: "Number Source",
    description: "Click to create a copy of this number.",
    hint: "Drag copies onto other numbers to add them.",
};

const TOOLTIP_NUMBER_SUB: TooltipInfo = TooltipInfo {
    title: "Subtraction Tool",
    description: "Click to create a subtraction operation.",
    hint: "Drag onto a number to subtract this value.",
};

const TOOLTIP_NUMBER_MUL: TooltipInfo = TooltipInfo {
    title: "Multiplication Tool",
    description: "Click to create a multiplication operation.",
    hint: "Drag onto a number to multiply by this value.",
};

const TOOLTIP_NUMBER_DIV: TooltipInfo = TooltipInfo {
    title: "Division Tool",
    description: "Click to create a division operation.",
    hint: "Drag onto a number to divide by this value.",
};

const TOOLTIP_NUMBER_MOD: TooltipInfo = TooltipInfo {
    title: "Modulo Tool",
    description: "Click to create a modulo operation.",
    hint: "Drag onto a number to get the remainder when divided by this value.",
};

const TOOLTIP_NUMBER: TooltipInfo = TooltipInfo {
    title: "Number",
    description: "A numeric value you can manipulate.",
    hint: "Drop arithmetic tools on this to change its value.",
};

const TOOLTIP_TEXT: TooltipInfo = TooltipInfo {
    title: "Text",
    description: "A text string.",
    hint: "Drag into box holes to store.",
};

const TOOLTIP_SCALES: TooltipInfo = TooltipInfo {
    title: "Scales",
    description: "Compare two numbers by dropping them on the pans.",
    hint: "The scales tip toward the larger number.",
};

const TOOLTIP_VACUUM: TooltipInfo = TooltipInfo {
    title: "Vacuum",
    description: "Erases items it touches.",
    hint: "Drop on box holes to erase contents, or on numbers to delete them.",
};

const TOOLTIP_WAND: TooltipInfo = TooltipInfo {
    title: "Magic Wand",
    description: "Creates copies of items it touches.",
    hint: "Drop on any widget to create a duplicate.",
};

const TOOLTIP_MAGNIFIER: TooltipInfo = TooltipInfo {
    title: "Magnifier",
    description: "Inspects widgets to show their internal state.",
    hint: "Drop on a robot to see its training steps.",
};

const TOOLTIP_ROBOT: TooltipInfo = TooltipInfo {
    title: "Robot",
    description: "Learns by watching your actions and can repeat them.",
    hint: "Click to start/stop training, click again to run.",
};

const TOOLTIP_NEST: TooltipInfo = TooltipInfo {
    title: "Nest",
    description: "Receives messages from birds.",
    hint: "Birds deliver items here. Click to take the oldest message.",
};

const TOOLTIP_BIRD: TooltipInfo = TooltipInfo {
    title: "Bird",
    description: "Delivers messages to its home nest.",
    hint: "Drop an item on a bird to send it to the nest.",
};

const TOOLTIP_DROPZONE: TooltipInfo = TooltipInfo {
    title: "Drop Zone",
    description: "Drop the correct answer here to verify.",
    hint: "Create the requested item and drop it here.",
};

const TOOLTIP_SENSOR_TIME: TooltipInfo = TooltipInfo {
    title: "Time Sensor",
    description: "Produces the current time as a number.",
    hint: "Click to generate a number with the current epoch milliseconds.",
};

const TOOLTIP_SENSOR_RANDOM: TooltipInfo = TooltipInfo {
    title: "Random Sensor",
    description: "Produces a random number.",
    hint: "Click to generate a random number (0-999999).",
};

/// Get tooltip information for a widget item.
pub fn tooltip_info(item: &WidgetItem) -> &'static TooltipInfo {
    match item {
        WidgetItem::Number(n) if n.is_copy_source() => match n.operator() {
            ArithOperator::Add => &TOOLTIP_NUMBER_ADD,
            ArithOperator::Subtract => &TOOLTIP_NUMBER_SUB,
            ArithOperator::Multiply => &TOOLTIP_NUMBER_MUL,
            ArithOperator::Divide => &TOOLTIP_NUMBER_DIV,
            ArithOperator::Modulo => &TOOLTIP_NUMBER_MOD,
        },
        WidgetItem::Number(_) => &TOOLTIP_NUMBER,
        WidgetItem::Text(_) => &TOOLTIP_TEXT,
        WidgetItem::Scales(_) => &TOOLTIP_SCALES,
        WidgetItem::Sensor(s) => match s.sensor_type() {
            SensorType::EpochMillis => &TOOLTIP_SENSOR_TIME,
            SensorType::Random => &TOOLTIP_SENSOR_RANDOM,
        },
        WidgetItem::Vacuum(_) => &TOOLTIP_VACUUM,
        WidgetItem::Wand(_) => &TOOLTIP_WAND,
        WidgetItem::Magnifier(_) => &TOOLTIP_MAGNIFIER,
        WidgetItem::Robot(_) => &TOOLTIP_ROBOT,
        WidgetItem::Nest(_) => &TOOLTIP_NEST,
        WidgetItem::Bird(_) => &TOOLTIP_BIRD,
        WidgetItem::DropZone(_) => &TOOLTIP_DROPZONE,
    }
}
