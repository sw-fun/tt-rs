# Robot Programming MVP Design

This document specifies robot behavior requirements and designs a Rust implementation approach.

## What is a ToonTalk Robot?

A robot is an **autonomous agent that learns by watching**. The user demonstrates actions, and the robot records and replays them. The key insight: **programming by demonstration** makes computing accessible to children.

### Core Behaviors

1. **Watch Mode**: Robot observes and records user actions
2. **Work Mode**: Robot replays recorded actions on new input
3. **Pattern Matching**: Robot recognizes when conditions are right to act
4. **Generalization**: Robot can work on "any number" not just "the number 5"

---

## Current Implementation Status

### What Works
- Training mode toggle (click to start/stop)
- Recording arithmetic operations (+, -, *, /)
- Recording copy (wand) and remove (vacuum) actions
- Sequential action replay

### What's Missing
- Robot doesn't track "what it's holding" during execution
- Robot can't generalize (works only on exact trained input)
- No visual feedback during execution
- PickUp action not recorded/executed

---

## Behavior Specifications

### Behavior 1: Hold and Release

**User Story**: As a user, I want the robot to pick up a widget, carry it, and put it somewhere else.

**Demonstration**:
1. User starts training
2. User drags widget A from location X
3. User drops widget A at location Y
4. User stops training

**Expected Replay**:
1. Robot "picks up" widget at location X
2. Robot "moves" (visually) toward location Y
3. Robot "drops" widget at location Y

**Design Approach**:
```
During Training:
  - On drag start: Record PickUp { source_path }
  - On drag end: Record Drop { target_path }

During Execution:
  - PickUp: Remove widget from source, store in robot_held_widget
  - Drop: Place robot_held_widget at target
```

### Behavior 2: Pattern Matching

**User Story**: As a user, I want to train a robot once, then have it work on any similar input.

**Demonstration**:
1. Train robot with number "7"
2. Use vacuum to "erase" the 7 to "any number"
3. Give robot number "42"
4. Robot treats 42 the same way it treated 7

**Expected Behavior**:
- Robot stores a **condition** (what it was trained on)
- Before running, robot checks if current input **matches** condition
- Erased values match anything of the same type

**Design Approach**:
```
Robot stores:
  - condition: Widget (snapshot from training start)

Before execution:
  - Call condition.matches(current_input)
  - If Match: proceed with actions
  - If NoMatch: do nothing (or try next robot in team)

Erasure levels:
  - Specific: matches exact value (42 matches 42)
  - Type: matches any widget of same type (any number)
```

### Behavior 3: Value Bindings

**User Story**: As a user, I want the robot to use the actual input value in its actions, not the training value.

**Demonstration**:
1. Train robot: "pick up the number, add it to itself"
2. Trained with 5 → got 10
3. Run with 7 → should get 14, not 10

**Expected Behavior**:
- Actions reference "the input" not "widget #123"
- When replaying, resolve references to actual matched widgets

**Design Approach**:
```
Path types:
  - ContextPath: "the widget I'm working on"
  - HeldPath: "what I'm holding"
  - BoxHolePath: "hole N of the box"
  - ResourcePath: "a fresh copy of this template"

During execution:
  - Resolve paths relative to current context
  - Not by absolute widget ID
```

### Behavior 4: Watched Execution

**User Story**: As a user, I want to see what the robot is doing step-by-step.

**Expected Behavior**:
- Animated cursor shows robot's "hand"
- Current action highlighted
- Configurable speed (instant, slow, step-through)
- Optional thought bubble showing current action

**Design Approach**:
```
Execution states:
  - Idle: waiting
  - Animating: showing movement
  - Paused: waiting for user (step mode)

Each action has:
  - Start position (where to pick up)
  - End position (where to drop)
  - Animation duration
```

### Behavior 5: Conditional Execution (Scales)

**User Story**: As a user, I want the robot to do different things based on comparison.

**Demonstration**:
1. Train robot with box containing [5, 3] and scales
2. Robot puts numbers on scales, sees left tips down
3. Robot picks up left number (the bigger one)
4. Train second robot for opposite case

**Expected Behavior**:
- Scales state (tipped_left, tipped_right, balanced) becomes part of condition
- Robot only runs if scales match expected state
- Robot "teams" handle all cases

**Design Approach**:
```
Condition includes:
  - Widget patterns (erased numbers, boxes)
  - Scales state (if scales present)

Robot teams:
  - List of robots tried in order
  - First matching robot runs
  - Others ignored
```

### Behavior 6: Recursion (Bird/Nest)

**User Story**: As a user, I want to create a counting robot that counts to 10.

### Behavior 7: Robot Uses Robot (Future Goal)

**User Story**: As a user, I want one robot to use another robot as a subroutine.

**Demonstration**:
1. Robot A is trained to double a number
2. Robot B is trained to: invoke Robot A, then invoke Robot A again
3. Result: Robot B quadruples the input

**Expected Behavior**:
- Robot can "click" another robot as an action
- The other robot runs to completion
- Original robot continues with its next action

**Use Cases**:
- **Subroutines**: Break complex behaviors into reusable pieces
- **Composition**: Build sophisticated programs from simple parts
- **Teaching robots**: A "teacher" robot trains a "student" robot

**Design Approach**:
```
InvokeRobot action:
  - target_robot: WidgetPath (which robot to run)
  - wait_for_completion: bool

Execution:
  - Pause current robot
  - Run target robot on current context
  - Resume current robot when target finishes
```

### Behavior 8: Robot Trains Robot (Future Goal)

**User Story**: As a user, I want one robot to train another robot.

**Demonstration**:
1. Untrained robot B sits in workspace
2. Robot A is trained to: click robot B (start training), do actions, click robot B (stop training)
3. Running robot A results in robot B becoming trained

**Why This Matters**:
- Enables "factory" robots that produce trained robots
- Meta-programming: programs that write programs
- Self-modifying systems (with care)

**Demonstration**:
1. Robot receives number N from nest
2. Robot checks: is N < 10?
3. If yes: add 1 to N, give result to bird
4. Bird delivers to nest, robot runs again
5. If no: stop

**Expected Behavior**:
- Bird/nest enable message passing
- Robot can wait for nest to receive something
- Robot can send results via bird
- This creates loops/recursion

**Design Approach**:
```
Bird actions:
  - GiveToBird { widget_path, bird_path }

Nest integration:
  - Robot can have nest in condition
  - Robot waits until nest has content
  - Nest content becomes robot's input
```

---

## Demo Tiers

### Tier 1: Works Now (Needs Testing)
- **Add 5**: Train robot to add +5 to a number
- **Copy Widget**: Train robot to copy with wand

### Tier 2: Needs Held Widget
- **Move to Box**: Pick up number, put in box hole
- **Swap**: Exchange two values in a box

### Tier 3: Needs Pattern Matching
- **Add 5 to ANY number**: Generalized adding
- **Double any number**: Copy input, add to itself

### Tier 4: Needs Scales
- **Max of two**: Return the larger of two numbers
- **Sort pair**: Arrange two numbers in order

### Tier 5: Needs Bird/Nest
- **Count to 10**: Recursive counting
- **Factorial**: Compute n!
- **Fibonacci**: Generate sequence

### Tier 6: Needs Text Operations
- **Hello World**: Display text (simplest: pre-made text widget)

### Tier 7: Meta-Programming (Future)
- **Robot trains robot**: A robot that teaches another robot
- **Robot uses robot**: A robot that invokes another robot as a subroutine
- **Robot creates robot**: A robot that constructs new robots programmatically

---

## Implementation Plan

### Phase 1: Held Widget State

**Goal**: Robot can pick up and drop widgets

**Changes to Robot**:
```rust
pub struct Robot {
    // existing fields...
    held_widget_id: Option<WidgetId>,
}
```

**Changes to AppState**:
```rust
// During robot execution, track what robot is "holding"
executing_robot_held: Option<WidgetId>,
```

**New Action Variants**:
```rust
pub enum Action {
    PickUp {
        source: WidgetPath,  // Where to find widget
    },
    Drop {
        target: WidgetPath,  // Where to place it
    },
    // existing variants...
}
```

**Recording**:
- On mousedown during training: record PickUp
- On mouseup (drop): record Drop

**Execution**:
- PickUp: resolve path, remove from container, store ID
- Drop: resolve path, place held widget there

### Phase 2: Watched Execution

**Goal**: Visual feedback during replay

**New Component**: `RobotCursor`
```rust
pub struct RobotCursor {
    visible: bool,
    position: (f64, f64),
    held_widget: Option<WidgetId>,
    animation_target: Option<(f64, f64)>,
}
```

**Execution Loop**:
1. Set cursor visible at robot position
2. For each action:
   a. Animate cursor to source
   b. Execute PickUp (cursor now shows held widget)
   c. Animate cursor to target
   d. Execute Drop (cursor releases widget)
3. Hide cursor

**Speed Control**:
- `instant`: No animation, execute all at once
- `normal`: 500ms per action
- `slow`: 1500ms per action
- `step`: Wait for user click between actions

### Phase 3: Pattern Matching

**Goal**: Robot matches input before running

**Condition Storage**:
```rust
pub struct Robot {
    // Snapshot of what robot was trained on
    condition: Option<Box<dyn Widget>>,
}
```

**Training Changes**:
- On training_start: Clone current context as condition
- Condition is immutable after training ends

**Execution Changes**:
```rust
pub fn should_run(&self, context: &dyn Widget) -> bool {
    match &self.condition {
        None => true,  // No condition = always run
        Some(cond) => cond.matches(context) == MatchResult::Match,
    }
}
```

**Erasure**:
- Vacuum on condition widget increases erasure level
- Erased widgets match more broadly

### Phase 4: Relative Paths

**Goal**: Actions reference widgets by role, not ID

**Path Types**:
```rust
pub enum WidgetPath {
    Context,                    // The widget robot is working on
    Held,                       // What robot is holding
    BoxHole { index: usize },   // Hole N of context (if box)
    Resource { template: Box<dyn Widget> },  // Fresh copy of template
    BacksideWidget { type_name: String },    // Widget on backside
}
```

**Path Resolution**:
```rust
pub fn resolve_path(
    path: &WidgetPath,
    context: &AppState,
    robot: &Robot
) -> Option<WidgetId> {
    match path {
        WidgetPath::Context => context.robot_context,
        WidgetPath::Held => robot.held_widget_id,
        WidgetPath::BoxHole { index } => {
            // Find box, get contents of hole
        }
        // etc.
    }
}
```

### Phase 5: Robot Teams

**Goal**: Multiple robots handle different cases

**Structure**:
```rust
pub struct Robot {
    // If this robot doesn't match, try next
    next_robot: Option<WidgetId>,
}
```

**Execution**:
```rust
pub fn run_robot_team(first: &Robot, context: &dyn Widget) {
    let mut current = Some(first);
    while let Some(robot) = current {
        if robot.should_run(context) {
            robot.execute(context);
            return;
        }
        current = robot.next_robot.map(|id| get_robot(id));
    }
    // No robot matched
}
```

---

## Tutorial Design

### Tutorial 1: "Teach Your First Robot"

**Learning Goal**: Understand training mode

**Steps**:
1. "Click the robot to start training" (robot glows yellow)
2. "Drag the +5 onto the 7" (action is recorded)
3. "Click the robot to stop training" (glow stops)
4. "Now give the robot a new number"
5. "Click the robot to run" (replays the addition)

**Validation**:
- Step 1: Robot enters Training state
- Step 2: ArithmeticAction recorded
- Step 3: Robot enters Idle state with actions
- Step 5: Robot executes successfully

### Tutorial 2: "Move Things Around"

**Learning Goal**: PickUp and Drop actions

**Steps**:
1. "Start training the robot"
2. "Pick up the number and put it in the box"
3. "Stop training"
4. "Try with a different number"

### Tutorial 3: "Make It Work for Any Number"

**Learning Goal**: Pattern matching and erasure

**Steps**:
1. "You trained the robot on the number 7"
2. "But what if you want it to work on ANY number?"
3. "Use the vacuum on the 7 in the robot's thought bubble"
4. "Now the 7 becomes a blank - meaning 'any number'"
5. "Give the robot a different number and watch it work"

---

## Success Criteria

### MVP Complete When:
- [ ] User can train robot to pick up and drop widgets
- [ ] Robot shows visual feedback during execution
- [ ] User can load pre-trained robot from tutorial
- [ ] One complete tutorial walks through training process

### Demo Ready When:
- [ ] 3-minute demonstration is compelling
- [ ] Non-programmer can understand the concept
- [ ] Multiple interesting demos are possible
- [ ] Reset/retry allows experimentation

---

## Files to Update

1. **components/agents/crates/tt-rs-robot/** - Robot struct, actions
2. **components/app/crates/tt-rs-app/src/robot_exec/** - Execution engine
3. **components/app/crates/tt-rs-app/src/ops/** - Recording during training
4. **documentation/plan.md** - Update roadmap
5. **documentation/prd.md** - Update requirements
6. **documentation/tutorials-roadmap.md** - Tutorial specifications
