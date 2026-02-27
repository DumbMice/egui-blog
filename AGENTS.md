# AGENTS.md

This file provides guidance for agentic coding agents (like Claude Code) working in this egui blog application repository.

## Repository Context

This is an **egui workspace repository** containing multiple crates. The `blog_app` crate is a blog application built on the egui framework. The workspace has dual remotes:
- **origin**: Upstream egui framework (emilk/egui) - for syncing original code
- **blog**: Your blog repository (DumbMice/egui-blog) - for developing your blog application

Branch strategy:
- **main branch**: Clean upstream egui framework (identical to origin/main)
- **blog branch**: Blog application implementation (target: `blog/blog`)

## Project Overview

The blog application is built with the egui immediate-mode GUI framework. The app compiles to both WebAssembly (for web deployment) and native targets. The primary development focus is the `blog_app` crate located at `crates/blog_app/`.

Key characteristics:
- **Dual-target**: Uses `eframe` for both web (`wasm32-unknown-unknown`) and native compilation
- **Content separation**: Blog posts are stored as Markdown files with YAML frontmatter in `crates/blog_app/posts/`
- **Modular architecture**: Separated into `posts/` (data/models) and `ui/` (presentation) modules
- **Feature-driven development**: Uses subagent-driven development workflow with detailed implementation plans in `docs/plans/`

## Build Commands

### Development (Auto-rebuild)
```bash
./scripts/watch_blog.sh
```
Starts a file watcher that rebuilds WASM on changes and serves the app at http://localhost:8766. Requires `cargo-watch` and `basic-http-server`.

### Manual Build and Serve
```bash
./scripts/build_blog_web.sh          # Build WASM only
./scripts/start_server_blog.sh       # Start HTTP server on port 8766
```

### Build Options
```bash
./scripts/build_blog_web.sh --release  # Optimized build with wasm-opt
./scripts/build_blog_web.sh --glow     # Use glow backend instead of wgpu
./scripts/build_blog_web.sh --open     # Build and open browser
```

### Native Run
```bash
cd crates/blog_app
cargo run --release
```

### Math Formula Rendering Requirements
The blog supports Typst math formulas in markdown. To enable math rendering:
1. Install Typst CLI: `cargo install typst` (or download from https://github.com/typst/typst)
2. Build with math feature: `cargo build --features math` or use the provided scripts
3. Formulas are processed at build time and embedded in the binary
4. Math formulas use Typst syntax: `$formula$` (inline) or `$ formula $` (display)
5. Generated math assets (SVGs, manifest) are not committed to git - they are rebuilt on each build

## Testing Commands

### Run All Tests
```bash
cd crates/blog_app
cargo test
```

### Run Specific Test
```bash
cd crates/blog_app
cargo test test_blog_app_handle_retry
```

### Run Test with Verbose Output
```bash
cd crates/blog_app
cargo test -- --nocapture
```

### Run Integration Tests
```bash
cd crates/blog_app
cargo test --test missing_posts
```

## Linting and Code Quality

### Run Clippy
```bash
cargo clippy -p blog_app
```

### WASM-Specific Linting
```bash
./scripts/clippy_wasm.sh
```

### Run All Workspace Tests
```bash
cargo test --workspace
```

### Format Code
```bash
cargo fmt -p blog_app
```

## Code Style Guidelines

### Imports Organization
1. **Standard library imports** first
2. **External crate imports** second
3. **Internal crate imports** third
4. **Module imports** last

Example from `crates/blog_app/src/posts/loader.rs`:
```rust
use std::path::{Path, PathBuf};
use std::fs;

use serde::Deserialize;
use thiserror::Error;

use crate::posts::BlogPost;
```

### Naming Conventions
- **Structs and Enums**: `PascalCase` (e.g., `BlogPost`, `PostManagerState`)
- **Variables and functions**: `snake_case` (e.g., `post_manager`, `load_embedded_posts`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_POSTS`)
- **Module names**: `snake_case` (e.g., `posts`, `ui`)

### Error Handling
- Use `thiserror` crate for error types
- Implement `#[derive(Debug, Error)]` for error enums
- Use `#[error("descriptive message: {0}")]` for error variants
- Propagate errors with `?` operator when appropriate

Example from `crates/blog_app/src/posts/loader.rs`:
```rust
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}
```

### Type Annotations
- Prefer explicit type annotations for public API functions
- Use `impl Trait` for return types when appropriate
- Use `&str` for string slices, `String` for owned strings

### Module Structure
- Keep modules focused and single-responsibility
- Use `mod.rs` for module declarations and re-exports
- Separate data models (`posts/`) from presentation (`ui/`)
- Use submodules for logical grouping (e.g., `ui/components.rs`, `ui/layout.rs`)

### State Management Pattern
The blog app uses a specific state management pattern:
1. `PostManager` maintains both post collection (`Vec<BlogPost>`) and loading state (`PostManagerState`)
2. `BlogApp` holds a `PostManager` instance and a separate `post_manager_state` clone for UI access
3. Error states include retry functionality via `handle_retry()` method

Example state enum from `crates/blog_app/src/posts/state.rs`:
```rust
pub enum PostManagerState {
    Loading,
    Loaded,
    Error(String),
    Empty,
}
```

### UI Organization (egui-specific)
- Immediate mode egui: UI constructed fresh each frame, no retained widget state
- Panel-based layout: Top (title/search/theme), Side (post list), Central (content), Bottom (footer)
- Component reuse: Common UI patterns extracted to `components.rs`
- Use `Context` for theme and visual settings

### Testing Strategy
- Unit tests in same file as implementation (Rust convention)
- Integration tests in `tests/` directory
- TDD approach with failing tests first, especially for state management features
- Test error states and retry functionality

### Conditional Compilation
Use `#[cfg(target_arch = "wasm32")]` for web-specific code:
```rust
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
```

### Dependencies Management
- Add dependencies to `crates/blog_app/Cargo.toml`, not workspace root
- Use workspace dependencies when available (e.g., `eframe = { workspace = true }`)
- Mark web-only dependencies under `[target.'cfg(target_arch = "wasm32")'.dependencies]`
- Mark native-only dependencies under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`

### Documentation
- Use `///` for public API documentation
- Include examples in doc comments when helpful
- Document error conditions and edge cases
- Use `#[allow(unused_imports)]` only when necessary with explanation

## Development Workflow

1. **Edit files**: `.rs` or `.md` files in `crates/blog_app/`
2. **Auto-rebuild**: `watch_blog.sh` detects changes and rebuilds WASM
3. **Test**: Refresh browser to see changes (no hot reload)
4. **Verify**: Run `cargo test` after significant changes
5. **Lint**: Run `cargo clippy -p blog_app` before committing

## Git Workflow

### Sync upstream egui (origin):
```bash
git checkout main
git pull origin main
```

### Develop blog application (blog):
```bash
git checkout blog
# Make changes to blog_app crate
git add .
git commit -m "Your commit message"
git push blog blog
```

### Keep blog branch updated with main:
```bash
git checkout blog
git rebase main
# Resolve any conflicts
git push blog blog --force-with-lease
```

## Important Notes for Agents

### Before Making Changes
1. **Check CLAUDE.md**: Review project overview and conventions
2. **Understand architecture**: Know the data flow between `posts/` and `ui/` modules
3. **Test existing functionality**: Run `cargo test` to ensure nothing is broken
4. **Follow patterns**: Mimic existing code style and organization

### When Adding Features
1. **Check dependencies**: Ensure new dependencies are added to correct section of `Cargo.toml`
2. **Handle both targets**: Consider web (`wasm32`) and native compilation
3. **Update tests**: Add corresponding unit and integration tests
4. **Maintain state pattern**: Follow existing `PostManagerState` pattern for new state

### When Fixing Bugs
1. **Reproduce first**: Create a test that demonstrates the bug
2. **Fix systematically**: Address root cause, not just symptoms
3. **Add regression test**: Ensure bug doesn't reoccur
4. **Update documentation**: If API behavior changes

### Performance Considerations
- WASM binary size is important for web deployment
- Use `--release` flag with `wasm-opt` for production builds
- Consider using `glow` backend for smaller WASM size if `wgpu` is too large
- Profile with `cargo build --release` and check `web_blog/blog_app_bg.wasm` size

## Tools

You have access to a set of tools you can use to answer the user's question.
You can invoke functions by writing a "<｜DSML｜function_calls>" block like the following as part of your reply to the user:
<｜DSML｜function_calls>
<｜DSML｜invoke name="$FUNCTION_NAME">
<｜DSML｜parameter name="$PARAMETER_NAME" string="true|false">$PARAMETER_VALUE

## File Locations Reference

- **Blog posts**: `crates/blog_app/posts/*.md` (Markdown with YAML frontmatter)
- **Implementation plans**: `crates/blog_app/docs/plans/*.md`
- **Build scripts**: `scripts/build_blog_web.sh`, `scripts/start_server_blog.sh`, `scripts/watch_blog.sh`
- **WASM output**: `web_blog/blog_app.js`, `web_blog/blog_app_bg.wasm`
- **Workspace config**: Root `Cargo.toml` defines shared dependencies and lint rules
- **Entry points**: 
  - `crates/blog_app/src/main.rs` (native)
  - `crates/blog_app/src/web.rs` (WASM)
  - `crates/blog_app/src/lib.rs` (core app)

## Common Pitfalls to Avoid

1. **Breaking workspace**: Changes should not break other crates in the egui workspace
2. **Missing conditional compilation**: Web-only code must be wrapped in `#[cfg(target_arch = "wasm32")]`
3. **Ignoring state management**: Always update `PostManagerState` appropriately
4. **Forgetting tests**: New features need corresponding tests
5. **Over-complicating UI**: Keep egui immediate-mode simple and declarative

## Verification Checklist

After making changes, always:
- [ ] Run `cargo test -p blog_app`
- [ ] Run `cargo clippy -p blog_app`
- [ ] Run `cargo fmt -p blog_app`
- [ ] Test web build: `./scripts/build_blog_web.sh`
- [ ] Test native build: `cd crates/blog_app && cargo run --release`
- [ ] Verify no warnings in clippy output
- [ ] Ensure all tests pass