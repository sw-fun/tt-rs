//! Robot types and enums.

/// The state a robot can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RobotState {
    /// Robot is idle, waiting for input that matches its pattern.
    #[default]
    Idle,
    /// Robot is being trained - recording user actions.
    Training,
    /// Robot is executing its recorded actions.
    Working,
}

/// An action that a robot can perform.
#[derive(Debug, Clone)]
pub enum Action {
    /// Pick up a widget from a location.
    PickUp {
        /// Path to the widget (e.g., "hole:0" for first hole).
        path: String,
    },
    /// Drop the held widget at a location.
    Drop {
        /// Path to the target location.
        path: String,
    },
    /// Copy a widget (using magic wand).
    Copy {
        /// Path to the widget to copy.
        path: String,
    },
    /// Remove/erase a widget (using vacuum).
    Remove {
        /// Path to the widget to remove.
        path: String,
    },
    /// Apply arithmetic operation (drop number on number).
    ApplyArithmetic {
        /// The operator to apply (+, -, *, /)
        operator: char,
        /// The operand value (numerator for rational)
        numerator: i64,
        /// The operand value (denominator for rational)
        denominator: i64,
        /// Path to the target number.
        target_path: String,
    },
    /// Click a sensor to produce a value.
    ClickSensor {
        /// Path to the sensor (e.g., "sensor:123" or "workspace:sensor").
        path: String,
    },
}

impl Action {
    /// Returns a human-readable description of this action.
    pub fn description(&self) -> String {
        match self {
            Action::PickUp { path } => {
                let target = Self::format_path(path);
                format!("Pick up {}", target)
            }
            Action::Drop { path } => {
                let target = Self::format_path(path);
                format!("Drop on {}", target)
            }
            Action::Copy { path } => {
                let target = Self::format_path(path);
                format!("Copy {}", target)
            }
            Action::Remove { path } => {
                let target = Self::format_path(path);
                format!("Remove {}", target)
            }
            Action::ApplyArithmetic {
                operator,
                numerator,
                denominator,
                target_path,
            } => {
                let target = Self::format_path(target_path);
                let op_name = match operator {
                    '+' => "Add",
                    '-' => "Subtract",
                    '*' => "Multiply",
                    '/' => "Divide",
                    '%' => "Modulo",
                    _ => "Apply",
                };
                if *denominator == 1 {
                    format!("{} {} to {}", op_name, numerator, target)
                } else {
                    format!("{} {}/{} to {}", op_name, numerator, denominator, target)
                }
            }
            Action::ClickSensor { path } => {
                let target = Self::format_path(path);
                format!("Click {}", target)
            }
        }
    }

    /// Format a path into a human-readable form.
    fn format_path(path: &str) -> String {
        if path.starts_with("workspace:") {
            let widget_type = path.strip_prefix("workspace:").unwrap_or(path);
            format!("{} in workspace", widget_type)
        } else if path.starts_with("box:") && path.contains(":hole:") {
            // Parse "box:123:hole:0"
            let parts: Vec<&str> = path.split(':').collect();
            if parts.len() == 4 {
                format!("box hole {}", parts[3])
            } else {
                path.to_string()
            }
        } else if path.starts_with("bird:") {
            "bird".to_string()
        } else if path.starts_with("sensor:") {
            "sensor".to_string()
        } else {
            path.to_string()
        }
    }
}
