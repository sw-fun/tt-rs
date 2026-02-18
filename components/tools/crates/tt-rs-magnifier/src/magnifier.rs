//! Magnifier (inspection) tool struct.

use tt_rs_core::WidgetId;

/// A magnifier tool for inspecting widgets.
///
/// Drop the magnifier on a robot to see its training state and recorded actions.
/// This is a tt3-level tool for advanced users who want to understand robot behavior.
#[derive(Debug, Clone)]
pub struct Magnifier {
    pub(crate) id: WidgetId,
}

impl Magnifier {
    /// Creates a new magnifier tool.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
        }
    }

    /// Creates a magnifier as a copy source (for the toolbox).
    pub fn as_copy_source() -> Self {
        Self {
            id: WidgetId::new(),
        }
    }

    /// Creates a copy of this magnifier with a new ID.
    pub fn copy_magnifier(&self) -> Magnifier {
        Magnifier {
            id: WidgetId::new(),
        }
    }
}

impl Default for Magnifier {
    fn default() -> Self {
        Self::new()
    }
}
