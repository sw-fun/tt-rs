# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

tt-rs ("Cartoon-oriented Talking Programming Application") is a Rust/WebAssembly reimplementation of ToonTalk, a visual programming environment for learning. Users create programs by training robots that watch and learn from their actions.

This is a derived work based on ToonTalk by Ken Kahn. See COPYRIGHT and LICENSE files.

### Current Implementation Status (February 2026)

**Working Features:**
- Number widget (rational arithmetic with +, -, *, / operators)
- Box widget (configurable holes 0-9, contents management)
- Text widget (basic display)
- Scales widget (numeric comparison, tipping animation)
- Robot widget (training mode, action recording, basic execution)
- Bird widget (SVG rendering, copy source)
- Nest widget (SVG rendering, message queue structure)
- Tools: Wand (copy), Vacuum (remove)
- Drag-and-drop with visual feedback
- Box joining (drop box on edge of another)
- Box splitting (drop box on number)
- Help panel with tutorials and user levels (tt1/tt2)
- Tooltips on all widgets
- Compact footer with GitHub/License/Changelog links
- Puzzle system with URL routing, Reset button, and "Show Me" animated demos

**In Progress:**
- Bird/Nest pairing and message delivery
- Pattern matching and erasure system

**Next Priorities:**
1. Complete Bird/Nest messaging (pairing + delivery)
2. Pattern matching with erasure
3. Persistence (save/load workspaces)

See [plan.md](documentation/plan.md) for detailed roadmap.

## IMPORTANT: Always Use Project Scripts

**Always use project scripts for building and serving.** This prevents regressions and ensures reproducible builds.

```bash
# Development server (DEDICATED PORT: 1140)
./scripts/serve.sh           # Start dev server at http://127.0.0.1:1140

# Quality checks (run before every commit)
./scripts/build-all.sh       # Build + test + clippy + fmt

# Production build for GitHub Pages
./scripts/build-release.sh   # Build and copy to docs/

# Automation scripts (run routinely to reduce manual work)
./scripts/pre-commit-checks.sh      # Run all checks before committing
./scripts/check-live-demo.sh        # Verify live demo is up to date
./scripts/check-changelog.sh        # Verify CHANGELOG completeness
./scripts/check-zindex.sh           # Verify CSS z-index conventions
./scripts/update-readme-screenshots.sh  # Update screenshot timestamps

# Claude slash commands
/my-checkpoint                       # Full workflow: validate, commit, deploy, push
/pre-commit                          # Run pre-commit checks and fix issues
/check-quality                       # Run sw-checklist analysis
```

## Claude Slash Commands

Use these commands to invoke automated workflows:

- `/my-checkpoint` - **Full workflow**: validate, update screenshot if needed, commit, deploy, push
- `/pre-commit` - Run pre-commit checks (build, live demo, changelog, screenshots)
- `/check-quality` - Run sw-checklist modularity analysis

## IMPORTANT: Commit Policy

**DO NOT commit changes without explicit user approval.**

When making code changes:
1. Make the code change
2. Run `./scripts/build-all.sh` to verify it builds
3. **STOP and report** what was changed
4. **WAIT for user** to either:
   - Run `/my-checkpoint` to execute the full deployment workflow
   - Explicitly say "commit" or "push"
   - Give other instructions

Never auto-commit or auto-push. The user controls when changes are committed.

**Screenshot Requirements** (documented in /pre-commit):
- Use the HIGHEST/LATEST user level (currently tt2)
- Expand the accordion for the MOST RECENTLY ADDED feature
- Do NOT show "About" section - show actual feature help
- Goal: Git history shows feature evolution over time

**Before starting development:** Read [learnings.md](documentation/learnings.md) for solutions to common issues.

## Build & Development Commands

```bash
# Prerequisites (one-time setup)
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

# Manual commands (prefer scripts above)
cargo test               # Run all tests
cargo test test_name     # Run specific test

# Kill any existing server on dedicated port
lsof -ti:1140 | xargs kill -9 2>/dev/null

# Quality (run before every commit)
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
cargo fmt --all
sw-checklist .           # Check modularity and standards
```

**Note:** This project uses port 1140 exclusively to avoid conflicts with other projects.

**Important:** Run clippy with `--target wasm32-unknown-unknown` - without this target, clippy reports false `dead_code` warnings for methods used only in WASM runtime.

## Multi-Component Architecture

This project uses a **multi-component architecture** organized by programming concept:

```
components/
├── core/        → Widget trait, WidgetId, MatchResult (no dependencies)
├── values/      → Number, Text (primitive data types)
├── containers/  → Box (data structures)
├── comparison/  → Scales (value comparison)
├── agents/      → Robot (autonomous behavior)
├── tools/       → Wand, Vacuum (user manipulation)
├── dnd/         → Drag-and-drop, UI components (Help, Tooltip)
├── state/       → Position, BoxContents, TrainingState
├── handlers/    → Hit testing utilities
├── commands/    → Command pattern (Move, Remove)
└── app/         → WASM entry point, Trunk config (depends on all)
```

**Dependency flow**: `core` ← `values/containers/comparison/agents/tools` ← `dnd` ← `state/handlers/commands` ← `app`

See [physical-design.md](documentation/physical-design.md) for detailed architecture and phased feature reveal strategy.

### Key Files

- `components/app/crates/tt-rs-app/index.html` - Trunk entry point
- `components/core/crates/tt-rs-core/src/widget_trait.rs` - Core `Widget` trait
- `components/app/crates/tt-rs-app/src/demo.rs` - Demo widget setup
- `docs/` - GitHub Pages deployment (built output, not source)

### Building Components

```bash
# Build all components (recommended)
./scripts/build-all.sh

# Build/test individual component
./components/core/scripts/build.sh
cd components/values && cargo test

# Run sw-checklist on individual components
cd components/core && sw-checklist
```

### Modularity Guidelines (sw-checklist)

**Strict limits (max 4 items per level):**
- Functions: max 4 per module (warn >3)
- Modules: max 4 per crate (warn >3)
- Crates: max 4 per component (warn >3)
- Components: unlimited (can split to separate repos later)

Keep modules focused to leave room for future features.

## Core Abstraction: Widget Trait

All visual programming objects implement the `Widget` trait (`components/core/crates/tt-rs-core/src/widget_trait.rs`):

```rust
pub trait Widget: std::fmt::Debug {
    fn type_name(&self) -> &'static str;                    // "number", "box", "text"
    fn id(&self) -> WidgetId;                               // Unique identifier
    fn copy(&self) -> Box<dyn Widget>;                      // Deep copy with new ID
    fn matches(&self, other: &dyn Widget) -> MatchResult;   // Pattern matching (Match/NoMatch)
    fn render(&self) -> Html;                               // Yew component rendering
    fn description(&self) -> String;                        // Human-readable text
}
```

**Implemented widgets**: Number, Text, Box, Robot, Scales, Bird, Nest, Wand, Vacuum

## Technology Stack

- **Rust 2024 edition** - Latest stable with improved linting and language features
- **Yew** for reactive UI components
- **Trunk** for WASM bundling (run from tt-rs-app crate)
- **Three.js** (planned) for 3D graphics
- **SVG/CSS** for 2D graphics and animations

## Development Guidelines

### Pre-Commit Checklist

Run `./scripts/build-all.sh` or manually:

1. `cargo test` - all tests pass
2. `cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings`
3. `cargo fmt --all`
4. `sw-checklist .` - check modularity standards

### Code Quality

- Zero clippy warnings
- **NEVER use `#[allow(...)]` to suppress warnings** - always fix the underlying issue
- All public APIs have doc comments
- Rust 2024 edition idioms
- Use inline format args: `format!("{name}")` not `format!("{}", name)`

**CRITICAL: Never bypass clippy or compiler warnings with `#[allow(unused_imports)]`, `#[allow(dead_code)]`, or similar attributes. These warnings exist for a reason - fix the actual problem by removing unused code, fixing the issue, or restructuring. The only exception is `#[allow(clippy::too_many_arguments)]` for complex callback configuration structs where refactoring would reduce clarity.**

### Adding New Features

1. Determine which component the feature belongs to (by concept, not "widget")
2. Create new crate in appropriate component: `components/<concept>/crates/tt-rs-<name>/`
3. Implement `Widget` trait from `tt-rs-core`
4. Add to widget enum in app component
5. Add copy source for palette
6. Update help panel documentation
7. Update [physical-design.md](documentation/physical-design.md) with new feature's phase

## DEPLOYMENT CHECKLIST - MANDATORY

**DO NOT BREAK THE LIVE DEMO.** Always use `./scripts/build-release.sh` for deployments.

### Common Mistakes (DO NOT REPEAT)

1. **Missing `--public-url /tt-rs/`** - GitHub Pages serves at `/<repo>/` subdirectory
   - Symptom: Blank page, 404 errors for .wasm, .js, .css files
   - Solution: Always use `./scripts/build-release.sh` which includes validation
   - NEVER run `trunk build --release` directly for deployment

2. **Missing `.nojekyll` file** - GitHub Jekyll processing breaks WASM
   - Symptom: Files with underscores don't load
   - Solution: Script automatically creates this file

3. **Forgetting to copy to docs/** - Build goes to dist/, not docs/
   - Solution: Script handles this automatically

### Deployment Steps

```bash
# 1. ALWAYS use the release script
./scripts/build-release.sh

# 2. The script will:
#    - Build with --public-url /tt-rs/
#    - Copy to docs/
#    - Create .nojekyll
#    - VALIDATE paths contain /tt-rs/ prefix
#    - FAIL if validation fails

# 3. Commit and push
git add docs/
git commit -m "Deploy to GitHub Pages"
git push

# 4. Verify live demo works at:
#    https://sw-fun.github.io/tt-rs/
```

### Before Any Deployment

- [ ] Read [learnings.md](documentation/learnings.md) if encountering issues
- [ ] Use `./scripts/build-release.sh` (NOT manual trunk commands)
- [ ] Verify script validation passes
- [ ] Test locally if possible
- [ ] Check live demo after push

## ToonTalk Background

ToonTalk exists in three versions:

1. **Original ToonTalk (1995)**: 3D C++ desktop app with cities, houses, trucks, bombs, helicopter
2. **ToonTalk Reborn (2014-2017)**: JavaScript/jQuery web version, simplified (no houses/city)
3. **tt-rs (this project)**: Rust/WASM reimplementation

**Current goal**: Match ToonTalk Reborn features, then add original ToonTalk features, then innovate.

Key concepts:
- **Robot**: Trained by demonstration, executes recorded actions
- **Bird/Nest**: Asynchronous message passing (actor model)
- **Box**: Container with numbered holes
- **Scales**: Visual comparison
- **Erasure**: Generalize patterns by removing detail

## CHANGELOG Workflow

Use `<latest>` as placeholder for current commit SHA. Replace it with the actual SHA when making the NEXT commit. See [learnings.md](documentation/learnings.md#changelog-workflow-avoiding-commit-sha-loops) for detailed workflow.

## Documentation

### IMPORTANT: Read These First

- [process.md](documentation/process.md) - **Development workflow and commit guidelines**
- [learnings.md](documentation/learnings.md) - **Solutions to issues encountered - ALWAYS consult before debugging!**

### Reference Documentation

- [plan.md](documentation/plan.md) - Implementation roadmap with next steps
- [physical-design.md](documentation/physical-design.md) - Component architecture & phased feature reveal
- [architecture.md](documentation/architecture.md) - System design and actual implementation
- [prd.md](documentation/prd.md) - Product requirements with status tracking
- [design.md](documentation/design.md) - Technical design decisions
- [tutorials-roadmap.md](documentation/tutorials-roadmap.md) - Demo and tutorial planning

### Development Workflow

1. **Before starting work**: Read `learnings.md` for known issues and solutions
2. **During development**: Follow `process.md` for workflow guidelines
3. **Before committing**: Run `./scripts/build-all.sh` and check validation passes
4. **After committing any feature/fix**: Run `./scripts/build-release.sh`, commit docs/, push

## MANDATORY: Live Demo Must Always Be Current

**CRITICAL: The live demo MUST be updated after EVERY feature or fix that changes behavior.**

The live demo at https://sw-fun.github.io/tt-rs/ is the primary way users experience tt-rs. Leaving it out of date is unacceptable.

### Complete Feature/Fix Workflow

```bash
# 1. Make your code changes
# 2. Run quality checks
./scripts/build-all.sh

# 3. Commit code changes
git add -A
git commit -m "feat/fix: description"

# 4. MANDATORY: Update live demo
./scripts/build-release.sh
git add docs/
git commit -m "deploy: Update live demo with <feature/fix description>"
git push

# 5. Verify live demo at https://sw-fun.github.io/tt-rs/
```

### DO NOT:
- Push code changes without updating the live demo
- Leave multiple commits without a corresponding deploy
- Skip the deploy step "to do later" - do it NOW

### Enforcement

If you (AI assistant or human) make feature/fix commits without updating the live demo, you have broken the process. Immediately run `./scripts/build-release.sh` and deploy.
