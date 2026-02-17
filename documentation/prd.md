# tt-rs Product Requirements Document

**Full Name**: Cartoon-oriented Talking Programming Application

## Executive Summary

tt-rs is a modern reimplementation of ToonTalk, an interactive visual programming environment originally created by Ken Kahn in the 1990s. This project brings the innovative programming-by-demonstration approach to modern web browsers using Rust, WebAssembly, and contemporary rendering technologies.

**Note**: This is a derived work. See the COPYRIGHT file for full attribution.

## Background

### ToonTalk History

ToonTalk has evolved through three major versions:

#### Original ToonTalk (Desktop C++ ~1995-2009)

A 3D animated world where programming concepts are mapped to concrete metaphors:

**Core Metaphors:**
| Concept | Metaphor | Description |
|---------|----------|-------------|
| Program | City | Collection of running processes |
| Process | House | Container where robots work |
| Method | Robot | Trained by demonstration |
| Message | Bird delivery | Asynchronous communication |
| Data | Box with holes | Containers with compartments |
| Comparison | Scales | Physical balance that tips |
| Pattern | Erased widget | Generalized by removing detail |

**Navigation & Process Management:**
- **Helicopter**: Fly over the city to navigate between houses
- **Truck**: Drop robot+box to spawn a new house (process)
- **Bomb**: Destroys a house (terminates process)
- **Notebook**: Store programs, serve as modules when dropped in truck

**Tools:**
- **Wand (Magic)**: Copy any widget
- **Vacuum (Dusty)**: Remove/erase widgets (suck, erase, spit modes)
- **Pump (Pumpy)**: Resize widgets

The system was influenced by Janus (concurrent constraint logic programming), the Actor model, and video games like Zelda and Robot Odyssey. Successfully used by children as young as 3 years old.

#### ToonTalk Reborn (JavaScript/jQuery ~2014-2017)

A web-based version that simplified some concepts for browser compatibility:

**Features Retained:**
- Numbers, Boxes, Text, Robots, Birds, Nests, Scales
- Wand and Vacuum tools
- Robot training and execution
- Pattern matching via erasure
- Sensors for keyboard/mouse events

**Features Removed/Changed:**
- No 3D city navigation
- No houses (flat workspace instead)
- No trucks, bombs, helicopter
- Simplified vacuum (no suck/spit modes)
- 2D interface only

**Technology:**
- jQuery and jQuery UI
- JSON program storage
- Google Drive integration
- Multilingual (100+ languages)

#### tt-rs (Rust/WebAssembly ~2024-present)

This project aims to:
1. Modernize the technology stack (Rust/WASM)
2. Initially match ToonTalk Reborn capabilities
3. Eventually restore original ToonTalk features
4. Add new innovations (3D graphics, design patterns, AI assistance)

## Product Vision

**For** children, educators, and curious adults
**Who** want to learn programming through visual, interactive exploration
**tt-rs is** a web-based visual programming environment
**That** teaches computational thinking through animated metaphors and programming by demonstration
**Unlike** text-based programming or simplified block coding
**Our product** provides a deeply interactive, game-like experience where abstract concepts become tangible objects.

## Target Users

### Primary Users

1. **Children (ages 5-12)**
   - Little to no programming experience
   - Comfortable with touch/mouse interfaces
   - May not read fluently

2. **Educators**
   - Teachers introducing computational thinking
   - After-school program leaders
   - Homeschool parents

3. **Curious Adults**
   - Alternative programming paradigm enthusiasts
   - Those intimidated by traditional programming
   - Concurrent programming learners

### Secondary Users

1. **Researchers**: CS education, programming languages, HCI
2. **Developers**: Rust/WASM game development, educational software

## Requirements by Phase

### Phase 1: MVP (ToonTalk Reborn Parity)

Goal: Match the core functionality of the 2017 JavaScript version.

#### FR1: Core Widgets (MVP)

| ID | Widget | Priority | Status |
|----|--------|----------|--------|
| FR1.1 | Number (rational arithmetic, operators) | Must Have | ✅ Done |
| FR1.2 | Box (container with holes) | Must Have | ✅ Done |
| FR1.3 | Text (string widget) | Must Have | ✅ Done |
| FR1.4 | Robot (programmable agent) | Must Have | ✅ Done |
| FR1.5 | Scales (comparison) | Must Have | ✅ Done |
| FR1.6 | Bird (message carrier) | Must Have | ❌ Pending |
| FR1.7 | Nest (message receiver) | Must Have | ❌ Pending |
| FR1.8 | Sensor (event detection) | Should Have | ❌ Pending |
| FR1.9 | DropZone (verification target) | Should Have | ❌ Pending |
| FR1.10 | Copy source for Box widget | Should Have | ❌ Pending |

#### FR2: Tools (MVP)

| ID | Tool | Priority | Status |
|----|------|----------|--------|
| FR2.1 | Wand (copy) | Must Have | ✅ Done |
| FR2.2 | Vacuum (remove) | Must Have | ✅ Done |
| FR2.3 | Vacuum (erase - pattern creation) | Should Have | ❌ Pending |

#### FR3: Robot Programming (MVP) - PRIORITY FOCUS

**The robot is the core differentiating feature of ToonTalk.** This must be demo-ready for blog/vlog content.

See [robot-mvp-design.md](robot-mvp-design.md) for detailed behavior specifications.

| ID | Feature | Priority | Status | Notes |
|----|---------|----------|--------|-------|
| FR3.1 | Train by demonstration | Must Have | ✅ Done | Click to start/stop training |
| FR3.2 | Record arithmetic actions | Must Have | ✅ Done | +, -, *, / operations recorded |
| FR3.3 | Record copy/remove actions | Must Have | ✅ Done | Wand and vacuum recorded |
| FR3.4 | **Held widget state** | Must Have | ❌ Pending | Robot tracks what it's holding |
| FR3.5 | **PickUp action** | Must Have | ❌ Pending | Record/execute picking up widgets |
| FR3.6 | **Drop action** | Must Have | ❌ Pending | Record/execute dropping held widget |
| FR3.7 | Execute unwatched | Must Have | ✅ Done | Instant replay without animation |
| FR3.8 | **Watched execution** | Must Have | ❌ Pending | Animated cursor shows actions |
| FR3.9 | **Pattern matching** | Must Have | ❌ Pending | Check conditions before running |
| FR3.10 | **Erasure for generalization** | Must Have | ❌ Pending | Vacuum erases values to "any" |
| FR3.11 | **Relative paths** | Must Have | ❌ Pending | Reference widgets by role not ID |
| FR3.12 | Robot teams | Should Have | ❌ Pending | Multiple robots for different cases |
| FR3.13 | Speed control | Should Have | ❌ Pending | instant/slow/step-through modes |

**Demo Milestones:**

| Demo | Dependencies | Description |
|------|--------------|-------------|
| Tier 1: Add 5 | None (works now) | Robot adds constant to number |
| Tier 2: Move to Box | FR3.4-FR3.6 | Robot picks up and places widget |
| Tier 3: Generalize | FR3.9-FR3.11 | Robot works on ANY matching input |
| Tier 4: Conditional | FR3.9 + Scales | Robot chooses based on comparison |
| Tier 5: Recursive | Bird/Nest | Robot counts to 10, factorial, etc. |

#### FR4: Workspace (MVP)

| ID | Feature | Priority | Status |
|----|---------|----------|--------|
| FR4.1 | Drag and drop | Must Have | ✅ Done |
| FR4.2 | Copy sources (palette) | Must Have | ✅ Done |
| FR4.3 | Help panel | Must Have | ✅ Done |
| FR4.4 | Widget tooltips | Should Have | ✅ Done |

#### FR5: Persistence (MVP)

| ID | Feature | Priority | Status |
|----|---------|----------|--------|
| FR5.1 | Save to localStorage | Must Have | ❌ Pending |
| FR5.2 | Load from localStorage | Must Have | ❌ Pending |
| FR5.3 | Export to file | Must Have | ❌ Pending |
| FR5.4 | Import from file | Must Have | ❌ Pending |
| FR5.5 | Workspace menu (tabbed browser) | Must Have | ✅ Done |
| FR5.6 | Bundled examples/tutorials | Must Have | ❌ Pending |
| FR5.7 | Workspace Notes (editable TextPane) | Must Have | ✅ Done |
| FR5.8 | User level preservation (tt1/tt2) | Must Have | ❌ Pending |
| FR5.9 | Workspace serialization (JSON format) | Must Have | ❌ Pending |

#### FR5B: Workspace Content (MVP)

| ID | Feature | Priority | Status |
|----|---------|----------|--------|
| FR5B.1 | Tutorials tab with guided lessons | Must Have | ✅ UI Done |
| FR5B.2 | Examples tab with demo workspaces | Must Have | ✅ UI Done |
| FR5B.3 | Challenges tab with puzzles | Must Have | ✅ UI Done |
| FR5B.4 | Tutorial workspace files (7 planned) | Must Have | ❌ Pending |
| FR5B.5 | Example workspace files (10 planned) | Should Have | ❌ Pending |
| FR5B.6 | Challenge workspace files (10 planned) | Should Have | ❌ Pending |
| FR5B.7 | ToonTalk Classic Puzzles (3 from 2017) | Should Have | ❌ Pending |

#### FR5C: Puzzle Infrastructure

| ID | Feature | Priority | Status |
|----|---------|----------|--------|
| FR5C.1 | DropZone widget (labelled verification target) | Should Have | ✅ Done |
| FR5C.2 | Pattern matching for drop verification | Should Have | ✅ Done |
| FR5C.3 | Success/failure feedback on drop | Should Have | ✅ Done |
| FR5C.4 | Puzzle auto-advance on success | Could Have | ❌ Pending |
| FR5C.5 | Copy source support for Box | Should Have | ❌ Pending |

#### FR5D: Puzzle/Tutorial Usability

| ID | Feature | Priority | Status |
|----|---------|----------|--------|
| FR5D.1 | Reset button to restart puzzle | Must Have | ✅ Done |
| FR5D.2 | URL-based routing for puzzles | Should Have | ✅ Done |
| FR5D.3 | Bookmarkable/shareable puzzle URLs | Should Have | ✅ Done |
| FR5D.4 | Browser reload stays on same puzzle | Should Have | ✅ Done |
| FR5D.5 | "Show Me" animated solution demo | Could Have | ✅ Done |
| FR5D.6 | Hint system for puzzles | Could Have | ❌ Pending |
| FR5D.7 | Undo last action | Should Have | ❌ Pending |

**Rationale**: Currently, if a user makes a mistake (e.g., putting numbers in wrong order), they cannot easily recover without reloading the page, which exits the puzzle. URL-based routing enables:
- Bookmarking specific puzzles for later
- Sharing puzzle links with others
- Browser back/forward navigation
- Reload staying on current puzzle
- Deep linking from tutorials/documentation

See documentation files for planned content:
- [tutorials.md](tutorials.md) - 7 step-by-step guided lessons
- [examples.md](examples.md) - 10 pre-built demo workspaces
- [challenges.md](challenges.md) - 10 programming puzzles

### Phase 2: Extended MVP

Goal: Polish and extend beyond basic ToonTalk Reborn.

#### FR6: Advanced Features

| ID | Feature | Priority |
|----|---------|----------|
| FR6.1 | Backside views (widget configuration) | Should Have |
| FR6.2 | Text explosion (text → letters in box) | Should Have |
| FR6.3 | Box labels | Should Have |
| FR6.4 | Scales in box holes | Should Have |
| FR6.5 | Element widget (HTML/SVG) | Could Have |
| FR6.6 | Import ToonTalk Reborn JSON | Should Have |

#### FR7: UX Improvements

| ID | Feature | Priority |
|----|---------|----------|
| FR7.1 | Undo/redo | Should Have |
| FR7.2 | Zoom and pan | Should Have |
| FR7.3 | URL-based sharing | Could Have |
| FR7.4 | Multiple workspaces | Could Have |

### Phase 3: Original ToonTalk Features

Goal: Restore the full concurrency model from the original.

#### FR8: Houses & City

| ID | Feature | Priority |
|----|---------|----------|
| FR8.1 | House widget (process container) | Should Have |
| FR8.2 | City view (multiple houses) | Should Have |
| FR8.3 | Enter/exit house navigation | Should Have |

#### FR9: Process Management

| ID | Feature | Priority |
|----|---------|----------|
| FR9.1 | Truck (process spawner) | Should Have |
| FR9.2 | Bomb (process terminator) | Should Have |
| FR9.3 | Helicopter navigation | Could Have |

#### FR10: Additional Tools

| ID | Feature | Priority |
|----|---------|----------|
| FR10.1 | Pumpy (resize tool) | Could Have |
| FR10.2 | Notebook (program storage) | Could Have |
| FR10.3 | Robot teams | Could Have |

### Phase 4: Future Innovations

Goal: Go beyond original ToonTalk with modern capabilities.

#### FR11: Modern Graphics

| ID | Feature | Priority |
|----|---------|----------|
| FR11.1 | Three.js 3D rendering | Could Have |
| FR11.2 | Physics-based animations | Could Have |
| FR11.3 | VR/AR support | Could Have |

#### FR12: Educational Enhancements

| ID | Feature | Priority |
|----|---------|----------|
| FR12.1 | Design pattern library | Could Have |
| FR12.2 | AI-assisted training | Could Have |
| FR12.3 | Classroom/collaboration mode | Could Have |

#### FR13: Audio/Speech

| ID | Feature | Priority |
|----|---------|----------|
| FR13.1 | Sound effects | Could Have |
| FR13.2 | Text-to-speech | Could Have |

## Non-Functional Requirements

### NFR1: Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR1.1 | Initial load time | < 3 seconds |
| NFR1.2 | WASM binary size | < 2MB compressed |
| NFR1.3 | Animation frame rate | 60 FPS |
| NFR1.4 | Robot execution speed | 10,000+ steps/second |

### NFR2: Compatibility

| ID | Requirement | Target |
|----|-------------|--------|
| NFR2.1 | Browsers | Chrome 90+, Firefox 90+, Safari 15+, Edge 90+ |
| NFR2.2 | Devices | Desktop, tablet (touch support) |
| NFR2.3 | Screen size | 1024x768 minimum |

### NFR3: Accessibility

| ID | Requirement | Priority |
|----|-------------|----------|
| NFR3.1 | Keyboard navigation | Should Have |
| NFR3.2 | Screen reader support | Could Have |
| NFR3.3 | High contrast mode | Could Have |
| NFR3.4 | Configurable animation speed | Should Have |

### NFR4: Code Quality

| ID | Requirement | Target |
|----|-------------|--------|
| NFR4.1 | Test coverage | > 80% domain logic |
| NFR4.2 | sw-checklist compliance | Pass all checks |
| NFR4.3 | Zero clippy warnings | Enforced |

## User Stories

### MVP Stories

1. **As a child**, I want to drag a number onto another number to add them, so I can do arithmetic visually.

2. **As a child**, I want to train a robot by showing it what to do, so I can automate tasks.

3. **As a child**, I want to give something to a bird and watch it fly to its nest, so I can understand message passing.

4. **As a child**, I want to put things in boxes and take them out, so I can organize my work.

5. **As an educator**, I want to save and share programs, so students can continue their work.

6. **As a user**, I want to watch a robot work step-by-step, so I can understand what it's doing.

7. **As a user**, I want to erase parts of a pattern, so my robot can work with many different inputs.

### Workspace Persistence Stories

8. **As a user**, I want to save my current workspace with a name and description, so I can return to it later.

9. **As a user**, I want to see a list of my saved workspaces with their descriptions, so I can choose which one to load.

10. **As a user**, I want to load a saved workspace and have it restore all widgets in their saved positions, so I can continue my work.

11. **As an educator**, I want to create a workspace with a trained robot and save it as a tutorial, so students can learn by example.

12. **As a user**, I want to export a workspace to a file, so I can share it with others or back it up.

13. **As a user**, I want to import a workspace from a file someone shared with me, so I can explore their programs.

14. **As a user**, I want to access bundled example workspaces (tutorials) that demonstrate features, so I can learn how to use the application.

15. **As a user**, I want each saved workspace to remember which user level (tt1/tt2) it was created in, so tutorials can target specific learning stages.

### Advanced Stories

16. **As an advanced user**, I want to create recursive programs using birds and nests, so I can compute factorials.

17. **As an advanced user**, I want multiple robots in different houses, so I can build parallel systems.

18. **As an educator**, I want to import ToonTalk Reborn programs, so I can use existing curricula.

## Milestones

### Milestone 1: Foundation ✅ Complete
- Widget trait system
- Number, Box, Text widgets
- Drag and drop
- Basic tools (Wand, Vacuum)

### Milestone 2: Robot Programming (In Progress)
- Robot training ✅
- Action recording ✅
- Basic execution ✅
- Pattern matching ❌
- Erasure system ❌

### Milestone 3: Messaging (Next)
- Bird widget
- Nest widget
- Message delivery animation
- Robot waits for nest

### Milestone 4: Persistence (In Progress)
- Workspace menu with tabs ✅
- Workspace Notes (TextPane) ✅
- JSON serialization ❌
- Save/load workspace ❌
- File import/export ❌
- Bundled tutorials/examples/challenges ❌

### Milestone 5: Polish
- Watched execution
- Backside views
- Sensors
- UX improvements

### Milestone 6: Original Features
- Houses and city
- Trucks and bombs
- Helicopter navigation

### Milestone 7: Innovations
- 3D graphics
- Sound/speech
- Advanced features

## Constraints

### Technical
- Must run entirely in browser (no server execution)
- Must compile to WebAssembly from Rust
- Minimal JavaScript (only browser API bindings)

### Legal
- BSD license compatibility
- Preserve original copyright notices
- Proper derived work attribution

### Resources
- Single primary developer
- Open source, community-driven

## Success Metrics

### MVP Launch
- Core widgets functional
- Save/load working
- 3+ example programs
- Basic documentation

### Post-Launch
- Monthly active users
- Programs created
- Community contributions
- Educational deployments

## Glossary

### Core Concepts
- **Widget**: Any visual object (number, box, robot, etc.)
- **Backside**: Widget's configuration/programming interface
- **Robot**: Automated agent performing recorded actions
- **Training**: Teaching a robot by demonstration
- **Thought Bubble**: Robot's conditions that must match

### Communication
- **Bird**: Messenger carrying items to its nest
- **Nest**: Receiver of messages from its bird

### Data
- **Box**: Container with numbered compartments (holes)
- **Number**: Rational number with optional operator
- **Text**: Text string widget

### Comparison
- **Scale**: Widget that tips based on value comparison

### Process (Original ToonTalk)
- **House**: Process container where robots work
- **City**: Program with multiple houses
- **Truck**: Process spawner
- **Bomb**: Process terminator
- **Helicopter**: City navigation tool

### Tools
- **Wand**: Copy tool
- **Vacuum (Dusty)**: Remove/erase tool
- **Pump (Pumpy)**: Resize tool
- **Notebook**: Program storage and module container

### Events
- **Sensor**: Widget responding to browser events

### Puzzle Support
- **DropZone**: Labeled verification target that checks dropped widgets against an expected pattern
- **Copy Source**: Any widget marked `is_copy_source` produces copies when dragged from (original stays)

## References

- [ToonTalk Wikipedia](https://en.wikipedia.org/wiki/ToonTalk)
- [ToonTalk Papers](http://toontalk.com/English/papers.htm)
- [ToonTalk Reborn Wiki](https://github.com/ToonTalk/ToonTalk/wiki)
- [ToonTalk Reborn Demo](https://toontalk.github.io/ToonTalk/)
- [Ken Kahn's Homepage](https://toontalk.com/English/kenkahn.htm)

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
