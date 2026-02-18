//! Accessor methods for Nest.

use crate::{Nest, NestColor};
use tt_rs_core::Widget;

impl Nest {
    /// Returns the nest's color.
    pub fn color(&self) -> NestColor {
        self.color
    }

    /// Returns the number of messages in the nest.
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Returns true if the nest has messages.
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Returns true if this is a copy source (palette item).
    pub fn is_copy_source(&self) -> bool {
        self.is_copy_source
    }

    /// Peeks at the top (oldest) message without removing it.
    pub fn peek_top(&self) -> Option<&dyn Widget> {
        self.messages.front().map(|m| m.as_ref())
    }

    /// Returns an iterator over all messages (oldest first).
    pub fn iter_messages(&self) -> impl Iterator<Item = &dyn Widget> {
        self.messages.iter().map(|m| m.as_ref())
    }
}

impl NestColor {
    /// Returns the CSS color string for this nest color.
    pub fn css_color(&self) -> &'static str {
        match self {
            NestColor::Blue => "#4a90d9",
            NestColor::Red => "#d94a4a",
            NestColor::Green => "#4ad94a",
            NestColor::Yellow => "#d9d94a",
        }
    }
}
