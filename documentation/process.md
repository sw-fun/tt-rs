# tt-rs Development Process

**Full Name**: Cartoon-oriented Talking Programming Application

## Overview

This document describes the development process, conventions, and guidelines for contributing to tt-rs.

**Note**: This is a derived work based on ToonTalk. See the COPYRIGHT file for attribution.

## Development Philosophy

### Incremental Progress

- Make small, focused commits
- Keep the main branch always buildable
- Prefer working software over perfect design
- Refactor as patterns emerge

### Documentation-First

- Update documentation before or alongside code changes
- Document design decisions as they're made
- Keep the README current

### Test-Driven Where Practical

- Write tests for domain logic
- Test edge cases and error conditions
- Integration tests for complex workflows
- Don't over-test trivial code

## Working with AI Assistants

This project uses AI coding assistants (e.g., Claude Code) as development partners. Follow these guidelines:

### For AI Assistants

1. **Read First**: Before making changes, read relevant existing code to understand patterns and conventions.

2. **Respect Context**: Use existing naming conventions, code style, and architectural patterns.

3. **Small Changes**: Make focused, incremental changes rather than large rewrites.

4. **Explain Reasoning**: When making design decisions, explain the rationale.

5. **Test Coverage**: Include tests for new functionality.

6. **Documentation**: Update relevant documentation alongside code changes.

7. **Follow Process**: When a checkpoint is requested, execute the pre-commit sequence rigorously.

### For Human Developers

1. **Clear Instructions**: Provide specific, focused tasks to AI assistants.

2. **Review Carefully**: Always review AI-generated code before committing.

3. **Provide Feedback**: Help AI assistants understand preferences and project conventions.

4. **Iterative Refinement**: Use back-and-forth to refine solutions.

## Code Standards

### Rust Style

Follow the official Rust style guide with these specifics:

```rust
// Use descriptive names
fn calculate_bird_flight_path(from: Point, to: Point) -> BezierPath { ... }

// Prefer explicit types for public APIs
pub fn create_widget(config: WidgetConfig) -> Result<Box<dyn Widget>, Error> { ... }

// Use `Self` in impl blocks
impl Number {
    pub fn new(value: i64) -> Self { ... }
}

// Group imports: std, external crates, internal modules
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use yew::prelude::*;

use crate::domain::Widget;
use crate::presentation::RenderContext;
```

### Error Handling

```rust
// Use thiserror for error types
#[derive(Debug, thiserror::Error)]
pub enum WidgetError {
    #[error("Widget not found: {0}")]
    NotFound(WidgetId),
}

// Use Result for fallible operations
pub fn find_widget(&self, id: WidgetId) -> Result<&dyn Widget, WidgetError> { ... }

// Use Option for optional values
pub fn parent(&self) -> Option<WidgetRef> { ... }

// Propagate errors with ?
pub fn save_workspace(&self) -> Result<(), Error> {
    let json = serde_json::to_string(&self)?;
    storage::save("workspace", &json)?;
    Ok(())
}
```

### Documentation

```rust
/// A widget representing a rational number.
///
/// Numbers support arithmetic operations by dropping one number onto another.
/// The operator determines the operation performed.
///
/// # Examples
///
/// ```
/// let num = Number::new(42);
/// assert_eq!(num.value(), Rational::from(42));
/// ```
pub struct Number {
    value: Rational,
    operator: ArithOperator,
    erased: ErasureLevel,
}
```

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_addition() {
        let a = Number::new(1);
        let b = Number::new(2);
        let result = a.apply_to(&b).unwrap();
        assert_eq!(result.value(), Rational::from(3));
    }

    #[test]
    fn test_number_pattern_matching() {
        let pattern = Number::erased(ErasureLevel::Type);
        let value = Number::new(42);
        assert!(matches!(pattern.matches(&value), MatchResult::Match(_)));
    }
}
```

## Pre-Commit Quality Process

**MANDATORY BEFORE EVERY COMMIT**: No exceptions, no deferrals.

### Step 1: Run Tests
```bash
cargo test
```
- ALL tests must pass
- NO disabled tests allowed

### Step 2: Fix Linting
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
- ZERO clippy warnings
- NEVER use #[allow(...)] to suppress warnings

### Step 3: Format Code
```bash
cargo fmt --all
```

### Step 4: Validate Markdown (if docs changed)
```bash
markdown-checker -f "**/*.md"
```
- All markdown must be ASCII-only for GitHub compatibility

### Step 5: Update Documentation
- Update docs/learnings.md if issues were found
- Update README.md if features added or changed

### Step 6: Commit
```bash
git add -A
git commit -m "Clear, descriptive message

Detailed explanation of changes...

[AI] Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

git push
```

## Git Workflow

### Branching Strategy

```
main
  |
  +-- feature/phase-1-setup
  |     +-- (merged when complete)
  |
  +-- feature/number-widget
  |     +-- (merged when complete)
  |
  +-- fix/bird-animation-bug
        +-- (merged when fixed)
```

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `refactor/description` - Code improvements
- `docs/description` - Documentation updates

### CHANGELOG Workflow

**CRITICAL: Avoid the SHA Loop**

When updating CHANGELOG.md, NEVER try to include the current commit's SHA - this creates an infinite loop (amending changes the SHA).

**Workflow:**

1. Use `<latest>` as placeholder for current commit SHA
2. Replace previous `<latest>` with actual SHA when making the NEXT commit
3. Only ONE `<latest>` should exist at a time

**Example:**
```markdown
## 2025-11-29

- feat: New feature (`<latest>`)           # Current commit - placeholder
- docs: Previous change (`e41f2bf`)        # Previous commit - actual SHA
```

**Rules:**
- NEVER amend commits just to update CHANGELOG SHAs
- NEVER try to include current commit SHA in CHANGELOG
- Replace `<latest>` only when making subsequent commits

### Commit Messages

Follow conventional commits:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code restructuring
- `docs`: Documentation
- `test`: Adding tests
- `chore`: Build/tooling

Examples:
```
feat(number): implement rational arithmetic

Add support for arbitrary precision rational numbers using the
num-rational crate. Numbers can be added, subtracted, multiplied,
and divided without loss of precision.
```

```
fix(robot): correct path resolution for nested boxes

The path resolver was not correctly handling boxes nested more than
2 levels deep. This fix adds proper recursive traversal.

Fixes #42
```

### Push Policy

**Always push immediately after commit**:
- Enables testing on other machines
- Provides backup of work
- Makes work visible for collaboration

## MANDATORY: Live Demo Updates

**CRITICAL: The live demo MUST be updated after EVERY feature or fix that changes behavior.**

The live demo at https://wrightmikea.github.io/tt-rs/ is the primary way users experience tt-rs. Leaving it out of date is unacceptable.

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

# 5. Verify live demo at https://wrightmikea.github.io/tt-rs/
```

### DO NOT:
- Push code changes without updating the live demo
- Leave multiple commits without a corresponding deploy commit
- Skip the deploy step "to do later" - do it NOW
- Consider a feature "done" until the live demo shows it

### Enforcement

If you (AI assistant or human) make feature/fix commits without updating the live demo, you have broken the process. Immediately run:

```bash
./scripts/build-release.sh
git add docs/
git commit -m "deploy: Update live demo"
git push
```

## Build and Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install trunk (dev server)
cargo install trunk

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli
```

### Development Commands

```bash
# Run development server
trunk serve

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt

# Build for production
trunk build --release

# Run specific test
cargo test test_number_addition

# Watch for changes and test
cargo watch -x test
```

### Project Structure

```
tt-rs/
+-- Cargo.toml          # Project manifest
+-- Cargo.lock          # Dependency lock file
+-- index.html          # HTML entry point for WASM
+-- src/
|   +-- lib.rs          # Library root
|   +-- main.rs         # Native entry (testing)
|   +-- domain/         # Business logic
|   +-- execution/      # Robot execution
|   +-- presentation/   # Yew components
|   +-- rendering/      # Graphics
|   +-- storage/        # Persistence
|   +-- audio/          # Speech and sound
|   +-- bindings/       # JS interop
+-- tests/              # Integration tests
+-- assets/             # Static files
+-- documentation/      # Project docs
```

## Quality Standards

### Code Quality

- Zero clippy warnings (enforced with `-D warnings`)
- All code formatted with `cargo fmt`
- Rust 2024 edition idioms
- Inline format arguments: `format!("{name}")` not `format!("{}", name)`

### Test Coverage

- Unit tests for pure logic
- Integration tests for complex workflows
- WASM tests using `wasm-bindgen-test`
- Edge case handling

### Documentation

- Public APIs documented with `///` comments
- Module-level docs with `//!` comments
- README kept up-to-date
- Examples in doc comments where helpful

### Tech Debt Limits

- Maximum 3 TODO comments per file
- Files under 500 lines (prefer 200-300)
- Functions under 50 lines (prefer 10-30)
- Address TODOs within 2 development sessions
- Never commit FIXMEs (fix immediately)

## Release Process

### Versioning

Follow semantic versioning (SemVer):
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

### Release Checklist

1. [ ] Update version in Cargo.toml
2. [ ] Update CHANGELOG.md
3. [ ] Run full test suite
4. [ ] Build production release
5. [ ] Test production build locally
6. [ ] Tag release in git
7. [ ] Deploy to hosting
8. [ ] Update documentation site
9. [ ] Announce release

### Deployment

```bash
# Build optimized release
trunk build --release

# The dist/ directory contains deployable files
ls dist/
# index.html
# tt-rs-[hash].js
# tt-rs-[hash]_bg.wasm
# assets/
```

## License Compliance

### BSD License Requirements

When contributing:
1. Keep existing copyright notices
2. Include license in distributions
3. Don't use project name for endorsement without permission

### Attribution

This project is a derived work:
- Original ToonTalk: Copyright (c) 1992-2009, Ken Kahn
- ToonTalk Reborn (JavaScript): Copyright (c) 2014-2017, Ken Kahn
- tt-rs: Copyright (c) 2025-2026, Michael A Wright

### Third-Party Dependencies

All dependencies must be compatible with BSD license. Check before adding new crates:
- MIT: Compatible
- Apache-2.0: Compatible
- BSD-*: Compatible
- GPL: NOT compatible without permission

## Getting Help

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Yew Documentation](https://yew.rs/docs/getting-started/introduction)
- [WebAssembly](https://webassembly.org/)
- [Original ToonTalk Papers](http://toontalk.com/English/papers.htm)

### Community

- GitHub Issues for project-specific questions
- Rust Discord for Rust/WASM questions
- Yew Discord for Yew-specific questions

## Appendix: Useful Commands

```bash
# Check WASM size
wasm-opt -Os dist/*.wasm -o /dev/null --print-sizes

# Profile compilation
cargo build --release --timings

# Check for outdated dependencies
cargo outdated

# Security audit
cargo audit

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench
```

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
