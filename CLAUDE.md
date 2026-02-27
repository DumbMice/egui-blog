# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains a blog application built with the egui immediate-mode GUI framework. The app compiles to both WebAssembly (for web deployment) and native targets. The primary development focus is the `blog_app` crate located at `crates/blog_app/`.

Key characteristics:
- **Dual-target**: Uses `eframe` for both web (`wasm32-unknown-unknown`) and native compilation
- **Content separation**: Blog posts are stored as Markdown files with YAML frontmatter in `crates/blog_app/posts/`
- **Modular architecture**: Separated into `posts/` (data/models) and `ui/` (presentation) modules
- **Feature-driven development**: Uses subagent-driven development workflow with detailed implementation plans in `docs/plans/`

## Common Development Tasks

### Building and Running

**New Unified Development Workflow (2026-02-27):**

**Development server (hot reload):**
```bash
cargo blog
# or
cargo run --bin blog_web_server --features dev
```
- Starts server with file watching on http://localhost:8766
- Auto-rebuilds WASM on changes to `.rs`, `.md`, `posts/` files
- Shows compiler errors in terminal, continues running after failures
- Auto-installs required tools (wasm-bindgen, basic-http-server)

**Production server (optimized):**
```bash
cargo blog-release
# or
cargo run --bin blog_web_server --features dev -- --serve-release
```
- Optimized builds with `wasm-opt -O2 --fast-math`
- Serves from `web_blog/release/`
- No file watching

**Native desktop app:**
```bash
cargo blog-native
# or
cargo run --bin blog_native
```

**Build only (no server):**
```bash
cargo run --bin blog_web_server --features dev -- --build-only --serve-release
```

**Command options:**
```bash
--port 9999           # Custom port (default: 8766)
--open                # Open browser automatically
--log-level info      # Control verbosity (debug, info, warn, error)
```

**Legacy scripts (deprecated):**
```bash
./scripts/watch_blog.sh           # Use `cargo blog` instead
./scripts/build_blog_web.sh       # Use `cargo blog-release --build-only`
./scripts/start_server_blog.sh    # Use `cargo blog-release`
./scripts/setup_web.sh            # Tools auto-installed by server
```

### Testing

**Run all tests for blog_app:**
```bash
cd crates/blog_app
cargo test
```

**Run specific test:**
```bash
cd crates/blog_app
cargo test test_blog_app_handle_retry
```

**Test with verbose output:**
```bash
cd crates/blog_app
cargo test -- --nocapture
```

### Linting and Code Quality

The workspace uses strict linting rules defined in root `Cargo.toml` and `clippy.toml`.

**Run clippy:**
```bash
cargo clippy -p blog_app
```

**WASM-specific linting:**
```bash
./scripts/clippy_wasm.sh
```

## Architecture Overview

### Entry Points
- `crates/blog_app/src/main.rs`: Native entry point using `eframe::run_native`
- `crates/blog_app/src/web.rs`: WASM bindings using `wasm-bindgen`, exports `WebHandle` struct
- `crates/blog_app/src/lib.rs`: Core `BlogApp` struct implementing `eframe::App`

### Core Modules

**`posts/` module** (`crates/blog_app/src/posts/`):
- `mod.rs`: `PostManager` struct managing blog post collection with `PostManagerState` (Loading/Loaded/Error/Empty)
- `loader.rs`: Markdown file loading with YAML frontmatter parsing, embedded compilation via `include_str!`
- `state.rs`: `PostManagerState` enum for tracking loading state

**`ui/` module** (`crates/blog_app/src/ui/`):
- `layout.rs`: Main UI layout functions (`top_panel`, `side_panel`, `main_content`, `bottom_panel`)
- `components.rs`: Reusable UI components (theme toggle, search bar, post preview, error messages)
- `markdown.rs`: Comprehensive markdown rendering using `pulldown-cmark` with syntax highlighting via `egui_extras::syntax_highlighting`

### Key Data Flow

1. **App initialization**: `BlogApp::default()` → `PostManager::default()` loads embedded posts
2. **UI rendering**: `BlogApp::ui()` calls layout functions, passing `PostManager` and `PostManagerState`
3. **Post loading**: Posts are embedded at compile time via `load_embedded_posts()`; runtime loading available via `load_posts_from_dir()`
4. **State management**: `PostManagerState` drives UI display (loading spinner, error messages, empty state)
5. **User interaction**: Search filtering, post selection, new post creation handled through mutable references

### Markdown Rendering Pipeline

```
Markdown file (YAML frontmatter + content)
    ↓ (compile-time or runtime)
parse_post_content() → BlogPost struct
    ↓
render_markdown() → pulldown-cmark Parser
    ↓
Event stream → egui UI widgets
    ↓
RichText, Hyperlink, Grid, Code blocks with syntax highlighting
```

### Build System

- **WASM target**: `wasm32-unknown-unknown` with `wasm-bindgen` for JavaScript interop
- **Output directory**: `web_blog/` contains generated `.wasm`, `.js`, and `index.html`
- **Development feature**: `dev` feature enables `notify` for file watching
- **Backend selection**: `wgpu` (default) or `glow` graphics backend
- **Conditional compilation**: `#[cfg(target_arch = "wasm32")]` used in `web.rs`; `web_app` feature enables web-specific dependencies
- **Dependencies**: Native-only dependencies under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`, web-only under `[target.'cfg(target_arch = "wasm32")'.dependencies]`

## Important Patterns and Conventions

### State Management
- `PostManager` maintains both post collection (`Vec<BlogPost>`) and loading state (`PostManagerState`)
- `BlogApp` holds a `PostManager` instance and a separate `post_manager_state` clone for UI access
- Error states include retry functionality via `handle_retry()` method

### UI Organization
- Immediate mode egui: UI constructed fresh each frame, no retained widget state
- Panel-based layout: Top (title/search/theme), Side (post list), Central (content), Bottom (footer)
- Component reuse: Common UI patterns extracted to `components.rs`

### Testing Strategy
- Unit tests in same file as implementation (Rust convention)
- Integration tests in `tests/` directory
- TDD approach with failing tests first, especially for state management features

### Development Workflow
1. Edit `.rs` or `.md` files in `crates/blog_app/`
2. `watch_blog.sh` detects changes and rebuilds WASM
3. Refresh browser to see changes (no hot reload)
4. Test with `cargo test` after significant changes

## File Locations

- **Blog posts**: `crates/blog_app/posts/*.md` (Markdown with YAML frontmatter)
- **Implementation plans**: `crates/blog_app/docs/plans/*.md`
- **Build scripts**: `scripts/build_blog_web.sh`, `scripts/start_server_blog.sh`, `scripts/watch_blog.sh`
- **WASM output**: `web_blog/blog_app.js`, `web_blog/blog_app_bg.wasm`
- **Workspace config**: Root `Cargo.toml` defines shared dependencies and lint rules

## Notes for Contributors

- The blog app is part of the larger egui workspace; changes should not break other crates
- Use `cargo test -p blog_app` to test only the blog app crate
- When adding new dependencies, add to `crates/blog_app/Cargo.toml`, not workspace root
- Markdown rendering supports tables, code blocks with syntax highlighting, lists, blockquotes, etc.
- Post loading gracefully handles missing files with `PostManagerState::Error` and retry UI