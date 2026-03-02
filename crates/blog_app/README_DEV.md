# Blog App Development Workflow

## Quick Start

### Development Server (Hot Reload)
```bash
cargo blog
# or
cargo run --bin blog_web_server --features dev
```
- Starts server on http://localhost:8766
- Watches for file changes (`.rs`, `.md`, `posts/`)
- Auto-rebuilds WASM on changes
- Shows compiler errors in terminal
- Server continues running after rebuild failures

### Production Server (Optimized)
```bash
cargo blog-release
# or
cargo run --bin blog_web_server --features dev -- --serve-release
```
- Optimized builds with `wasm-opt`
- Serves from `web_blog/release/`
- No file watching

### Native Desktop App
```bash
cargo blog-native
# or
cargo run --bin blog_native
```

## Command Options
```bash
--port 9999           # Custom port (default: 8766)
--open                # Open browser automatically
--build-only          # Build only, don't start server
--log-level info      # Control verbosity (debug, info, warn, error)
```

## Development Workflow
1. **Start server**: `cargo blog`
2. **Open browser**: http://localhost:8766 (or use `--open`)
3. **Edit files**: Modify `.rs`, `.md`, or files in `posts/`
4. **Auto-rebuild**: Server detects changes and rebuilds WASM
5. **Refresh browser**: F5 to see changes
6. **Fix errors**: Compiler errors shown in terminal, server continues running

## Key Features
- **Hot reload**: File changes → auto-rebuild → refresh browser
- **Error resilience**: Build failures don't crash server
- **Compiler errors**: Full Rust error output in terminal
- **Auto tool installation**: `wasm-bindgen`, `basic-http-server` installed if missing
- **Port management**: Checks for conflicts, supports custom ports
- **Math always enabled**: Typst math formulas processed at build time

## Build Outputs
- **Development**: `web_blog/dev/` (debug builds, file watching)
- **Production**: `web_blog/release/` (optimized with wasm-opt, no watching)

## Legacy Scripts (Deprecated)
- `./scripts/watch_blog.sh` → Use `cargo blog`
- `./scripts/build_blog_web.sh` → Use `cargo blog-release --build-only`
- `./scripts/start_server_blog.sh` → Use `cargo blog-release`
- `./scripts/setup_web.sh` → Tools auto-installed by server

## Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_blog_app_handle_retry

# Run with verbose output
cargo test -- --nocapture
```

## Themes

The blog app uses Catppuccin themes for consistent aesthetics with GitHub-like markdown rendering:

### Available Themes
- **Light** (☀): Catppuccin Latte theme - light pastel colors
- **Dark** (🌙): Catppuccin Macchiato theme - dark caramel colors

### Theme Switching
- **UI**: Click theme buttons in top panel (☀ 🌙)
- **Display**: Shows as "Light" and "Dark" for simplicity
- **Internally**: Uses Catppuccin Latte and Macchiato color palettes

### GitHub-like Markdown
- Base font size: 16px (matches GitHub)
- Heading sizes follow GitHub standards
- Proper spacing between headings and paragraphs
- Code blocks with syntax highlighting
- Math formula rendering with Typst
- **Strong text**: High contrast colors for accessibility

## Linting & Code Quality
```bash
# Run clippy
cargo clippy -p blog_app

# Format code
cargo fmt -p blog_app
```