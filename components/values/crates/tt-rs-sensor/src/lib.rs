//! tt-rs-sensor: Sensor widget for time and random number generation.
//!
//! # Module Organization
//!
//! - [`Sensor`] - struct and constructors
//! - [`SensorType`] - time or random mode
//! - `widget_impl` - Widget trait implementation
//! - `rendering` - HTML rendering

mod rendering;
mod sensor;
mod widget_impl;

pub use sensor::{Sensor, SensorType};
