//! Sensor struct and constructors.

use tt_rs_core::WidgetId;
use tt_rs_number::Number;

/// The type of value the sensor produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SensorType {
    /// Produces current epoch time in milliseconds.
    #[default]
    EpochMillis,
    /// Produces a random integer (0-999999).
    Random,
}

impl SensorType {
    /// Returns the display label for this sensor type.
    pub fn label(&self) -> &'static str {
        match self {
            SensorType::EpochMillis => "time",
            SensorType::Random => "random",
        }
    }
}

/// A sensor widget that produces Number widgets when activated.
#[derive(Debug, Clone)]
pub struct Sensor {
    pub(crate) id: WidgetId,
    pub(crate) sensor_type: SensorType,
    pub(crate) is_copy_source: bool,
}

impl Sensor {
    /// Creates a new time sensor.
    pub fn new_time() -> Self {
        Self {
            id: WidgetId::new(),
            sensor_type: SensorType::EpochMillis,
            is_copy_source: false,
        }
    }

    /// Creates a new random sensor.
    pub fn new_random() -> Self {
        Self {
            id: WidgetId::new(),
            sensor_type: SensorType::Random,
            is_copy_source: false,
        }
    }

    /// Returns the sensor type.
    pub fn sensor_type(&self) -> SensorType {
        self.sensor_type
    }

    /// Returns true if this sensor is a copy source.
    pub fn is_copy_source(&self) -> bool {
        self.is_copy_source
    }

    /// Sets this sensor as a copy source (builder pattern).
    pub fn as_copy_source(mut self) -> Self {
        self.is_copy_source = true;
        self
    }

    /// Creates a copy with a new ID.
    pub fn copy_sensor(&self) -> Sensor {
        Sensor {
            id: WidgetId::new(),
            sensor_type: self.sensor_type,
            is_copy_source: false,
        }
    }

    /// Produces a Number widget based on the sensor type.
    pub fn produce_number(&self) -> Number {
        match self.sensor_type {
            SensorType::EpochMillis => {
                let millis = js_sys::Date::now() as i64;
                Number::new(millis)
            }
            SensorType::Random => {
                let random = (js_sys::Math::random() * 1_000_000.0) as i64;
                Number::new(random)
            }
        }
    }
}
