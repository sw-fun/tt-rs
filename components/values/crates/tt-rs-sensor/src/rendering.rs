//! Rendering functions for Sensor.

use crate::{Sensor, SensorType};
use yew::prelude::*;

/// Renders a Sensor as HTML.
pub fn render(s: &Sensor) -> Html {
    let widget_id = s.id.to_string();
    let is_copy_source = s.is_copy_source();

    let class = if is_copy_source {
        "widget sensor copy-source"
    } else {
        "widget sensor"
    };

    // Use specific image for each sensor type
    let (image_src, alt_text) = match s.sensor_type() {
        SensorType::EpochMillis => ("images/tt-clock.svg", "time sensor"),
        SensorType::Random => ("images/tt-die.svg", "random sensor"),
    };

    html! {
        <div class={class} data-widget-id={widget_id} data-copy-source={is_copy_source.to_string()}>
            <img src={image_src} alt={alt_text} class="sensor-image" draggable="false" />
        </div>
    }
}
