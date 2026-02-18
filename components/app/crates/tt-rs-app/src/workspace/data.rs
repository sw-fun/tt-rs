//! Workspace data structures for JSON serialization.

use serde::{Deserialize, Serialize};

/// Metadata about a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    /// Workspace identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Description of the workspace.
    pub description: String,
    /// User level (tt1 or tt2).
    pub user_level: String,
    /// Whether this is a bundled workspace (read-only).
    #[serde(default)]
    pub is_bundled: bool,
    /// Creation timestamp (ISO 8601).
    #[serde(default)]
    pub created_at: Option<String>,
    /// Last modified timestamp (ISO 8601).
    #[serde(default)]
    pub modified_at: Option<String>,
}

/// A complete workspace with metadata and widgets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Workspace metadata.
    pub metadata: WorkspaceMetadata,
    /// All widgets in the workspace.
    pub widgets: Vec<WidgetData>,
    /// All boxes in the workspace.
    pub boxes: Vec<BoxData>,
    /// Workspace notes content.
    #[serde(default)]
    pub notes: String,
    /// Optional position for the notes pane.
    #[serde(default)]
    pub notes_position: Option<PositionData>,
    /// Optional size for the notes pane (width, height).
    #[serde(default)]
    pub notes_size: Option<(f64, f64)>,
    /// Demo steps for "Show Me" animation (for tutorials).
    #[serde(default)]
    pub demo_steps: Vec<DemoStep>,
}

/// Position in the workspace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionData {
    pub x: f64,
    pub y: f64,
}

impl PositionData {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Serializable widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WidgetData {
    #[serde(rename = "number")]
    Number(NumberData),
    #[serde(rename = "text")]
    Text(TextData),
    #[serde(rename = "scales")]
    Scales(ScalesData),
    #[serde(rename = "robot")]
    Robot(RobotData),
    #[serde(rename = "vacuum")]
    Vacuum(VacuumData),
    #[serde(rename = "wand")]
    Wand(WandData),
    #[serde(rename = "nest")]
    Nest(NestData),
    #[serde(rename = "bird")]
    Bird(BirdData),
    #[serde(rename = "dropzone")]
    DropZone(DropZoneData),
    #[serde(rename = "sensor")]
    Sensor(SensorData),
    /// Box as a widget (for expected patterns in drop zones).
    #[serde(rename = "box")]
    Box(BoxPatternData),
    /// ShowMe button for tutorials.
    #[serde(rename = "showme")]
    ShowMe(ShowMeButtonData),
    /// Magnifier tool for inspecting widgets.
    #[serde(rename = "magnifier")]
    Magnifier(MagnifierData),
}

/// Box pattern data (for use in expected patterns).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxPatternData {
    /// Number of holes in the box.
    pub num_holes: usize,
    /// Contents of each hole.
    #[serde(default)]
    pub contents: Vec<BoxHoleContent>,
}

/// Number widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberData {
    /// Unique name for semantic targeting in demos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Numerator of the rational number.
    pub numerator: i64,
    /// Denominator (default 1 for integers).
    #[serde(default = "default_denominator")]
    pub denominator: u64,
    /// Arithmetic operator (+, -, *, /).
    #[serde(default = "default_operator")]
    pub operator: String,
    /// Position in workspace.
    pub position: PositionData,
    /// Whether this is a copy source (infinite stack).
    #[serde(default)]
    pub is_copy_source: bool,
}

fn default_denominator() -> u64 {
    1
}

fn default_operator() -> String {
    "+".to_string()
}

/// Text widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextData {
    /// Text content.
    pub content: String,
    /// Position in workspace.
    pub position: PositionData,
}

/// Scales widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalesData {
    /// Position in workspace.
    pub position: PositionData,
    /// Left pan value (optional).
    #[serde(default)]
    pub left_value: Option<i64>,
    /// Right pan value (optional).
    #[serde(default)]
    pub right_value: Option<i64>,
}

/// Robot widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotData {
    /// Unique name for semantic targeting in demos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position in workspace.
    pub position: PositionData,
    /// Recorded actions (serialized).
    #[serde(default)]
    pub actions: Vec<ActionData>,
    /// Whether the robot is trained.
    #[serde(default)]
    pub is_trained: bool,
}

/// Serializable action data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionData {
    /// Action type (move, copy, etc.).
    pub action_type: String,
    /// Source widget description.
    #[serde(default)]
    pub source: Option<String>,
    /// Target location description.
    #[serde(default)]
    pub target: Option<String>,
}

/// Vacuum tool data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VacuumData {
    /// Position in workspace.
    pub position: PositionData,
}

/// Wand tool data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WandData {
    /// Position in workspace.
    pub position: PositionData,
}

/// Magnifier tool data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagnifierData {
    /// Unique name for semantic targeting in demos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position in workspace.
    pub position: PositionData,
}

/// Sensor widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    /// Unique name for semantic targeting in demos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position in workspace.
    pub position: PositionData,
    /// Sensor type (time or random).
    pub sensor_type: String,
    /// Whether this is a copy source.
    #[serde(default)]
    pub is_copy_source: bool,
}

/// Nest widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestData {
    /// Unique name for semantic targeting in demos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position in workspace.
    pub position: PositionData,
    /// Whether this is a copy source.
    #[serde(default)]
    pub is_copy_source: bool,
    /// Contents queued in the nest.
    #[serde(default)]
    pub contents: Vec<WidgetData>,
}

/// Bird widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdData {
    /// Unique name for semantic targeting in demos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position in workspace.
    pub position: PositionData,
    /// Whether this is a copy source.
    #[serde(default)]
    pub is_copy_source: bool,
    /// Paired nest index (if paired).
    #[serde(default)]
    pub paired_nest_index: Option<usize>,
    /// Paired nest name (alternative to index, for puzzle files).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paired_nest_name: Option<String>,
}

/// Box widget data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxData {
    /// Unique name for semantic targeting in demos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of holes in the box.
    pub num_holes: usize,
    /// Position in workspace.
    pub position: PositionData,
    /// Contents of each hole (hole index -> widget data).
    #[serde(default)]
    pub contents: Vec<BoxHoleContent>,
    /// Whether the box is erased (pattern).
    #[serde(default)]
    pub erased: bool,
    /// Whether this is a copy source.
    #[serde(default)]
    pub is_copy_source: bool,
}

/// Content of a box hole.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxHoleContent {
    /// Hole index (0-based).
    pub hole: usize,
    /// Widget in this hole.
    pub widget: WidgetData,
}

/// DropZone widget data for puzzles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropZoneData {
    /// Label/instruction displayed.
    pub label: String,
    /// Position in workspace.
    pub position: PositionData,
    /// Role name for semantic targeting (e.g., "example", "practice").
    #[serde(default)]
    pub role: Option<String>,
    /// Expected pattern to match against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<Box<WidgetData>>,
    /// URL to navigate to on success (optional).
    #[serde(default)]
    pub on_success_url: Option<String>,
    /// Message to show on success (optional).
    #[serde(default)]
    pub on_success_message: Option<String>,
}

/// ShowMe button widget data for tutorials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowMeButtonData {
    /// Position in workspace.
    pub position: PositionData,
    /// Demo steps to animate when clicked.
    #[serde(default)]
    pub demo_steps: Vec<DemoStep>,
}

/// Semantic target reference for demo animations.
/// Resolved at runtime to get actual widget center coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DemoTarget {
    /// Target a widget by its unique name.
    #[serde(rename = "widget")]
    Widget { name: String },
    /// Target a box by its unique name.
    #[serde(rename = "box")]
    Box { name: String },
    /// Target a specific hole in a named box.
    #[serde(rename = "box_hole")]
    BoxHole { name: String, hole: usize },
    /// Target a dropzone by its role name.
    #[serde(rename = "dropzone")]
    DropZone { role: String },
}

/// A single step in a "Show Me" demo animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum DemoStep {
    /// Wait for a duration (milliseconds).
    #[serde(rename = "wait")]
    Wait { duration: u32 },
    /// Move cursor to a position (smooth animation) - raw coordinates.
    #[serde(rename = "move_to")]
    MoveTo { x: f64, y: f64, duration: u32 },
    /// Move cursor to a semantic target (resolved to center at runtime).
    #[serde(rename = "move_to_target")]
    MoveToTarget { target: DemoTarget, duration: u32 },
    /// Move cursor relative to current position.
    #[serde(rename = "move_relative")]
    MoveRelative { offset: PositionData, duration: u32 },
    /// Start dragging from current position.
    #[serde(rename = "drag_start")]
    DragStart,
    /// End dragging at current position.
    #[serde(rename = "drag_end")]
    DragEnd,
    /// Click at current position (mousedown + mouseup).
    #[serde(rename = "click")]
    Click,
}
