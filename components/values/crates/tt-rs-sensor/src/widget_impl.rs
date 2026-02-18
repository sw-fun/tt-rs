//! Widget trait implementation for Sensor.

use crate::rendering;
use crate::sensor::{Sensor, SensorType};
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::Html;

impl Widget for Sensor {
    fn type_name(&self) -> &'static str {
        "sensor"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_sensor())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() != "sensor" {
            return MatchResult::NoMatch;
        }
        // Match if same sensor type
        if other.description() == self.description() {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        rendering::render(self)
    }

    fn description(&self) -> String {
        match self.sensor_type {
            SensorType::EpochMillis => "sensor time".to_string(),
            SensorType::Random => "sensor random".to_string(),
        }
    }
}
