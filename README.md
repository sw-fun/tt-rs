# tt-rs

**Cartoon-oriented Talking Programming Application**

A modern Rust/WebAssembly reimplementation of ToonTalk, an interactive visual programming environment for learning computational thinking through animated metaphors.

![tt-rs Screenshot](images/screenshot-1771358169.png)

**[Live Demo](https://sw-fun.github.io/tt-rs/)**

## About

tt-rs brings programming-by-demonstration to modern web browsers. Users create programs by training robots that watch and learn from their actions. The system uses concrete metaphors - birds carry messages, boxes hold data, scales compare values - making abstract computing concepts tangible and accessible.

This is a derived work based on ToonTalk, originally created by Ken Kahn.

## Related Projects & Resources

### ToonTalk Origins
- [ToonTalk Website](https://www.toontalk.com/) - Official ToonTalk website by Ken Kahn
- [ToonTalk Reborn (JavaScript)](https://github.com/ToonTalk/ToonTalk) - Ken Kahn's 2014-2017 JavaScript/jQuery reimplementation
- [ToonTalk on Wikipedia](https://en.wikipedia.org/wiki/ToonTalk) - Encyclopedia entry about ToonTalk

## Status

Active development with working demo. Current features:

- **Widgets**: Numbers (rational arithmetic), Boxes (configurable holes), Text, Scales (comparison), Robot, Bird, Nest
- **Tools**: Wand (copy), Vacuum (remove)
- **Interaction**: Drag-and-drop, tooltips, contextual help panel
- **User Levels**: tt1 (basics), tt2 (messaging) with level-specific help

## Documentation

- [Architecture](documentation/architecture.md) - System design and module structure
- [Product Requirements](documentation/prd.md) - Features, requirements, and user stories
- [Design](documentation/design.md) - Technical design decisions
- [Implementation Plan](documentation/plan.md) - Phased development roadmap
- [Development Process](documentation/process.md) - Contributing guidelines

### For AI Coding Agents

- [AI Agent Instructions](documentation/ai_agent_instructions.md) - Guidelines for AI development partners
- [AI Process](documentation/ai_process.md) - Detailed workflow for AI agents
- [Development Tools](documentation/tools.md) - Recommended tooling

## Technology

- **Rust** - Core logic compiled to WebAssembly
- **Yew** - Reactive UI framework
- **Three.js** - 3D graphics (planned)
- **SVG/CSS** - 2D graphics and animations

## Building

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install trunk

# Development server (port 1140)
./scripts/serve.sh

# Build + test + clippy + fmt
./scripts/build-all.sh

# Production build for GitHub Pages
./scripts/build-release.sh
```

## License

BSD 3-Clause License

See [COPYRIGHT](COPYRIGHT) and [LICENSE](LICENSE) for full attribution and terms.

### Attribution

- Original ToonTalk (C++): Copyright (c) 1992-2009, Ken Kahn
- ToonTalk Reborn (JavaScript): Copyright (c) 2014-2017, Ken Kahn
- tt-rs (Rust/WebAssembly): Copyright (c) 2025-2026, Michael A Wright
