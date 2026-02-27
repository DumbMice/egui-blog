# Blog App Three-Target Architecture Implementation Plan

## Overview
Restructure the blog application build system to create three distinct targets with a unified development workflow:
1. Native desktop application (optional)
2. WASM binary/library for web deployment  
3. Web server that hosts the WASM app with hot reload (default target)

## Current State Analysis

### ✅ Completed
1. **Math rendering**: Always enabled (removed optional feature)
2. **Conditional compilation**: `main.rs` has proper WASM/native separation
3. **Development binary skeleton**: `blog_dev.rs` exists but will be replaced
4. **Cargo aliases**: Configured but need updating
5. **Build scripts**: Functional but shell-based

### ❌ Issues Identified
1. **Winit compilation failure**: Native builds fail due to missing platform features
2. **WASM build issue**: `arboard` dependency fails without `--no-default-features`
3. **Three-target architecture incomplete**: Need proper separation
4. **Development workflow**: Shell script-based, not Rust-based

## Architecture Design

### Three-Target Structure
```
blog_app crate with:
1. blog_native        - Native desktop app (binary)
2. blog_web_server    - Unified dev/prod server (binary)  
3. WASM Library       - WebAssembly module (library)
```

### Output Directory Structure
```
web_blog/
├── dev/      # Development builds (from wasm32/debug)
└── release/  # Release builds (from wasm32/release + wasm-opt)
```

### Command-Line Interface
```bash
# Default: dev mode with file watching
cargo run --bin blog_web_server

# Serve release builds (no file watching)
cargo run --bin blog_web_server -- --serve-release

# Custom port
cargo run --bin blog_web_server -- --port 9999
cargo run --bin blog_web_server -- --serve-release --port 8080

# Build only
cargo run --bin blog_web_server -- --build-only --serve-release

# Open browser automatically
cargo run --bin blog_web_server -- --open
cargo run --bin blog_web_server -- --serve-release --open
```

## Implementation Phases

### Phase 1: Fix Dependencies (Blocking Issues)
1. Fix winit platform features for native builds
2. Ensure proper WASM build configuration to avoid `arboard` issues

### Phase 2: Create Binary Entry Points
1. Create `src/bin/blog_native.rs` (native desktop app)
2. Create `src/bin/blog_web_server.rs` (unified dev/prod server)
3. Update `src/main.rs` to show help message

### Phase 3: Update Cargo Configuration
1. Update `Cargo.toml` with new binary targets
2. Update `.cargo/config.toml` aliases
3. Set `default-run = "blog_web_server"`

### Phase 4: Implement `blog_web_server` Core Logic
1. Development mode with file watching (`notify` crate)
2. Release mode with optimized builds (`wasm-opt`)
3. HTTP server using `basic-http-server`
4. Tool installation/checking

### Phase 5: Implement Core Functions
1. `build_wasm()` - WASM building with proper feature flags
2. `start_file_watcher()` - File watching for dev mode
3. `start_http_server()` - HTTP server for both modes
4. `ensure_tools_installed()` - Check/install required tools

### Phase 6: Migration from Shell Scripts
1. Map shell script functionality to new binary
2. Update documentation
3. Mark shell scripts as deprecated

### Phase 7: Testing and Validation
1. Test all workflows
2. Verify output structure
3. Test file watching

## Key Technical Decisions

### HTTP Server Choice
Keep `basic-http-server` binary for simplicity. Matches current workflow.

### File Watching
Use `notify` crate (already optional dependency). Pure Rust solution.

### Logging Levels
- **Development mode**: Debug level logging
- **Release mode**: Info level logging

### Backend Choice
Keep current backend (wgpu by default). No `--glow` flag initially.

## Dependencies to Add
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }  # Command-line parsing
```

## Success Criteria

1. ✅ `cargo run` launches dev server on port 8766
2. ✅ File changes trigger WASM rebuild in dev mode
3. ✅ `--serve-release` produces optimized builds in `web_blog/release/`
4. ✅ `--port` flag changes server port
5. ✅ Dev and release builds in separate directories
6. ✅ Native desktop app works (`cargo run --bin blog_native`)
7. ✅ WASM builds successfully
8. ✅ Updated documentation

## Migration Plan

### Shell Script Mapping
- `watch_blog.sh` → `blog_web_server` (dev mode)
- `build_blog_web.sh` → `blog_web_server --serve-release --build-only`
- `start_server_blog.sh` → `blog_web_server --serve-release`
- `setup_web.sh` → `ensure_tools_installed()` function

### Files to Remove
- `src/bin/blog_dev.rs`
- `blog-watch` alias from `.cargo/config.toml`

## Testing Strategy

### Test Commands
```bash
# Test 1: Native desktop
cargo run --bin blog_native

# Test 2: Dev server (default)
cargo run

# Test 3: Release server
cargo run --bin blog_web_server -- --serve-release

# Test 4: Custom port
cargo run --bin blog_web_server -- --port 9999

# Test 5: Build only
cargo run --bin blog_web_server -- --serve-release --build-only
```

### Expected Output Structure
```
web_blog/
├── dev/      # blog_app_bg.wasm, blog_app.js, index.html
└── release/  # Optimized versions
```

## Risk Assessment

### High Risk Areas
1. **Winit dependency resolution** - Test fix first
2. **File watching reliability** - Implement fallback logic
3. **WASM build integration** - Keep shell script logic as reference

### Mitigation Strategies
1. Implement incrementally (fix winit first)
2. Add comprehensive error messages
3. Keep shell scripts during transition period

## Timeline Estimate
- **Phase 1-2**: 2 hours (dependencies + entry points)
- **Phase 3-4**: 3 hours (core implementation)
- **Phase 5-6**: 2 hours (functions + migration)
- **Phase 7**: 1 hour (testing)
- **Total**: 8 hours

## References
- Current build scripts: `scripts/build_blog_web.sh`, `scripts/watch_blog.sh`
- Current entry points: `src/main.rs`, `src/lib.rs`, `src/web.rs`
- Documentation: `AGENTS.md`, `TODO.md`

## Implementation Results

### ✅ Completed Implementation

#### 1. Three-Target Architecture
- **blog_native**: Native desktop binary (`src/bin/blog_native.rs`)
- **blog_web_server**: Unified dev/prod server (`src/bin/blog_web_server.rs`)
- **WASM Library**: Existing `lib.rs` + `web.rs`

#### 2. File Watching & Hot Reload
- Uses `notify` crate with `--features dev`
- Detects changes to `.rs`, `.md`, and files in `posts/` directory
- Ignores generated files (`assets/math/`, `src/math/embedded.rs`, `target/`)
- Triggers automatic WASM rebuild on file changes

#### 3. Error Handling
- **Initial build failure**: Server exits with compiler errors shown
- **Rebuild failure**: Errors logged, file watcher continues, server serves last working version
- **Compiler output captured**: Full Rust compiler error messages displayed
- **User guidance**: Clear messages for fixing errors and retrying

#### 4. Command-Line Interface
```bash
# Development server (hot reload)
cargo run --bin blog_web_server --features dev

# Production server (optimized)
cargo run --bin blog_web_server --features dev -- --serve-release

# Native desktop
cargo run --bin blog_native

# Build only
cargo run --bin blog_web_server --features dev -- --build-only --serve-release

# Options
--port 9999           # Custom port (default: 8766)
--open                # Open browser automatically
--log-level info      # Control verbosity
```

#### 5. Cargo Aliases
```bash
cargo blog          # Development server (hot reload)
cargo blog-release  # Production server (optimized)
cargo blog-native   # Native desktop app
cargo blog-wasm     # Build WASM library only
```

#### 6. Build Outputs
- **Development**: `web_blog/dev/` (debug builds, file watching)
- **Production**: `web_blog/release/` (optimized with wasm-opt, no watching)

### Key Features
1. **Auto tool installation**: `wasm-bindgen`, `basic-http-server` installed if missing
2. **Port conflict detection**: Checks if port is in use before starting server
3. **Math always enabled**: Removed optional feature flag
4. **Winit fixes**: Added platform features for native builds
5. **Legacy scripts deprecated**: Shell scripts replaced with Rust binary

### Testing Results
- ✅ Server starts successfully
- ✅ WASM builds correctly  
- ✅ File watcher detects changes
- ✅ Auto-rebuild triggers on file changes
- ✅ Rebuild completes successfully
- ✅ HTTP server serves updated files
- ✅ Build failures handled gracefully
- ✅ Compiler errors visible in terminal
- ✅ Server continues after rebuild failures

---
*Last updated: 2026-02-27*
*Status: ✅ IMPLEMENTATION COMPLETED*