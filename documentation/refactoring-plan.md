# tt-rs Refactoring Plan: Multi-Component Architecture

## Overview

This document details the strategy for preserving git history from `old-tt-rs` while establishing a new multi-component architecture in `tt-rs`.

### Project Context

| Directory | Purpose |
|-----------|---------|
| `ToonTalk/` | Original jQuery implementation - reference for "What" (use-cases, behaviors) |
| `old-tt-rs/` | First Rust/Yew attempt - preserves commit history through b156e8a |
| `tt-rs/` | New repo with multi-component architecture |

## Strategy: Git History Preservation

### Recommended Approach: Fetch + Rebase

This approach imports the old-tt-rs history into tt-rs, then performs the refactoring as new commits. Git will track file moves with `-M` flag.

```
old-tt-rs history ──> tt-rs (imported) ──> refactoring commits (file moves tracked)
```

---

## Phase 1: Import History into tt-rs

### Step 1.1: Add old-tt-rs as remote
```bash
cd /Users/mike/github/sw-fun/tt-rs
git remote add old-origin ../old-tt-rs
git fetch old-origin
```

### Step 1.2: Reset to old-tt-rs main branch
```bash
# This makes tt-rs have the exact same history as old-tt-rs
git reset --hard old-origin/main
```

### Step 1.3: Verify history is preserved
```bash
git log --oneline | head -20
# Should show: b156e8a, d7cf990, ff1a1ac, etc.
```

### Step 1.4: Remove the old-origin remote
```bash
git remote remove old-origin
```

### Step 1.5: Set up new GitHub remote (when ready)
```bash
git remote add origin git@github.com:sw-fun/tt-rs.git
```

- [ ] 1.1 Add old-tt-rs as remote
- [ ] 1.2 Reset to old-tt-rs main
- [ ] 1.3 Verify history preserved
- [ ] 1.4 Remove old-origin remote
- [ ] 1.5 Configure new remote (later)

---

## Phase 2: Create Multi-Component Directory Structure

### Target Architecture
```
tt-rs/
├── CLAUDE.md                           # AI agent guidelines
├── COPYRIGHT                           # BSD-3-Clause attribution
├── LICENSE                             # License file
├── README.md                           # Project overview
├── documentation/                      # Design docs, plans
│   ├── architecture.md
│   ├── design.md
│   ├── plan.md
│   ├── prd.md
│   └── refactoring-plan.md            # This file
├── scripts/
│   ├── build-all.sh                   # Delegates to each component
│   └── serve.sh                       # Delegates to app component
├── components/
│   ├── core/                          # Core abstractions (max 5 crates)
│   │   ├── Cargo.toml                 # Workspace
│   │   ├── scripts/build.sh
│   │   └── crates/
│   │       ├── tt-rs-core/            # Widget trait, WidgetId, etc.
│   │       └── tt-rs-macros/          # Proc macros if needed
│   ├── widgets/                       # Widget implementations
│   │   ├── Cargo.toml
│   │   ├── scripts/build.sh
│   │   └── crates/
│   │       ├── tt-rs-number/
│   │       ├── tt-rs-text/
│   │       └── tt-rs-box/
│   ├── dnd/                           # Drag-and-drop system
│   │   ├── Cargo.toml
│   │   ├── scripts/build.sh
│   │   └── crates/
│   │       ├── tt-rs-dnd-core/
│   │       ├── tt-rs-dnd/
│   │       └── tt-rs-web-utils/
│   └── app/                           # Main application
│       ├── Cargo.toml
│       ├── scripts/
│       │   ├── build.sh
│       │   └── serve.sh
│       └── crates/
│           └── tt-rs-app/
│               ├── src/
│               └── index.html
└── docs/                              # GitHub Pages
    ├── .nojekyll
    └── index.html
```

### Step 2.1: Create component directories
```bash
mkdir -p components/{core,widgets,dnd,app}/crates
mkdir -p components/{core,widgets,dnd,app}/scripts
```

### Step 2.2: Move crates to components (git mv for history)
```bash
# Core component
git mv crates/tt-rs-core components/core/crates/
git mv crates/tt-rs-macros components/core/crates/

# Widgets component
git mv crates/tt-rs-number components/widgets/crates/
git mv crates/tt-rs-text components/widgets/crates/
git mv crates/tt-rs-box components/widgets/crates/

# DnD component
git mv crates/tt-rs-dnd-core components/dnd/crates/
git mv crates/tt-rs-dnd components/dnd/crates/
git mv crates/tt-rs-web-utils components/dnd/crates/

# App component
git mv crates/tt-rs-app components/app/crates/
```

- [ ] 2.1 Create component directories
- [ ] 2.2 Move crates with `git mv`

---

## Phase 3: Create Component Workspace Files

Each component needs its own `Cargo.toml` workspace with `[workspace.dependencies]`.

### Step 3.1: Create components/core/Cargo.toml
```toml
[workspace]
resolver = "2"
members = [
    "crates/tt-rs-core",
    "crates/tt-rs-macros",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "BSD-3-Clause"
repository = "https://github.com/sw-fun/tt-rs"

[workspace.dependencies]
yew = { version = "0.21", features = ["csr"] }

[profile.release]
opt-level = "s"
lto = true
```

### Step 3.2: Create components/widgets/Cargo.toml
```toml
[workspace]
resolver = "2"
members = [
    "crates/tt-rs-number",
    "crates/tt-rs-text",
    "crates/tt-rs-box",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "BSD-3-Clause"
repository = "https://github.com/sw-fun/tt-rs"

[workspace.dependencies]
yew = { version = "0.21", features = ["csr"] }
tt-rs-core = { path = "../core/crates/tt-rs-core" }

[profile.release]
opt-level = "s"
lto = true
```

### Step 3.3: Create components/dnd/Cargo.toml
```toml
[workspace]
resolver = "2"
members = [
    "crates/tt-rs-dnd-core",
    "crates/tt-rs-dnd",
    "crates/tt-rs-web-utils",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "BSD-3-Clause"
repository = "https://github.com/sw-fun/tt-rs"

[workspace.dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "console", "Document", "Element", "EventTarget",
    "HtmlElement", "MouseEvent", "Node", "Window", "DomRect",
] }
tt-rs-core = { path = "../core/crates/tt-rs-core" }

[profile.release]
opt-level = "s"
lto = true
```

### Step 3.4: Create components/app/Cargo.toml
```toml
[workspace]
resolver = "2"
members = [
    "crates/tt-rs-app",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "BSD-3-Clause"
repository = "https://github.com/sw-fun/tt-rs"

[workspace.dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "console", "Document", "Element", "EventTarget",
    "HtmlElement", "MouseEvent", "Node", "Window", "DomTokenList",
] }
log = "0.4"
wasm-logger = "0.2"
tt-rs-core = { path = "../core/crates/tt-rs-core" }
tt-rs-number = { path = "../widgets/crates/tt-rs-number" }
tt-rs-text = { path = "../widgets/crates/tt-rs-text" }
tt-rs-box = { path = "../widgets/crates/tt-rs-box" }
tt-rs-dnd = { path = "../dnd/crates/tt-rs-dnd" }

[profile.release]
opt-level = "s"
lto = true
```

- [ ] 3.1 Create core/Cargo.toml
- [ ] 3.2 Create widgets/Cargo.toml
- [ ] 3.3 Create dnd/Cargo.toml
- [ ] 3.4 Create app/Cargo.toml

---

## Phase 4: Update Crate Dependencies

Each crate's `Cargo.toml` needs updated paths for cross-component dependencies.

### Step 4.1: Update tt-rs-core/Cargo.toml
```toml
[package]
name = "tt-rs-core"
description = "Core abstractions for tt-rs widgets"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
yew = { workspace = true }
```

### Step 4.2: Update widget crates (tt-rs-number, tt-rs-text, tt-rs-box)
```toml
[dependencies]
tt-rs-core = { path = "../../../core/crates/tt-rs-core" }
yew = { workspace = true }
```

### Step 4.3: Update dnd crates
```toml
# tt-rs-dnd/Cargo.toml
[dependencies]
tt-rs-dnd-core = { path = "../tt-rs-dnd-core" }
tt-rs-core = { path = "../../../core/crates/tt-rs-core" }
tt-rs-web-utils = { path = "../tt-rs-web-utils" }
yew.workspace = true
wasm-bindgen.workspace = true
web-sys = { workspace = true }
```

### Step 4.4: Update tt-rs-app/Cargo.toml
```toml
[dependencies]
tt-rs-core = { path = "../../../core/crates/tt-rs-core" }
tt-rs-number = { path = "../../../widgets/crates/tt-rs-number" }
tt-rs-text = { path = "../../../widgets/crates/tt-rs-text" }
tt-rs-box = { path = "../../../widgets/crates/tt-rs-box" }
tt-rs-dnd = { path = "../../../dnd/crates/tt-rs-dnd" }
yew = { workspace = true }
wasm-bindgen = { workspace = true }
web-sys = { workspace = true }
log = { workspace = true }
wasm-logger = { workspace = true }
```

- [ ] 4.1 Update tt-rs-core dependencies
- [ ] 4.2 Update widget crate dependencies
- [ ] 4.3 Update dnd crate dependencies
- [ ] 4.4 Update tt-rs-app dependencies

---

## Phase 5: Create Build Scripts

### Step 5.1: Create component build.sh scripts

**components/core/scripts/build.sh:**
```bash
#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$(dirname "$SCRIPT_DIR")"
echo "Building core component..."
cargo build --target wasm32-unknown-unknown
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo fmt --check
echo "Core component built successfully"
```

**components/app/scripts/serve.sh:**
```bash
#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPONENT_DIR="$(dirname "$SCRIPT_DIR")"
APP_CRATE="$COMPONENT_DIR/crates/tt-rs-app"
lsof -ti:1140 | xargs kill -9 2>/dev/null || true
cd "$APP_CRATE"
trunk serve --port 1140
```

### Step 5.2: Create root delegation scripts

**scripts/build-all.sh:**
```bash
#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo "Building all components..."
COMPONENTS=(core widgets dnd app)
for component in "${COMPONENTS[@]}"; do
    echo "=== Building $component ==="
    "$PROJECT_ROOT/components/$component/scripts/build.sh"
done
echo "All components built successfully"
```

**scripts/serve.sh:**
```bash
#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
exec "$PROJECT_ROOT/components/app/scripts/serve.sh"
```

- [ ] 5.1 Create component build scripts
- [ ] 5.2 Create root delegation scripts
- [ ] 5.3 Make scripts executable: `chmod +x scripts/*.sh components/*/scripts/*.sh`

---

## Phase 6: Remove Old Root Cargo.toml

### Step 6.1: Delete old workspace file
```bash
rm Cargo.toml  # Old root workspace no longer needed
rm -rf crates/ # If empty after moves
```

- [ ] 6.1 Remove old Cargo.toml
- [ ] 6.2 Remove empty crates/ directory

---

## Phase 7: Test Builds

### Step 7.1: Test each component individually
```bash
cd components/core && cargo build --target wasm32-unknown-unknown
cd components/widgets && cargo build --target wasm32-unknown-unknown
cd components/dnd && cargo build --target wasm32-unknown-unknown
cd components/app && cargo build --target wasm32-unknown-unknown
```

### Step 7.2: Test full build
```bash
./scripts/build-all.sh
```

### Step 7.3: Test serve
```bash
./scripts/serve.sh
# Visit http://localhost:1140
```

- [ ] 7.1 Test core build
- [ ] 7.2 Test widgets build
- [ ] 7.3 Test dnd build
- [ ] 7.4 Test app build
- [ ] 7.5 Test build-all.sh
- [ ] 7.6 Test serve.sh

---

## Phase 8: Validate Modularity with sw-checklist

### Step 8.1: Run sw-checklist on each component
```bash
cd components/core && sw-checklist
cd components/widgets && sw-checklist
cd components/dnd && sw-checklist
cd components/app && sw-checklist
```

### Modularity Constraints
- Max 5 crates per component
- Max 7 modules per crate
- Max 7 public functions per module

- [ ] 8.1 sw-checklist passes for core
- [ ] 8.2 sw-checklist passes for widgets
- [ ] 8.3 sw-checklist passes for dnd
- [ ] 8.4 sw-checklist passes for app

---

## Phase 9: Commit the Refactoring

### Step 9.1: Stage all changes
```bash
git add -A
```

### Step 9.2: Create refactoring commit
```bash
git commit -m "refactor: Restructure into multi-component architecture

- Move crates into components/{core,widgets,dnd,app}/
- Each component is an independent Cargo workspace
- Add per-component build.sh scripts
- Root scripts delegate to component scripts
- Enables sw-checklist validation per component
- Maintains max 5 crates per component constraint

File moves tracked with git mv for history preservation."
```

- [ ] 9.1 Stage changes
- [ ] 9.2 Commit refactoring

---

## Phase 10: Update Documentation

### Step 10.1: Update CLAUDE.md with architecture
### Step 10.2: Update README.md
### Step 10.3: Archive documentation from old-tt-rs if needed

- [ ] 10.1 Update CLAUDE.md
- [ ] 10.2 Update README.md
- [ ] 10.3 Verify all docs current

---

## Summary Checklist

### Phase 1: Import History
- [ ] Add old-tt-rs as remote
- [ ] Reset to old-tt-rs main
- [ ] Verify history
- [ ] Remove old-origin remote

### Phase 2: Directory Structure
- [ ] Create component directories
- [ ] git mv crates to components

### Phase 3: Workspace Files
- [ ] core/Cargo.toml
- [ ] widgets/Cargo.toml
- [ ] dnd/Cargo.toml
- [ ] app/Cargo.toml

### Phase 4: Crate Dependencies
- [ ] Update all crate Cargo.toml files

### Phase 5: Build Scripts
- [ ] Component build scripts
- [ ] Root delegation scripts

### Phase 6: Cleanup
- [ ] Remove old Cargo.toml

### Phase 7: Test Builds
- [ ] All components build
- [ ] serve.sh works

### Phase 8: Validate Modularity
- [ ] sw-checklist passes all components

### Phase 9: Commit
- [ ] Commit refactoring

### Phase 10: Documentation
- [ ] All docs updated

---

## Alternative Approaches Considered

### Option A: Cherry-pick commits (Not Recommended)
Cherry-picking loses the original commit SHAs and can cause merge issues.

### Option B: Subtree merge (Overcomplicated)
Git subtrees add complexity without clear benefit for this use case.

### Option C: Start fresh, lose history (Not Recommended)
Loses valuable context about why decisions were made.

### Option D: Fetch + Reset (Recommended - This Plan)
Cleanest approach: import full history, then refactor with tracked moves.
