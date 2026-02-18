//! Bundled puzzle and tutorial loading.

use super::data::Workspace;

/// Bundled puzzle JSON strings (compiled into the binary).
const PUZZLE_FILL_BOX: &str = include_str!("../../assets/puzzles/puzzle-fill-box.json");
const PUZZLE_MAKE_FOUR: &str = include_str!("../../assets/puzzles/puzzle-make-four.json");
const PUZZLE_MAKE_NINE: &str = include_str!("../../assets/puzzles/puzzle-make-nine.json");

/// Bundled tutorial JSON strings (compiled into the binary).
const TUTORIAL_FILL_BOX: &str = include_str!("../../assets/puzzles/tutorial-fill-box.json");
const TUTORIAL_ADD_NUMBERS: &str = include_str!("../../assets/puzzles/tutorial-add-numbers.json");
const TUTORIAL_COPY_WIDGET: &str = include_str!("../../assets/puzzles/tutorial-copy-widget.json");
const TUTORIAL_TRAIN_ROBOT: &str = include_str!("../../assets/puzzles/tutorial-train-robot.json");
const TUTORIAL_ROBOT_SENSOR: &str = include_str!("../../assets/puzzles/tutorial-robot-sensor.json");

/// Load a bundled puzzle or tutorial by ID.
pub fn load_bundled_puzzle(id: &str) -> Option<Workspace> {
    let json = match id {
        // Puzzles
        "puzzle-fill-box" => PUZZLE_FILL_BOX,
        "puzzle-make-four" => PUZZLE_MAKE_FOUR,
        "puzzle-make-nine" => PUZZLE_MAKE_NINE,
        // Tutorials
        "tutorial-fill-box" => TUTORIAL_FILL_BOX,
        "tutorial-add-numbers" => TUTORIAL_ADD_NUMBERS,
        "tutorial-copy-widget" => TUTORIAL_COPY_WIDGET,
        "tutorial-train-robot" => TUTORIAL_TRAIN_ROBOT,
        "tutorial-robot-sensor" => TUTORIAL_ROBOT_SENSOR,
        _ => return None,
    };

    match serde_json::from_str(json) {
        Ok(workspace) => Some(workspace),
        Err(e) => {
            log::error!("Failed to parse puzzle/tutorial {}: {}", id, e);
            None
        }
    }
}
