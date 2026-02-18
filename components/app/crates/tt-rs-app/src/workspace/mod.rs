//! Workspace serialization for save/load functionality.
//!
//! Defines JSON-serializable data structures for workspaces.

mod data;
mod puzzles;
mod serialize;
#[cfg(test)]
mod tests;

pub use data::{
    BoxData, BoxPatternData, DemoStep, DemoTarget, DropZoneData, NumberData, PositionData,
    RobotData, ScalesData, TextData, WidgetData, Workspace, WorkspaceMetadata,
};
pub use puzzles::load_bundled_puzzle;
pub use serialize::{from_workspace, to_workspace, workspace_user_level};
