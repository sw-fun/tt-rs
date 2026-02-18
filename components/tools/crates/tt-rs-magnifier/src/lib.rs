//! tt-rs-magnifier: Magnifier tool for inspecting widgets.
//!
//! The magnifier is a tool that inspects widgets to show their internal state.
//! Drop the magnifier on a robot to see its training state and recorded actions.
//!
//! This is a tt3-level tool for advanced users.

mod magnifier;
mod widget_impl;

pub use magnifier::Magnifier;
