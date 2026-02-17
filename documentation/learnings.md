# Development Learnings

This document captures solutions to issues encountered during development.

## Clippy False Positive: Dead Code Warnings for WASM Projects

**Problem:** Running `cargo clippy --all-targets --all-features -- -D warnings` reports false positive `dead_code` warnings for methods that are used in tests or will be used in WASM runtime.

**Cause:** When clippy runs without the wasm32 target, it doesn't see cfg-gated code as "used", leading to false positives.

**Solution:** Run clippy with the WASM target:

```bash
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
```

**Reference:** [wasm-bindgen issue #1297](https://github.com/rustwasm/wasm-bindgen/issues/1297)

---

## GitHub Pages Deployment for Trunk/WASM Projects

**Problem:** Live demo shows blank screen with 404 errors for `.wasm`, `.js`, and `.css` files.

**Cause:** By default, `trunk build` generates asset paths relative to `/` (root). GitHub Pages serves project sites at `/<repo-name>/`, so paths like `/tt-rs-xxx.wasm` resolve to `https://username.github.io/tt-rs-xxx.wasm` instead of `https://username.github.io/tt-rs/tt-rs-xxx.wasm`.

**Solution:** Use the `--public-url` flag when building for GitHub Pages:

```bash
trunk build --release --public-url /tt-rs/
```

This prefixes all asset paths with `/tt-rs/`, so they resolve correctly.

**Deployment steps:**

1. Build with correct public URL:
   ```bash
   trunk build --release --public-url /tt-rs/
   ```

2. Copy dist to docs:
   ```bash
   rm -rf docs/*
   cp -r dist/* docs/
   touch docs/.nojekyll
   ```

3. Commit and push

4. Enable GitHub Pages in repository settings:
   - Source: Deploy from branch
   - Branch: main
   - Folder: /docs

---

## Dedicated Development Port

**Problem:** Port conflicts when developing multiple projects simultaneously.

**Solution:** Assign a dedicated port per project.

For tt-rs, use port **1140**:

```bash
trunk serve --port 1140
```

To kill any existing server on that port:

```bash
lsof -ti:1140 | xargs kill -9 2>/dev/null
```

---

## Rust Edition 2024 Benefits

**Decision:** Use Rust edition 2024 (latest stable) instead of older editions.

**Benefits:**

1. **Improved Linter Checking:** Edition 2024 enables more precise lint detection and clearer error messages
2. **Language Ergonomics:** New features like improved async handling, better pattern matching exhaustiveness
3. **Future Compatibility:** Code written for 2024 edition follows current Rust idioms and best practices
4. **Stricter Defaults:** Better default warnings catch more potential issues at compile time

**Note:** Some tools (like trunk) may not fully support edition 2024 yet. If encountering issues, check tool versions and update as needed rather than downgrading the edition.

---

## sw-checklist Modularity Standards

**Tool:** `sw-checklist` enforces modularity to prevent tech debt accumulation.

**Thresholds:**

| Metric | Warning | Fail |
|--------|---------|------|
| Lines per function | > 25 | > 50 |
| Functions per module | > 4 | > 7 |
| Modules per crate | > 4 | > 7 |
| Crates per project | > 4 | > 7 |

**Rationale:** Leave room for future features. When you're at the threshold, the next feature forces a refactor.

### Workspace Refactoring Strategy

When a single-crate project exceeds thresholds, restructure as a Rust workspace:

1. **Create workspace root `Cargo.toml`:**
   ```toml
   [workspace]
   resolver = "2"
   members = ["crates/crate-name", ...]

   [workspace.package]
   version = "0.1.0"
   edition = "2024"
   ```

2. **Split by responsibility:**
   - `*-core`: Core traits and types (no dependencies)
   - `*-number`, `*-text`, etc.: Domain widgets
   - `*-ui`: Presentation components
   - `*-app`: WASM entry point (ties everything together)

3. **Use workspace inheritance:**
   ```toml
   [package]
   name = "crate-name"
   version.workspace = true
   edition.workspace = true
   ```

### Function Splitting Strategy

When a module has too many functions:

1. **Group by cohesion:** Accessors, constructors, operations, trait implementations
2. **Create focused modules:** `accessors.rs`, `arithmetic.rs`, `widget_impl.rs`
3. **Keep public API simple:** Re-export from `lib.rs`

### Example Split (Number Widget)

Before: `number.rs` with 18+ functions

After:
```
tt-rs-number/src/
├── lib.rs          # Re-exports public API
├── number.rs       # Struct and constructors (4 functions)
├── accessors.rs    # Getters and apply_to (7 functions)
├── arithmetic.rs   # Pure math functions (6 functions)
├── operator.rs     # ArithOperator enum (2 functions)
└── widget_impl.rs  # Widget trait impl (7 functions)
```

---

## Trunk with Rust Workspaces

**Problem:** `trunk serve` reports "could not find the root package of the target crate" with workspace projects.

**Cause:** Trunk needs explicit configuration when the Cargo.toml at project root is a workspace, not a package.

**Solution:** In `index.html`, specify the exact crate path:

```html
<link data-trunk rel="rust" href="crates/tt-rs-app/Cargo.toml" data-wasm-opt="z">
```

If trunk still fails, create a `Trunk.toml` at project root:

```toml
[build]
target = "index.html"

[[proxy]]
backend = "http://localhost:1140/"
```

**Solution:** Keep `index.html`, `favicon.ico`, and `assets/` within the app crate directory (`crates/tt-rs-app/`). Run trunk from that crate directory. Use project scripts to automate this.

---

## Project Scripts Best Practice

**Principle:** Always use project scripts for building and serving. Repeated use prevents regressions and makes builds repeatable and reproducible.

**Benefits:**

1. **Reproducibility:** Same commands run every time, no manual errors
2. **Documentation:** Scripts self-document the build process
3. **Regression Prevention:** Tested scripts catch issues early
4. **Onboarding:** New developers can build immediately

**Standard Scripts:**

```
scripts/
├── serve.sh        # Development server
├── build-all.sh    # Build + test + clippy + fmt
└── build-release.sh # Production build for deployment
```

**serve.sh** - Development server:
```bash
#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_CRATE="$PROJECT_ROOT/crates/tt-rs-app"

lsof -ti:1140 | xargs kill -9 2>/dev/null || true
cd "$APP_CRATE"
trunk serve --port 1140
```

**build-all.sh** - Quality checks:
```bash
#!/bin/bash
set -e
cd "$(dirname "$0")/.."

cargo build --target wasm32-unknown-unknown
cargo test
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
cargo fmt --all -- --check
```

**Usage in development:**
```bash
# Start dev server
./scripts/serve.sh

# Run all quality checks
./scripts/build-all.sh

# Build for deployment
./scripts/build-release.sh
```

---

## Asset Paths for GitHub Pages Subdirectory Deployment

**Problem:** Images and assets fail to load on GitHub Pages when the site is deployed to a subdirectory (e.g., `https://username.github.io/repo-name/`).

**Root Cause:** Absolute paths with leading `/` resolve to the domain root, not the subdirectory.

| Path Format | Resolves To | Works on GH Pages? |
|-------------|-------------|-------------------|
| `/images/foo.svg` | `https://username.github.io/images/foo.svg` | NO |
| `images/foo.svg` | `https://username.github.io/repo-name/images/foo.svg` | YES |

**Solution:** Always use **relative paths without leading `/`** for assets in Rust/Yew code:

```rust
// WRONG - breaks on GitHub Pages subdirectory
<img src="/images/tt-bird.svg" />

// CORRECT - works everywhere
<img src="images/tt-bird.svg" />
```

**Why this happens:**
- During development with `trunk serve`, the app is served from `/`, so both formats work
- On GitHub Pages project sites, the app is served from `/repo-name/`, breaking absolute paths

**Prevention:** The `build-release.sh` script validates that no absolute asset paths exist in the built HTML before deployment.

---

## Box Keyboard Resize Regression

**Problem:** Dragging a box and pressing a number key should create a copy of the box with the specified number of holes, but the original box was being removed instead of kept as a prototype.

**Root Cause:** The `handle_box_drop` function was removing the original box when creating a new box with the pending number of holes, instead of keeping the original as a prototype.

**Solution:** When a keyboard number is pressed while dragging a box:
1. Keep the original box in place (it acts as a prototype)
2. Create a new box with the specified number of holes at the drop location

**Code Fix:**
```rust
// In box_ops.rs handle_box_drop()
if let Some(num_holes) = pending {
    // Create a new box with requested number of holes (original stays as prototype)
    create_new_box(state, num_holes, event);
    return true;
}
```

**Key Points:**
- The original box stays in its original position
- A new box is created at the drop location with the specified number of holes
- This matches the behavior of copy sources (stacks) where clicking creates a copy

---

## Z-Index for Widget Stacking

**Problem:** Widgets created by clicking on copy source stacks appeared "under" the stacks, making them impossible to drag.

**Root Cause:** Inline z-index values in Rust code were overriding the CSS class-based z-index hierarchy.

**Solution:** All z-index values are defined in CSS only, NOT in Rust code. The CSS uses CSS custom properties for maintainability:

```css
:root {
    --z-copy-source-behind: 1;   /* Copy source stacks (behind everything) */
    --z-containers: 10;          /* Boxes */
    --z-values: 20;              /* Numbers, Text */
    --z-comparison: 30;          /* Scales */
    --z-agents: 40;              /* Robot, Bird, Nest */
    --z-tools: 50;               /* Vacuum, Wand */
    --z-tooltip-hover: 100;      /* Tooltips */
    --z-dragging: 200;           /* Widget being dragged */
}
```

**Key Principle:** z-index must be set in CSS classes, NOT in Rust inline styles. This allows:
- Different widget types to have appropriate stacking order
- Consistent behavior regardless of DOM creation order
- Easy maintenance via CSS variables

**Validation:** Run `./scripts/check-zindex.sh` to verify:
1. CSS variables are defined correctly
2. No inline z-index in Rust format strings

**Pre-commit:** The z-index check is included in `./scripts/pre-commit-checks.sh` for UI changes.

---

## CRITICAL: Live Demo Must Always Be Current

**Problem:** Feature commits were pushed without updating the live demo at https://sw-fun.github.io/tt-rs/, leaving users with an outdated version.

**Root Cause:** The deployment step was documented but not enforced as mandatory in the workflow.

**Solution:** Update the live demo after EVERY feature or fix commit. No exceptions.

**Complete Feature/Fix Workflow:**

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

**Key Rules:**
- NEVER push code changes without updating the live demo
- NEVER consider a feature "done" until the live demo shows it
- If you forgot to update the demo, fix it immediately

**Enforcement:** If feature commits exist without a corresponding deploy commit, the process has been broken. Fix immediately by running `./scripts/build-release.sh` and committing docs/.

---

## CHANGELOG Workflow: Avoiding Commit SHA Loops

**Problem:** Updating a CHANGELOG entry with its own commit SHA creates an infinite loop - each amend changes the SHA, requiring another update.

**Solution:** Use `<latest>` as a placeholder for the current commit's SHA.

**Workflow:**

1. **When adding a new entry:** Use `<latest>` as the SHA placeholder:
   ```markdown
   - feat: New feature description (`<latest>`)
   ```

2. **When committing:** Commit normally. The `<latest>` placeholder stays in the file.

3. **On the NEXT commit:** Replace the previous `<latest>` with its actual SHA:
   ```markdown
   - feat: Another new feature (`<latest>`)
   - feat: Previous feature (`abc1234`)  # <-- was <latest>, now has real SHA
   ```

**Key Rules:**
- NEVER try to put the current commit's SHA in the CHANGELOG - it's impossible
- NEVER amend commits just to update CHANGELOG SHAs - this changes the SHA
- Only ONE `<latest>` should exist at a time (the most recent entry)
- Replace `<latest>` with actual SHA when making the NEXT commit

**Example CHANGELOG:**
```markdown
## 2025-11-29

- feat: Bird/Nest messaging (`<latest>`)     # Current commit - placeholder
- docs: Update documentation (`e41f2bf`)     # Previous commit - real SHA
- fix: Asset path fix (`4345422`)            # Older commit - real SHA
```

---
