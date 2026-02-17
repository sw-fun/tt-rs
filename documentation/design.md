# tt-rs Design Document

**Full Name**: Cartoon-oriented Talking Programming Application

## Overview

This document describes the key design decisions for tt-rs, a modern Rust/WebAssembly reimplementation of ToonTalk.

**Note**: This is a derived work based on ToonTalk. See the COPYRIGHT file for attribution.

## Design Principles

### 1. Faithful to the Original Vision

The genius of the original system lies in its metaphor-based approach to programming concepts. Every design decision should preserve or enhance these metaphors:

- Robots train by watching demonstrations
- Birds carry messages to nests
- Boxes hold things in compartments
- Scales compare by tipping
- Erasure creates generalized patterns

### 2. Modern Technology, Timeless Concepts

While the concepts remain unchanged, the implementation uses modern tools:

| Original | ToonTalk Reborn | tt-rs |
|----------|-----------------|-------|
| C++ | JavaScript/jQuery | Rust/WASM |
| Windows GDI | CSS/Canvas | Three.js/SVG/CSS |
| File system | localStorage/JSON | IndexedDB/JSON |

### 3. Type Safety Through Rust

Rust's type system helps prevent entire classes of bugs:

- No null pointer exceptions (use `Option<T>`)
- No data races (ownership system)
- Clear interfaces through traits
- Compile-time guarantees

### 4. Minimal JavaScript

JavaScript is used only where absolutely necessary:

- Browser API bindings (via wasm-bindgen)
- Three.js interop
- DOM manipulation where Yew is insufficient

All business logic, state management, and game logic is in Rust.

### 5. Speech-Ready Architecture

The "talking" aspect of the application name suggests potential for:

- Pre-generated TTS audio assets for common phrases
- Dynamic TTS integration during runtime
- Audio feedback for user interactions

## Core Data Model

### Widget Hierarchy

```
Widget (trait)
+-- Number
|   +-- { numerator: BigInt, denominator: BigInt, operator: ArithOp }
+-- Box
|   +-- { holes: Vec<Option<WidgetRef>>, orientation: Orientation }
+-- Robot
|   +-- { conditions: Conditions, body: Actions, state: RobotState }
+-- Bird
|   +-- { nest: NestRef, color: Color }
+-- Nest
|   +-- { contents: VecDeque<WidgetRef>, waiting: Vec<RobotRef> }
+-- Scale
|   +-- { left: Option<WidgetRef>, right: Option<WidgetRef> }
+-- Sensor
|   +-- { event_type: EventType, attribute: String }
+-- Element
|   +-- { content: DomContent, attributes: HashMap<String, String> }
+-- Tool
    +-- Wand
    +-- Vacuum
```

### Widget Identity

Each widget has a unique identity:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
```

### Widget References

Widgets are managed through reference-counted pointers:

```rust
pub type WidgetRef = Rc<RefCell<dyn Widget>>;

// For cross-thread scenarios (future)
pub type SharedWidgetRef = Arc<RwLock<dyn Widget + Send + Sync>>;
```

### Frontside and Backside

Every widget has two faces:

```rust
pub struct WidgetView {
    frontside: FrontsideView,  // Visual representation
    backside: BacksideView,    // Configuration interface
    is_flipped: bool,
}

pub struct FrontsideView {
    position: Point,
    dimensions: Size,
    visible: bool,
    // Widget-specific rendering data
}

pub struct BacksideView {
    widgets: Vec<WidgetRef>,  // Widgets on the backside
    configuration: WidgetConfig,
}
```

## Robot Programming Model

### Robot Structure

```rust
pub struct Robot {
    id: RobotId,
    name: Option<String>,

    // Conditions for when robot can run
    frontside_condition: Option<WidgetRef>,  // Pattern to match
    backside_conditions: Vec<WidgetRef>,     // Required backside widgets

    // What the robot does
    body: ActionSequence,

    // Behavior
    run_once: bool,           // Stop after one successful run
    watched_speed: f32,       // Animation speed when watched

    // Chaining
    next_robot: Option<Box<Robot>>,  // Run this robot next

    // Runtime state
    state: RobotState,
}

pub enum RobotState {
    Idle,
    Training,
    Waiting,        // Waiting for conditions to match
    Running(usize), // Currently at step N
    Finished,
}
```

### Pattern Matching

Robots use pattern matching to decide when to run:

```rust
pub enum MatchResult {
    Match(Bindings),           // Pattern matches, with bindings
    NoMatch,                   // Pattern doesn't match
    Partial(Vec<Requirement>), // Could match if requirements met
}

pub struct Bindings {
    widget_bindings: HashMap<WidgetPath, WidgetRef>,
    value_bindings: HashMap<String, Value>,
}
```

### Erasure for Generalization

The vacuum tool creates "erased" patterns that match more broadly:

```rust
pub struct Number {
    value: Rational,
    erased: ErasureLevel,
}

pub enum ErasureLevel {
    Specific,      // Matches only this exact value
    Sign,          // Matches any number with same sign
    Type,          // Matches any number
}

impl Number {
    fn matches(&self, other: &Number) -> MatchResult {
        match self.erased {
            ErasureLevel::Specific => {
                if self.value == other.value {
                    MatchResult::Match(Bindings::empty())
                } else {
                    MatchResult::NoMatch
                }
            }
            ErasureLevel::Sign => {
                if self.value.signum() == other.value.signum() {
                    MatchResult::Match(Bindings::with("value", other.value))
                } else {
                    MatchResult::NoMatch
                }
            }
            ErasureLevel::Type => {
                MatchResult::Match(Bindings::with("value", other.value))
            }
        }
    }
}
```

### Action Recording and Playback

During training, actions are recorded as a sequence:

```rust
pub struct ActionSequence {
    steps: Vec<RecordedAction>,
}

pub struct RecordedAction {
    action: Action,
    path: WidgetPath,
    timing: Duration,  // Time since last action
}

pub enum Action {
    PickUp,
    Drop { target: WidgetPath },
    Copy,
    Remove,
    Edit(EditAction),
    GiveToBird { bird: WidgetPath },
    // ... more actions
}
```

### Widget Path Resolution

Paths describe how to find a widget relative to the context:

```rust
pub enum WidgetPath {
    // Direct references
    Context,                    // The widget being processed
    Backside(Box<WidgetPath>),  // The backside of a widget

    // Box navigation
    Hole(usize, Box<WidgetPath>),  // Hole N of a box

    // Pattern-matched references
    MatchedWidget(String),      // Widget matched by name

    // Type-based lookup
    WidgetOfType(String),       // First widget of type on backside

    // Nest contents
    NestTop(Box<WidgetPath>),   // Top item in a nest
}
```

## Message Passing (Bird/Nest)

### Actor-Like Communication

Birds and nests implement a simple actor model:

```rust
impl Bird {
    pub fn give(&self, message: WidgetRef) -> DeliveryFuture {
        if let Some(nest) = &self.nest {
            self.fly_to_nest(message, nest)
        } else {
            // Bird without nest - message is lost
            DeliveryFuture::immediate(DeliveryResult::Lost)
        }
    }

    async fn fly_to_nest(&self, message: WidgetRef, nest: &NestRef) {
        // Animate flight
        self.animate_flight(nest.position()).await;

        // Deliver message
        nest.borrow_mut().receive(message);

        // Return home
        self.animate_return().await;
    }
}

impl Nest {
    pub fn receive(&mut self, message: WidgetRef) {
        self.contents.push_back(message);

        // Wake up any waiting robots
        for robot in self.waiting.drain(..) {
            robot.borrow_mut().notify_nest_ready();
        }
    }

    pub fn take(&mut self) -> Option<WidgetRef> {
        self.contents.pop_front()
    }
}
```

### Concurrent Execution

Multiple birds can be in flight simultaneously:

```rust
pub struct DeliveryManager {
    active_deliveries: Vec<DeliveryFuture>,
}

impl DeliveryManager {
    pub async fn tick(&mut self) {
        // Progress all active deliveries
        let mut completed = vec![];
        for (i, delivery) in self.active_deliveries.iter_mut().enumerate() {
            if delivery.poll().is_ready() {
                completed.push(i);
            }
        }
        // Remove completed deliveries
        for i in completed.into_iter().rev() {
            self.active_deliveries.remove(i);
        }
    }
}
```

## Visual Design

### Modern Widget Appearance

#### Numbers

Replace the old Lego-like numbers with clean, modern typography:

```rust
pub fn render_number(num: &Number, ctx: &RenderContext) -> Html {
    let display = num.to_display_string();
    let class = format!("widget number {}", num.operator_class());

    html! {
        <div class={class}>
            <svg viewBox="0 0 120 80">
                // Rounded rectangle background with gradient
                <defs>
                    <linearGradient id="num-gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                        <stop offset="0%" style="stop-color:#f8f8f8" />
                        <stop offset="100%" style="stop-color:#e8e8e8" />
                    </linearGradient>
                </defs>
                <rect rx="8" ry="8" fill="url(#num-gradient)"
                      stroke="#ccc" stroke-width="2" />

                // Operator indicator
                <text class="operator" x="10" y="20">{num.operator_symbol()}</text>

                // Value with nice font
                <text class="value" x="60" y="50" text-anchor="middle">
                    {display}
                </text>
            </svg>
        </div>
    }
}
```

#### Boxes

3D-looking containers with clear compartments:

```rust
pub fn render_box(box_widget: &BoxWidget, ctx: &RenderContext) -> Html {
    html! {
        <div class="widget box-widget">
            <svg viewBox="0 0 200 60">
                // 3D effect with shadow
                <filter id="box-shadow">
                    <feDropShadow dx="2" dy="2" stdDeviation="2" flood-opacity="0.3"/>
                </filter>

                // Box frame
                <rect class="box-frame" filter="url(#box-shadow)" />

                // Compartment dividers
                {for (0..box_widget.size()).map(|i| {
                    html! { <line class="divider" x1={hole_x(i)} y1="0" x2={hole_x(i)} y2="60" /> }
                })}

                // Hole contents
                {for box_widget.holes().iter().enumerate().map(|(i, hole)| {
                    render_hole(i, hole, ctx)
                })}
            </svg>
        </div>
    }
}
```

#### Birds

Animated birds with personality:

```rust
pub fn render_bird(bird: &Bird, ctx: &RenderContext) -> Html {
    let animation_class = match bird.state() {
        BirdState::Idle => "bird-idle",
        BirdState::Carrying(_) => "bird-carrying",
        BirdState::Flying => "bird-flying",
    };

    html! {
        <div class={format!("widget bird {}", animation_class)}>
            // Using CSS animation sprites or Three.js for 3D bird
            <div class="bird-sprite" style={format!("--bird-color: {}", bird.color.to_css())} />
        </div>
    }
}
```

### Animation System

#### CSS-Based Animations

For simple transitions:

```css
.widget {
    transition: transform 0.2s ease-out, opacity 0.2s;
}

.widget.dragging {
    transform: scale(1.05);
    opacity: 0.9;
    filter: drop-shadow(0 4px 8px rgba(0,0,0,0.3));
}

.bird-flying {
    animation: bird-flap 0.2s infinite;
}

@keyframes bird-flap {
    0%, 100% { transform: translateY(0) rotate(0deg); }
    50% { transform: translateY(-5px) rotate(-5deg); }
}
```

#### Programmatic Animations

For complex, data-driven animations:

```rust
pub struct AnimationController {
    active_animations: Vec<Box<dyn Animation>>,
}

pub trait Animation {
    fn update(&mut self, delta: Duration) -> AnimationState;
    fn apply(&self, target: &mut dyn Widget);
}

pub struct BirdFlightAnimation {
    bird: WidgetRef,
    from: Point,
    to: Point,
    progress: f32,
    duration: Duration,
}

impl Animation for BirdFlightAnimation {
    fn update(&mut self, delta: Duration) -> AnimationState {
        self.progress += delta.as_secs_f32() / self.duration.as_secs_f32();
        if self.progress >= 1.0 {
            AnimationState::Complete
        } else {
            AnimationState::Running
        }
    }

    fn apply(&self, target: &mut dyn Widget) {
        let t = ease_in_out(self.progress);
        let pos = self.from.lerp(self.to, t);

        // Add arc for natural flight path
        let arc_height = (self.from.distance(self.to) * 0.2) * (t * (1.0 - t) * 4.0);
        let pos = Point::new(pos.x, pos.y - arc_height);

        target.set_position(pos);
    }
}
```

### Three.js Integration

For 3D effects:

```rust
#[wasm_bindgen]
extern "C" {
    pub type ThreeScene;
    pub type ThreeMesh;

    #[wasm_bindgen(constructor, js_namespace = THREE)]
    pub fn new() -> ThreeScene;

    #[wasm_bindgen(method)]
    pub fn add(this: &ThreeScene, mesh: &ThreeMesh);
}

pub struct ThreeRenderer {
    scene: ThreeScene,
    camera: ThreeCamera,
    renderer: ThreeRenderer,
}

impl ThreeRenderer {
    pub fn render_3d_widget(&self, widget: &dyn Widget) -> ThreeMesh {
        match widget.get_type_name() {
            "number" => self.create_number_mesh(widget),
            "box" => self.create_box_mesh(widget),
            _ => self.create_default_mesh(widget),
        }
    }
}
```

## State Management

### Application State

```rust
pub struct AppState {
    workspace: Workspace,
    selection: Selection,
    tool: Option<Tool>,
    dragging: Option<DragState>,
    robot_executions: Vec<RobotExecution>,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
}

pub enum Selection {
    None,
    Single(WidgetId),
    Multiple(HashSet<WidgetId>),
}

pub struct DragState {
    widget: WidgetRef,
    offset: Point,
    original_position: Point,
}
```

### Command Pattern for Undo

```rust
pub trait Command {
    fn execute(&self, state: &mut AppState) -> Result<(), Error>;
    fn undo(&self, state: &mut AppState) -> Result<(), Error>;
    fn description(&self) -> &str;
}

pub struct MoveWidgetCommand {
    widget_id: WidgetId,
    from: Point,
    to: Point,
}

impl Command for MoveWidgetCommand {
    fn execute(&self, state: &mut AppState) -> Result<(), Error> {
        state.workspace.move_widget(self.widget_id, self.to)
    }

    fn undo(&self, state: &mut AppState) -> Result<(), Error> {
        state.workspace.move_widget(self.widget_id, self.from)
    }

    fn description(&self) -> &str {
        "Move widget"
    }
}
```

## Serialization

### JSON Format

Compatible with ToonTalk Reborn where possible:

```rust
#[derive(Serialize, Deserialize)]
pub struct WidgetJson {
    pub semantic: SemanticData,
    pub view: ViewData,
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SemanticData {
    #[serde(rename = "number")]
    Number {
        numerator: String,
        denominator: String,
        operator: String,
    },
    #[serde(rename = "box")]
    Box {
        size: usize,
        contents: Vec<Option<Box<WidgetJson>>>,
        horizontal: bool,
    },
    #[serde(rename = "robot")]
    Robot {
        body: BodyJson,
        conditions: Option<Box<WidgetJson>>,
        run_once: bool,
    },
    // ... more types
}
```

## Workspace Persistence System

### Workspace Data Model

A workspace captures the complete state of the application at a point in time:

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Workspace {
    /// Metadata for display in the workspace list
    pub metadata: WorkspaceMetadata,
    /// All widgets in the workspace (excluding copy sources)
    pub widgets: Vec<WidgetData>,
    /// All boxes in the workspace
    pub boxes: Vec<BoxData>,
    /// Widget positions
    pub positions: HashMap<String, PositionData>,
    /// Which widgets are in which box holes
    pub box_contents: HashMap<String, Vec<BoxHoleData>>,
    /// Schema version for forward compatibility
    pub version: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkspaceMetadata {
    /// Unique identifier (UUID)
    pub id: String,
    /// User-provided name for the workspace
    pub name: String,
    /// User-provided description (purpose, instructions, tutorial goals)
    pub description: String,
    /// User level when workspace was saved (tt1, tt2, etc.)
    pub user_level: String,
    /// Timestamp when saved
    pub created_at: String,
    /// Timestamp when last modified
    pub modified_at: String,
    /// Whether this is a bundled example/tutorial (read-only)
    pub is_bundled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PositionData {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BoxHoleData {
    pub hole_index: usize,
    pub widget_id: String,
}
```

### Widget Serialization

Each widget type implements serialization:

```rust
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WidgetData {
    Number {
        id: String,
        numerator: i64,
        denominator: u64,
        operator: String,
        is_copy_source: bool,
    },
    Text {
        id: String,
        content: String,
    },
    Scales {
        id: String,
        left_value: Option<i64>,
        right_value: Option<i64>,
    },
    Robot {
        id: String,
        actions: Vec<ActionData>,
        state: String,
    },
    Bird {
        id: String,
        paired_nest_id: Option<String>,
    },
    Nest {
        id: String,
        message_count: usize,
    },
    Vacuum { id: String },
    Wand { id: String },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BoxData {
    pub id: String,
    pub num_holes: usize,
    pub erased: bool,
}
```

### Storage Architecture

Three storage backends with a unified interface:

```rust
pub trait WorkspaceStorage {
    /// List all available workspaces
    fn list(&self) -> Result<Vec<WorkspaceMetadata>, StorageError>;
    /// Load a workspace by ID
    fn load(&self, id: &str) -> Result<Workspace, StorageError>;
    /// Save a workspace (creates or updates)
    fn save(&mut self, workspace: &Workspace) -> Result<(), StorageError>;
    /// Delete a workspace by ID
    fn delete(&mut self, id: &str) -> Result<(), StorageError>;
}
```

#### 1. Browser LocalStorage

Primary storage for user workspaces:

```rust
pub struct LocalStorageBackend {
    prefix: String,  // "tt-rs-workspace-"
}

impl LocalStorageBackend {
    const INDEX_KEY: &'static str = "tt-rs-workspace-index";

    fn workspace_key(&self, id: &str) -> String {
        format!("{}{}", self.prefix, id)
    }
}
```

**Storage format:**
- Index: `tt-rs-workspace-index` → JSON array of workspace IDs
- Workspaces: `tt-rs-workspace-{uuid}` → Full workspace JSON

#### 2. File System (Import/Export)

For sharing and backup:

```rust
pub struct FileSystemBackend;

impl FileSystemBackend {
    /// Export workspace to downloadable JSON file
    pub fn export(workspace: &Workspace) -> Result<(), StorageError> {
        // Use web_sys to trigger file download
        // Filename: "{name}-{date}.tt-rs.json"
    }

    /// Import workspace from user-selected file
    pub fn import(file: web_sys::File) -> Result<Workspace, StorageError> {
        // Read file contents, parse JSON, validate schema
    }
}
```

#### 3. Bundled Examples

Built into the application binary:

```rust
pub struct BundledExamplesBackend {
    examples: Vec<Workspace>,  // Compiled into WASM
}

// Examples defined at compile time
const BUNDLED_EXAMPLES: &[&str] = &[
    include_str!("../examples/tutorial-arithmetic.json"),
    include_str!("../examples/tutorial-robot-basics.json"),
    include_str!("../examples/tutorial-messaging.json"),
];
```

### Workspace Menu UI

A modal dialog for workspace management:

```rust
pub enum WorkspaceDialogMode {
    List,       // Default: show list of workspaces
    Save,       // Save current workspace (name + description input)
    Confirm,    // Confirm overwrite or delete
}

#[derive(Properties, Clone, PartialEq)]
pub struct WorkspaceMenuProps {
    pub is_open: bool,
    pub on_close: Callback<()>,
    pub on_save: Callback<WorkspaceMetadata>,
    pub on_load: Callback<String>,  // workspace ID
    pub on_delete: Callback<String>,
    pub on_export: Callback<String>,
    pub on_import: Callback<web_sys::File>,
    pub current_level: UserLevel,
}
```

### Menu Button Location

A "Workspace" button in the header, next to the user level selector:

```
┌─────────────────────────────────────────────────────────────────┐
│ tt-rs - Visual Programming Environment   [Workspace ▼] [tt2 ▼] │
└─────────────────────────────────────────────────────────────────┘
```

### Dialog Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ Workspaces                                              [Close] │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [Save Current Workspace]  [Import from File]               │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ── My Workspaces ───────────────────────────────────────────── │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ★ Robot Counter (tt2)                          [Load][Del] │ │
│ │   A robot that increments by 5 when clicked.               │ │
│ │   Modified: Nov 29, 2025                                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ── Examples & Tutorials ────────────────────────────────────── │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 📚 Basic Arithmetic (tt1)                           [Load] │ │
│ │   Learn to add, subtract, multiply and divide.             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 📚 Training Your First Robot (tt1)                  [Load] │ │
│ │   Step-by-step guide to teaching a robot.                  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 📚 Messaging with Birds (tt2)                       [Load] │ │
│ │   Send messages between widgets using birds and nests.     │ │
│ └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Save Dialog

```
┌─────────────────────────────────────────────────────────────────┐
│ Save Workspace                                          [Close] │
├─────────────────────────────────────────────────────────────────┤
│ Name:                                                           │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ My Robot Counter                                           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Description:                                                    │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ A robot trained to increment a number by 5 each time      │ │
│ │ it's clicked. Good for demonstrating basic robot training. │ │
│ │                                                            │ │
│ │ Instructions:                                               │ │
│ │ 1. Click the robot to run it                               │ │
│ │ 2. Watch the number increase by 5                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ User Level: tt2 (automatically set)                             │
│                                                                 │
│              [Cancel]                    [Save]                 │
└─────────────────────────────────────────────────────────────────┘
```

### ID Remapping on Load

When loading a workspace, widget IDs must be remapped to avoid conflicts:

```rust
impl Workspace {
    /// Load workspace and remap all IDs to fresh values
    pub fn load_with_fresh_ids(data: &Workspace) -> (AppState, HashMap<String, WidgetId>) {
        let mut id_map: HashMap<String, WidgetId> = HashMap::new();

        // Generate new IDs for all widgets
        for widget in &data.widgets {
            let old_id = widget.id();
            let new_id = WidgetId::new();
            id_map.insert(old_id, new_id);
        }

        // Remap box contents
        // Remap bird/nest pairings
        // Remap robot action targets

        // ...
    }
}
```

### Excluding Copy Sources

Copy sources (palette items) are NOT saved:
- They are recreated by `demo::init_widgets()` on load
- Only user-created widgets and their positions are persisted
- This keeps workspace files small and ensures palette consistency

## URL-Based Routing

### Design Goals

URL routing enables:
- **Persistence**: Browser reload stays on the same puzzle/workspace
- **Sharing**: Copy URL to share a specific puzzle with others
- **Bookmarking**: Save puzzles to browser bookmarks
- **Navigation**: Browser back/forward buttons work intuitively
- **Deep linking**: Documentation and tutorials can link directly to puzzles

### URL Structure

```
https://sw-fun.github.io/tt-rs/
https://sw-fun.github.io/tt-rs/#/tutorial/fill-a-box
https://sw-fun.github.io/tt-rs/#/tutorial/make-a-4
https://sw-fun.github.io/tt-rs/#/example/counting-robot
https://sw-fun.github.io/tt-rs/#/challenge/factorial
https://sw-fun.github.io/tt-rs/#/workspace/{uuid}
```

Using hash-based routing (`#/path`) for GitHub Pages compatibility (no server-side routing).

### Route Types

```rust
pub enum Route {
    /// Default workspace (empty or demo)
    Home,
    /// Load a bundled tutorial puzzle
    Tutorial { slug: String },
    /// Load a bundled example workspace
    Example { slug: String },
    /// Load a bundled challenge puzzle
    Challenge { slug: String },
    /// Load a user-saved workspace by ID
    Workspace { id: String },
}

impl Route {
    pub fn from_hash(hash: &str) -> Self {
        // Parse hash fragment into Route
        // e.g., "#/tutorial/fill-a-box" -> Tutorial { slug: "fill-a-box" }
    }

    pub fn to_hash(&self) -> String {
        match self {
            Route::Home => String::new(),
            Route::Tutorial { slug } => format!("#/tutorial/{}", slug),
            Route::Example { slug } => format!("#/example/{}", slug),
            Route::Challenge { slug } => format!("#/challenge/{}", slug),
            Route::Workspace { id } => format!("#/workspace/{}", id),
        }
    }
}
```

### Navigation Integration

```rust
pub struct Router {
    current_route: Route,
    history_listener: EventListener,
}

impl Router {
    /// Update URL without page reload
    pub fn navigate_to(&mut self, route: Route) {
        let hash = route.to_hash();
        window().location().set_hash(&hash).unwrap();
        self.current_route = route;
    }

    /// Listen for browser back/forward
    pub fn on_popstate<F>(&self, callback: F) where F: Fn(Route) {
        // Called when user clicks back/forward or manually changes URL
    }
}
```

### Puzzle Loading with URL Update

When user clicks a puzzle in the Workspace menu:
1. Load the puzzle data
2. Update the URL hash
3. Close the menu

```rust
pub fn load_puzzle(slug: &str, state: &mut AppState) {
    // Load puzzle
    let workspace = puzzles::load_puzzle(slug);
    *state = from_workspace(&workspace);

    // Update URL
    let route = Route::Tutorial { slug: slug.to_string() };
    Router::navigate_to(route);
}
```

### Page Reload Behavior

On page load:
1. Parse current URL hash
2. If route is a puzzle/workspace, load it
3. Otherwise, show default workspace

```rust
pub fn on_mount() {
    let hash = window().location().hash().unwrap_or_default();
    let route = Route::from_hash(&hash);

    match route {
        Route::Tutorial { slug } => load_puzzle(&slug),
        Route::Example { slug } => load_example(&slug),
        Route::Challenge { slug } => load_challenge(&slug),
        Route::Workspace { id } => load_workspace(&id),
        Route::Home => init_default_workspace(),
    }
}
```

## Puzzle/Tutorial Usability

### Reset Button

Each puzzle/tutorial needs a reset button to restart from the beginning:

```rust
pub struct PuzzleControls {
    pub on_reset: Callback<()>,
    pub on_hint: Option<Callback<()>>,
    pub on_show_me: Option<Callback<()>>,
}
```

**Reset button behavior:**
1. Reload the puzzle from its original JSON file
2. Clear any error state on the DropZone
3. Keep the same URL (don't navigate away)
4. Optionally show a "Reset" confirmation toast

**UI placement:**
- Add a floating control bar when a puzzle is loaded
- Or add controls to the DropZone widget itself

### "Show Me" Animated Demo (Future)

A demonstration system that animates the solution:

```rust
pub struct DemoAnimation {
    steps: Vec<DemoStep>,
    current_step: usize,
    speed: f32,
}

pub enum DemoStep {
    Wait { duration: Duration },
    MoveTo { widget_id: WidgetId, position: Position },
    DragStart { widget_id: WidgetId },
    DragMove { path: Vec<Position> },
    DragEnd { target: DragTarget },
    ShowTooltip { text: String, position: Position },
}
```

**Implementation approach:**
1. Each puzzle JSON can include an optional `demo` field with animation steps
2. The "Show Me" button plays the animation
3. Widget positions and drags are interpolated for smooth animation
4. Tooltips can appear to explain each step

### Undo/Redo for Puzzles

Command pattern for undoing user actions:

```rust
pub enum PuzzleCommand {
    /// Widget was placed in a box hole
    PlaceInHole { widget_id: WidgetId, box_id: WidgetId, hole: usize },
    /// Widget was removed from a box hole
    RemoveFromHole { widget_id: WidgetId, box_id: WidgetId, hole: usize },
    /// Widget position changed
    MoveWidget { widget_id: WidgetId, from: Position, to: Position },
    /// Number arithmetic was applied
    ApplyArithmetic { source: WidgetId, target: WidgetId, result: Number },
}

pub struct UndoStack {
    commands: Vec<PuzzleCommand>,
    max_size: usize,
}

impl UndoStack {
    pub fn undo(&mut self, state: &mut AppState) {
        if let Some(cmd) = self.commands.pop() {
            cmd.undo(state);
        }
    }
}
```

### Hint System (Future)

Progressive hints for stuck users:

```rust
pub struct PuzzleHints {
    hints: Vec<String>,
    revealed_count: usize,
}

impl PuzzleHints {
    pub fn reveal_next(&mut self) -> Option<&str> {
        if self.revealed_count < self.hints.len() {
            let hint = &self.hints[self.revealed_count];
            self.revealed_count += 1;
            Some(hint)
        } else {
            None
        }
    }
}
```

Example hints for "Fill a Box" puzzle:
1. "Drag the number 1 into the left hole of the box"
2. "Drag the number 2 into the right hole of the box"
3. "Now drag the filled box onto the drop zone"

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TtrsError {
    #[error("Widget not found: {0}")]
    WidgetNotFound(WidgetId),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Robot error: {0}")]
    RobotError(RobotError),
}

#[derive(Debug, thiserror::Error)]
pub enum RobotError {
    #[error("No match for conditions")]
    NoMatch,

    #[error("Path not found: {0:?}")]
    PathNotFound(WidgetPath),

    #[error("Action failed: {0}")]
    ActionFailed(String),
}
```

### User-Friendly Messages

```rust
impl TtrsError {
    pub fn user_message(&self) -> String {
        match self {
            Self::WidgetNotFound(_) => "Oops! I can't find that thing.".into(),
            Self::InvalidOperation(msg) => format!("Sorry, I can't do that: {}", msg),
            Self::RobotError(RobotError::NoMatch) =>
                "The robot doesn't know what to do with this.".into(),
            // ... more friendly messages
        }
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_addition() {
        let a = Number::new(1, 1);
        let b = Number::new(2, 1);
        let result = a.apply_to(&b).unwrap();
        assert_eq!(result.value(), Rational::new(3, 1));
    }

    #[test]
    fn pattern_matching() {
        let pattern = Number::erased(ErasureLevel::Type);
        let value = Number::new(42, 1);
        assert!(matches!(pattern.matches(&value), MatchResult::Match(_)));
    }
}
```

### Integration Tests

```rust
#[wasm_bindgen_test]
async fn robot_training_and_execution() {
    let workspace = Workspace::new();

    // Create widgets
    let num1 = workspace.add_number(1);
    let num2 = workspace.add_number(2);
    let robot = workspace.add_robot();

    // Train robot
    robot.start_training(&workspace);
    workspace.pick_up(num1);
    workspace.drop_on(num2);
    robot.stop_training();

    // Run robot with new data
    let num3 = workspace.add_number(10);
    let num4 = workspace.add_number(20);
    robot.run_on(&workspace, num3).await;

    // Verify result
    assert_eq!(num4.value(), Rational::new(30, 1));
}
```

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
