# Tutorials and Demos Roadmap

This document outlines the planned tutorials and demonstrations for tt-rs, organized by the features they showcase. Each tutorial builds on previous concepts.

## Tutorial Philosophy

Following Ken Kahn's approach from the original ToonTalk:

1. **Learning by doing**: Users discover through interaction, not reading
2. **Concrete before abstract**: Start with specific examples, then generalize
3. **Immediate feedback**: Every action has visible results
4. **No failure states**: Incorrect actions are simply undone, not punished

## Current Demo (Live)

**URL**: https://sw-fun.github.io/tt-rs/

**Features demonstrated:**
- Dragging widgets from the copy source palette
- Number arithmetic (drop number on number)
- Box manipulation (resize with 0-9 keys while dragging)
- Box joining (drop box on edge of another)
- Box splitting (drop box on number)
- Scales comparison (drop numbers on scales)
- Robot training (click robot to start/stop training)
- Wand tool (copy widgets)
- Vacuum tool (remove widgets)

---

## MVP Priority: Robot Training Tutorial

**This is the primary tutorial for blog/vlog demonstrations.** The robot training experience is what makes ToonTalk unique - users teach robots by showing them what to do.

### Tutorial: Teach Your First Robot (Blog/Vlog Ready)

**Concept**: Programming by demonstration - show the robot once, it repeats forever

**Duration**: 3-5 minutes (ideal for video demo)

**Prerequisites**: None (this is the flagship tutorial)

**Scenario**: Teach a robot to add 5 to any number

**Steps**:

1. **Setup the workspace**
   - Drag a number "7" from the palette to the workspace
   - Drag a "+5" number from the palette nearby
   - Drag the robot to the workspace

2. **Enter training mode**
   - Click the robot (it glows yellow - "I'm watching!")
   - The robot is now recording your actions

3. **Demonstrate the task**
   - Drag the "+5" onto the "7"
   - Result: "12" appears
   - This action is now recorded

4. **Stop training**
   - Click the robot again (glow stops)
   - Robot now "knows" how to add 5

5. **Test with new input**
   - Drag a new number "3" to the workspace
   - Click the robot to run
   - Watch: robot picks up "+5" and drops it on "3" → "8"

6. **Generalize (advanced)**
   - Use vacuum on the "7" in robot's memory
   - Now robot works on ANY number, not just 7

**Key talking points for blog/vlog:**
- "Programming without typing code"
- "Show once, replay forever"
- "The robot watches and learns"
- "Erase details to make it work on any input"

**Visual highlights**:
- Yellow glow during training (recording indicator)
- Thought bubble showing what robot learned
- Animated replay during execution

**Success criteria**:
- User can train robot in <2 minutes
- Robot successfully replays on different inputs
- Concept of "teaching" is intuitive

---

## Robot MVP Tutorial Series

These tutorials are designed for blog/vlog demonstration and map to implementation phases.
See [robot-mvp-design.md](robot-mvp-design.md) for technical details.

### RT1: The Adding Robot (Tier 1 - Works Now)

**Implementation Phase**: None - should work with current code

**Learning Goal**: Basic training and replay

**Setup**: Number "7", number "+5", robot

**Steps**:
1. Click robot (yellow glow = training)
2. Drag +5 onto 7 (becomes 12)
3. Click robot (training stops)
4. Drag new number "3" to workspace
5. Click robot (replays: adds 5 to... wait, it adds 5 to the original 7!)

**Discussion Point**: This reveals a limitation - robot recorded "add 5 to widget #7", not "add 5 to whatever number is there". This motivates pattern matching.

### RT2: The Organizer (Tier 2 - Needs Held Widget)

**Implementation Phase**: Phase 0.1 (Held Widget State)

**Learning Goal**: Pick up and drop actions

**Setup**: Number "42", empty 2-hole box, robot

**Steps**:
1. Click robot to train
2. Drag number 42 into box hole 0
3. Click robot to stop
4. Place new number outside box
5. Click robot - it picks up number and puts it in box

**Key Insight**: Robot learned "pick up the number, put it in hole 0"

### RT3: Watch the Robot Work (Tier 2 - Needs Watched Execution)

**Implementation Phase**: Phase 0.2 (Watched Execution)

**Learning Goal**: See robot's actions step-by-step

**Setup**: Same as RT2, but with watched mode enabled

**Steps**:
1. Train robot as in RT2
2. Enable "Watch Robot" mode
3. Click robot to run
4. See animated cursor pick up widget, move to box, drop it
5. Adjust speed: slow, normal, fast, step-by-step

**Visual Elements**:
- Robot cursor (small hand icon)
- Trail showing movement path
- Highlight on current target
- Speed slider or buttons

### RT4: Make It Work for Any Number (Tier 3 - Needs Pattern Matching)

**Implementation Phase**: Phase 0.3 (Pattern Matching)

**Learning Goal**: Generalization through erasure

**Setup**: Robot trained on specific number (e.g., 7)

**Steps**:
1. Robot currently only works on "7"
2. Open robot's "thought bubble" (conditions view)
3. Use vacuum on the "7" - it becomes a blank/ghost
4. Now robot works on ANY number
5. Test with 42, 100, -5 - all work!

**Key Concepts**:
- "Erased" = "matches anything of same type"
- Vacuum removes detail, making patterns more general
- This is how one training generalizes to infinite inputs

### RT5: The Doubler (Tier 3 - Needs Value Bindings)

**Implementation Phase**: Phase 0.4 (Relative Paths)

**Learning Goal**: Robot uses matched input in actions

**Setup**: Number, robot, trained to "double"

**Steps**:
1. Train robot on number 5
2. Copy the 5 (with wand)
3. Drop copy onto original (5 + 5 = 10)
4. Erase the condition
5. Test: give robot 7, get 14

**Why This Matters**: Robot doesn't just replay "copy widget #5", it copies "the matched input" - whatever that is.

### RT6: Maximum Finder (Tier 4 - Needs Scales)

**Implementation Phase**: Requires scales in conditions

**Learning Goal**: Conditional behavior

**Setup**: Box with [5, 3], scales, two robots

**Robot 1**:
1. Train: put numbers on scales
2. Scales tip left (5 > 3)
3. Pick up left number (the bigger one)
4. Erase numbers in condition

**Robot 2** (handles other case):
1. Train when scales tip right
2. Pick up right number

**Result**: Robot team always returns the larger number

### RT7: Count to 10 (Tier 5 - Needs Bird/Nest)

**Implementation Phase**: Bird/Nest messaging + robot recursion

**Learning Goal**: Loops through message passing

**Setup**: Nest with number 1, bird paired to nest, robot, scales

**Steps**:
1. Robot receives number from nest
2. Compare to 10 with scales
3. If less: add 1, give to bird
4. Bird delivers to nest
5. Robot activates again
6. Repeat until 10

**Key Insight**: No explicit loop construct - recursion through messaging

---

## Phase 1 Tutorials: Current Features

### Tutorial 1.1: Numbers and Arithmetic

**Concept**: Visual arithmetic through drag-and-drop

**Steps**:
1. Drag a "+5" from the palette to the workspace
2. Drag a "+1" and drop it onto the 5
3. Observe the result: 6
4. Try different operators (-, *, /)
5. Experiment with fractions (1/2, 1/3, etc.)

**Learning outcomes**:
- Numbers can be combined by dropping one on another
- The operator on the dropped number determines the operation
- All arithmetic is exact (rational numbers, no rounding)

### Tutorial 1.2: Boxes and Organization

**Concept**: Containers with compartments (data structures)

**Steps**:
1. Drag a 2-hole box to the workspace
2. Drop a number into the first hole
3. Drop another number into the second hole
4. While dragging a box, press 5 to make it a 5-hole box
5. Join two boxes by dropping one on the edge of another
6. Split a box by dropping it on a number

**Learning outcomes**:
- Boxes organize data into compartments
- Box size can be changed with keyboard 0-9
- Boxes can be combined (joined) or divided (split)

### Tutorial 1.3: Scales and Comparison

**Concept**: Visual comparison of values

**Steps**:
1. Drag scales to the workspace
2. Drop a number on the left side
3. Drop a number on the right side
4. Observe which way the scales tip
5. Try equal values (scales balance)

**Learning outcomes**:
- Scales compare values visually
- Larger values make their side heavier
- Equal values keep scales balanced

### Tutorial 1.4: Robot Training Basics

**Concept**: Programming by demonstration

**Steps**:
1. Create a number "5" and a number "+1"
2. Click the robot to start training (yellow glow)
3. Drag the +1 onto the 5 (making 6)
4. Click the robot to stop training
5. Create new numbers and click robot again
6. Observe: robot replays your action on new data

**Learning outcomes**:
- Robots learn by watching you
- Training records your actions
- Robots can repeat actions automatically

### Tutorial 1.5: Tools - Copy and Remove

**Concept**: Manipulating the workspace

**Steps**:
1. Drag the wand (magic stick with star) to the workspace
2. Touch any widget with the wand - it creates a copy
3. Drag the vacuum to the workspace
4. Touch any widget with vacuum - it removes it
5. Hold vacuum on a box hole to remove just that content

**Learning outcomes**:
- Wand copies any widget
- Vacuum removes widgets
- Tools can target specific parts of containers

---

## Phase 2 Tutorials: Bird/Nest Messaging (Planned)

### Tutorial 2.1: Meet the Bird and Nest

**Prerequisites**: None
**Features needed**: Bird widget, Nest widget

**Steps**:
1. Drag a bird and its nest to the workspace
2. Notice they share a color (paired)
3. Drop a number onto the bird
4. Watch the bird fly to its nest and deposit the number
5. Take the number from the nest

**Learning outcomes**:
- Birds carry things to their nests
- This is how parts of a program communicate
- Messages can be stored in nests until needed

### Tutorial 2.2: Bird Messaging with Robots

**Prerequisites**: Tutorial 1.4 (Robot basics), Tutorial 2.1
**Features needed**: Bird, Nest, Robot

**Steps**:
1. Create a bird, nest, robot, and number
2. Train robot: pick up number, give to bird
3. Stop training
4. Give robot new numbers to send
5. Collect messages from nest

**Learning outcomes**:
- Robots can use birds to send messages
- This enables communication between program parts
- Messages queue up in nests

---

## Phase 3 Tutorials: Pattern Matching (Planned)

### Tutorial 3.1: Erasing for Generalization

**Prerequisites**: Tutorial 1.4 (Robot basics)
**Features needed**: Erasure system, improved vacuum

**Steps**:
1. Train robot to double a specific number (e.g., 5 → 10)
2. Robot only works on that exact number
3. Use vacuum on the "5" in robot's memory
4. The number becomes "erased" (any number)
5. Now robot doubles ANY number

**Learning outcomes**:
- Vacuum can erase details from patterns
- Erased widgets match anything of the same type
- This makes robots more general (work with any input)

### Tutorial 3.2: Matching Box Patterns

**Prerequisites**: Tutorial 3.1
**Features needed**: Box pattern matching

**Steps**:
1. Train robot with a 2-hole box containing [3, 5]
2. Erase the numbers to [?, ?]
3. Robot now works on any 2-hole box
4. Try different box contents

**Learning outcomes**:
- Box patterns match structure
- Erasing contents generalizes the pattern
- Robot binds matched values to use in actions

---

## Phase 4 Tutorials: Sensors (Planned)

### Tutorial 4.1: Keyboard Sensor

**Prerequisites**: Tutorial 1.4
**Features needed**: Keyboard sensor

**Steps**:
1. Add keyboard sensor to workspace
2. Train robot to respond when sensor activates
3. Press key, watch sensor produce key code
4. Robot performs action in response

**Learning outcomes**:
- Sensors detect external events
- Keyboard sensor captures key presses
- Combine sensors with robots for interactive programs

---

## Phase 5 Tutorials: Recursion with Birds (Planned)

### Tutorial 5.1: Counting Down

**Prerequisites**: Tutorials 2.x, 3.x
**Features needed**: Bird/Nest, Pattern matching, Scales

**Steps**:
1. Create countdown robot:
   - If number > 0: subtract 1, send to bird
   - If number = 0: stop
2. Give bird to nest's own bird (self-reference)
3. Start with number 5
4. Watch countdown: 5 → 4 → 3 → 2 → 1 → 0

**Learning outcomes**:
- Robots can send work to themselves
- This creates loops (recursion)
- Conditions (scales) control when to stop

### Tutorial 5.2: Computing Factorial

**Prerequisites**: Tutorial 5.1
**Features needed**: All of above

**Steps**:
1. Create factorial robot:
   - Box with [n, accumulator]
   - If n > 1: multiply acc by n, send [n-1, new-acc] to bird
   - If n = 1: result is accumulator
2. Start with [5, 1]
3. Watch: [5,1] → [4,5] → [3,20] → [2,60] → [1,120]
4. Result: 120

**Learning outcomes**:
- Complex algorithms can be built visually
- State is carried in boxes
- Recursion terminates with base case

---

## Phase 6 Tutorials: Houses & Concurrency (Future)

### Tutorial 6.1: Multiple Processes with Houses

**Prerequisites**: All previous
**Features needed**: House, City view, Truck

**Steps**:
1. Create a robot and box
2. Load them into a truck
3. Truck drives away, creates new house
4. Enter new house to see robot working
5. Create more houses
6. Use birds to communicate between houses

**Learning outcomes**:
- Houses are independent processes
- Trucks spawn new processes
- Birds connect processes through message passing

---

## Demo Programs to Include

### Demo 1: Simple Calculator
- Add two numbers in a box
- Robot trained to add them and replace with result

### Demo 2: Number Sorter (with scales)
- Two numbers in a box
- Robot uses scales to compare, swaps if needed

### Demo 3: Counter (with bird/nest)
- Incrementing counter using recursive messaging

### Demo 4: Fibonacci Sequence (advanced)
- Compute Fibonacci using birds and boxes

### Demo 5: Interactive Drawing (with sensors)
- Keyboard controls position
- Robot moves element in response

---

## Tutorial Implementation Notes

### In-App Tutorial System

When implementing tutorials:

1. **Overlay guidance**: Highlight relevant UI elements
2. **Step tracking**: Know which step user is on
3. **Gentle hints**: After delay, show hint if stuck
4. **Skip option**: Allow experienced users to skip ahead
5. **Progress saving**: Remember completed tutorials

### Help Panel Integration

Current help panel sections map to tutorials:

| Help Section | Tutorials |
|--------------|-----------|
| Getting Started | 1.1, 1.2 |
| Numbers | 1.1 |
| Boxes | 1.2 |
| Tools | 1.5 |
| Robots | 1.4, 3.1, 3.2 |

### URL-Based Tutorial Loading

Tutorials and puzzles are loaded via hash-based URLs for GitHub Pages compatibility:

```
https://sw-fun.github.io/tt-rs/#/tutorial/counting-down
https://sw-fun.github.io/tt-rs/#/puzzle/fill-a-box
```

**Benefits:**
- Bookmarkable and shareable links
- Browser reload stays on same puzzle/tutorial
- Deep linking directly to specific content
- History navigation (back/forward buttons work)

### Puzzle/Tutorial Usability Features

Each puzzle and tutorial includes these interactive features:

#### Reset Button
- Restores puzzle to initial state
- Clears all user actions and modifications
- Allows retry without page reload

#### Undo/Redo
- Revert most recent action
- Step backward through action history
- Enable experimentation without fear of mistakes

#### "Show Me" Animated Demo
- Watch an automated demonstration of the solution
- Animated drag-and-drop showing each step
- Visual cues (glow, trail) highlight the action
- Pause/step-through capability for learning

#### Progressive Hints
- After 60 seconds of no progress: generic encouragement
- After 2 minutes: specific hint about next step
- "Give Up" option shows solution without penalty

### Tutorial/Puzzle Error Feedback

Visual feedback helps users understand puzzle state:

| State | Visual Feedback |
|-------|-----------------|
| Initial | Dashed border, neutral colors |
| Correct Answer | Green background, checkmark (✓), success message |
| Wrong Answer | Red/pink background, X mark (✗), "Try again!", shake animation |

Wrong answers allow retry - widget bounces back to original position, and error feedback clears on next attempt

---

## Success Metrics for Tutorials

### Effectiveness Measures

1. **Completion rate**: % of users finishing each tutorial
2. **Time to complete**: Average time per tutorial
3. **Help requests**: How often users need hints
4. **Concept retention**: Can users apply concepts later?

### Target Metrics

| Tutorial Level | Target Completion | Target Time |
|----------------|-------------------|-------------|
| Beginner (1.x) | 90% | < 5 minutes |
| Intermediate (2.x, 3.x) | 75% | < 10 minutes |
| Advanced (5.x) | 50% | < 20 minutes |

---

## References

- [Original ToonTalk Tutorials](http://www.toontalk.com/English/tutorial.htm)
- [ToonTalk Reborn Manual](https://github.com/ToonTalk/ToonTalk/wiki)
- Ken Kahn's papers on educational programming

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
