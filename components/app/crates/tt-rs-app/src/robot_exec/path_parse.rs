//! Path parsing utilities for robot actions.

use tt_rs_core::WidgetId;

/// Parse "widget:123" -> WidgetId.
pub fn parse_widget_path(path: &str) -> Option<WidgetId> {
    let parts: Vec<&str> = path.split(':').collect();
    if parts.len() == 2 && parts[0] == "widget" {
        parts[1].parse::<u64>().ok().map(WidgetId::from_u64)
    } else {
        None
    }
}

/// Parse "box:123:hole:0" -> (WidgetId, usize).
pub fn parse_box_hole_path(path: &str) -> Option<(WidgetId, usize)> {
    let parts: Vec<&str> = path.split(':').collect();
    if parts.len() == 4 && parts[0] == "box" && parts[2] == "hole" {
        let box_id = parts[1].parse::<u64>().ok().map(WidgetId::from_u64)?;
        let hole = parts[3].parse::<usize>().ok()?;
        Some((box_id, hole))
    } else {
        None
    }
}

/// Parse "workspace:number" -> widget type string.
pub fn parse_workspace_type_path(path: &str) -> Option<&str> {
    let parts: Vec<&str> = path.split(':').collect();
    if parts.len() == 2 && parts[0] == "workspace" {
        Some(parts[1])
    } else {
        None
    }
}

/// Parse "bird:123" -> WidgetId of the bird.
pub fn parse_bird_path(path: &str) -> Option<WidgetId> {
    let parts: Vec<&str> = path.split(':').collect();
    if parts.len() == 2 && parts[0] == "bird" {
        parts[1].parse::<u64>().ok().map(WidgetId::from_u64)
    } else {
        None
    }
}

/// Parse "sensor:123" -> WidgetId of the sensor.
pub fn parse_sensor_path(path: &str) -> Option<WidgetId> {
    let parts: Vec<&str> = path.split(':').collect();
    if parts.len() == 2 && parts[0] == "sensor" {
        parts[1].parse::<u64>().ok().map(WidgetId::from_u64)
    } else {
        None
    }
}
