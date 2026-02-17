//! Widget item enum for app state.

mod render;
mod tooltip;

use tt_rs_bird::Bird;
use tt_rs_core::{Widget, WidgetId};
use tt_rs_dropzone::DropZone;
use tt_rs_nest::Nest;
use tt_rs_number::Number;
use tt_rs_robot::Robot;
use tt_rs_scales::Scales;
use tt_rs_text::Text;
use tt_rs_vacuum::Vacuum;
use tt_rs_wand::Wand;
use yew::prelude::*;

pub use render::render_small;
pub use tooltip::TooltipInfo;

/// A widget item with its type for rendering.
#[derive(Clone)]
#[allow(dead_code)]
pub enum WidgetItem {
    Number(Number),
    Text(Text),
    Scales(Scales),
    Vacuum(Vacuum),
    Wand(Wand),
    Robot(Robot),
    Nest(Nest),
    Bird(Bird),
    DropZone(DropZone),
}

impl WidgetItem {
    pub fn id(&self) -> WidgetId {
        match self {
            WidgetItem::Number(n) => n.id(),
            WidgetItem::Text(t) => t.id(),
            WidgetItem::Scales(s) => s.id(),
            WidgetItem::Vacuum(v) => v.id(),
            WidgetItem::Wand(w) => w.id(),
            WidgetItem::Robot(r) => r.id(),
            WidgetItem::Nest(nest) => nest.id(),
            WidgetItem::Bird(bird) => bird.id(),
            WidgetItem::DropZone(dz) => dz.id(),
        }
    }

    pub fn render(&self) -> Html {
        render::render(self)
    }

    pub fn is_vacuum(&self) -> bool {
        matches!(self, WidgetItem::Vacuum(_))
    }
    pub fn is_wand(&self) -> bool {
        matches!(self, WidgetItem::Wand(_))
    }
    pub fn is_robot(&self) -> bool {
        matches!(self, WidgetItem::Robot(_))
    }
    pub fn is_dropzone(&self) -> bool {
        matches!(self, WidgetItem::DropZone(_))
    }

    pub fn tooltip_info(&self) -> &'static TooltipInfo {
        tooltip::tooltip_info(self)
    }

    #[allow(dead_code)]
    pub fn as_robot_mut(&mut self) -> Option<&mut Robot> {
        match self {
            WidgetItem::Robot(r) => Some(r),
            _ => None,
        }
    }

    /// Returns the type name of this widget (e.g., "number", "robot").
    pub fn type_name(&self) -> &'static str {
        match self {
            WidgetItem::Number(_) => "number",
            WidgetItem::Text(_) => "text",
            WidgetItem::Scales(_) => "scales",
            WidgetItem::Vacuum(_) => "vacuum",
            WidgetItem::Wand(_) => "wand",
            WidgetItem::Robot(_) => "robot",
            WidgetItem::Nest(_) => "nest",
            WidgetItem::Bird(_) => "bird",
            WidgetItem::DropZone(_) => "dropzone",
        }
    }

    /// Returns true if this widget is a copy source (palette item).
    /// Note: Only widgets that track copy source status have the method.
    /// Others (Text, Scales, tools) are never copy sources in practice.
    pub fn is_copy_source(&self) -> bool {
        match self {
            WidgetItem::Number(n) => n.is_copy_source(),
            WidgetItem::Nest(nest) => nest.is_copy_source(),
            WidgetItem::Bird(bird) => bird.is_copy_source(),
            // These widget types don't track copy source status
            WidgetItem::Text(_)
            | WidgetItem::Scales(_)
            | WidgetItem::Vacuum(_)
            | WidgetItem::Wand(_)
            | WidgetItem::Robot(_)
            | WidgetItem::DropZone(_) => false,
        }
    }

    /// Creates a copy of this widget with a new ID.
    pub fn copy_widget(&self) -> WidgetItem {
        match self {
            WidgetItem::Number(n) => WidgetItem::Number(n.copy_number()),
            WidgetItem::Text(t) => WidgetItem::Text(t.copy_text()),
            WidgetItem::Scales(s) => WidgetItem::Scales(s.copy_scales()),
            WidgetItem::Vacuum(v) => WidgetItem::Vacuum(v.copy_vacuum()),
            WidgetItem::Wand(w) => WidgetItem::Wand(w.copy_wand()),
            WidgetItem::Robot(r) => WidgetItem::Robot(r.copy_robot()),
            WidgetItem::Nest(nest) => WidgetItem::Nest(nest.copy_nest()),
            WidgetItem::Bird(bird) => WidgetItem::Bird(bird.copy_bird()),
            WidgetItem::DropZone(dz) => WidgetItem::DropZone(dz.copy_dropzone()),
        }
    }

    /// Converts this widget to a boxed trait object for message passing.
    pub fn to_boxed_widget(&self) -> Box<dyn Widget> {
        match self {
            WidgetItem::Number(n) => Box::new(n.clone()),
            WidgetItem::Text(t) => Box::new(t.clone()),
            WidgetItem::Scales(s) => Box::new(s.clone()),
            WidgetItem::Vacuum(v) => Box::new(v.clone()),
            WidgetItem::Wand(w) => Box::new(w.clone()),
            WidgetItem::Robot(r) => Box::new(r.clone()),
            WidgetItem::Nest(nest) => Box::new(nest.clone()),
            WidgetItem::Bird(bird) => Box::new(bird.clone()),
            WidgetItem::DropZone(dz) => Box::new(dz.clone()),
        }
    }

    /// Converts a boxed trait object back to a WidgetItem.
    /// Uses description parsing to reconstruct widget state.
    pub fn from_boxed_widget(widget: Box<dyn Widget>) -> WidgetItem {
        let desc = widget.description();
        match widget.type_name() {
            "number" => {
                // Parse number from description like "number +5", "number -1", "number *2", "number /2"
                // All numbers have an operator - Add is the default for plain numbers
                use tt_rs_number::ArithOperator;
                let value_str = desc.strip_prefix("number ").unwrap_or("+0");

                // Check for operator prefix (all descriptions now have one)
                let (operator, num_str) = if let Some(rest) = value_str.strip_prefix('+') {
                    (ArithOperator::Add, rest)
                } else if let Some(rest) = value_str.strip_prefix('-') {
                    (ArithOperator::Subtract, rest)
                } else if let Some(rest) = value_str.strip_prefix('*') {
                    (ArithOperator::Multiply, rest)
                } else if let Some(rest) = value_str.strip_prefix('/') {
                    (ArithOperator::Divide, rest)
                } else {
                    // Fallback for legacy descriptions without operator
                    (ArithOperator::Add, value_str)
                };

                // Parse the numeric part (may be rational like "3/4")
                if let Some((num, denom)) = num_str.split_once('/') {
                    if let (Ok(n), Ok(d)) = (num.parse::<i64>(), denom.parse::<u64>()) {
                        return WidgetItem::Number(Number::rational(n, d).with_operator(operator));
                    }
                }
                if let Ok(n) = num_str.parse::<i64>() {
                    return WidgetItem::Number(Number::new(n).with_operator(operator));
                }
                WidgetItem::Number(Number::new(0))
            }
            "text" => {
                // Parse text from description like 'text "hello"'
                let content = desc
                    .strip_prefix("text \"")
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap_or("");
                WidgetItem::Text(Text::new(content))
            }
            "scales" => WidgetItem::Scales(Scales::new()),
            "vacuum" => WidgetItem::Vacuum(Vacuum::new()),
            "wand" => WidgetItem::Wand(Wand::new()),
            "robot" => WidgetItem::Robot(Robot::new()),
            "nest" => WidgetItem::Nest(Nest::new()),
            "bird" => WidgetItem::Bird(Bird::new()),
            "dropzone" => {
                // Parse dropzone from description like 'dropzone "I need a 4"'
                let label = desc
                    .strip_prefix("dropzone \"")
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap_or("Drop here");
                WidgetItem::DropZone(DropZone::new(label))
            }
            _ => {
                log::warn!("Unknown widget type: {}", widget.type_name());
                WidgetItem::Number(Number::new(0))
            }
        }
    }
}
