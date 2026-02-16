# Design: Watch & Serve Script for Blog App Development

**Date**: 2026-02-16
**Author**: Claude Opus 4.6
**Status**: Approved ✅

## Overview
Create a development workflow script that automatically rebuilds the blog WASM when source files change and serves it via a local HTTP server. Equivalent to `npm run dev` for Rust/WASM projects.

## Goals
1. **Automatic rebuilds**: Watch Rust and markdown files, trigger WASM rebuilds
2. **Integrated server**: Start HTTP server in background, serve rebuilt WASM
3. **Single command**: Run everything with `./scripts/watch_blog.sh`
4. **No browser refresh**: Manual F5 required (simpler, more explicit)

## Architecture

### Script: `scripts/watch_blog.sh`
```
┌─────────────────────────────────────────────┐
│ ./scripts/watch_blog.sh                     │
│                                             │
│ 1. Check/install cargo-watch                │
│ 2. Start basic-http-server (background)     │
│ 3. Run cargo watch → build_blog_web.sh      │
│ 4. Cleanup server on exit                   │
└─────────────────────────────────────────────┘
```

### Dependencies
- `cargo-watch`: Global Cargo tool (`cargo install cargo-watch`)
- `basic-http-server`: Already installed by existing scripts

### File Watching Patterns
- `crates/blog_app/**/*.rs`: Rust source changes
- `crates/blog_app/**/*.md`: Markdown post changes
- Excludes: `target/`, `web_blog/`, temporary files

### Server Details
- **Port**: 8766 (matches existing blog server)
- **Directory**: `web_blog/` (serves static files)
- **Background process**: Started with PID tracking
- **Automatic serving**: Static files, WASM rebuilds update immediately

### Build Command
- Uses existing `./scripts/build_blog_web.sh`
- Maintains debug mode default (fast builds)
- Shows build output in terminal

## Error Handling
1. **Port already in use**: Exit gracefully with helpful message
2. **Failed builds**: Don't stop watcher, show error output
3. **Missing dependencies**: Install or prompt to install
4. **Script interruption** (Ctrl+C): Clean up server process

## Usage Flow
```bash
# Terminal 1 (runs everything):
./scripts/watch_blog.sh

# Output:
# Installing cargo-watch (if needed)...
# Starting server on http://localhost:8766
# Watching for file changes...

# Terminal 2 (optional, view logs):
tail -f some_log_file.log

# Browser:
# Navigate to http://localhost:8766
# Edit .rs or .md files → automatic rebuild → F5 to see changes
```

## Integration with Existing Workflow
- **Replaces**: Manual `./scripts/build_blog_web.sh` + `./scripts/start_server_blog.sh`
- **Complements**: Existing scripts still usable individually
- **Port conflict**: Same port 8766, can't run both scripts simultaneously

## Alternatives Considered
1. **Platform-specific watchers** (`inotifywait`/`fswatch`): Less portable
2. **Python watchdog**: Extra dependency
3. **Separate server process**: More complex manual workflow

## Implementation Checklist
- [ ] Create `scripts/watch_blog.sh`
- [ ] Test install/check of `cargo-watch`
- [ ] Implement background server with PID tracking
- [ ] Integrate `cargo watch` with build script
- [ ] Add cleanup handlers (trap EXIT)
- [ ] Test complete workflow
- [ ] Update TODO.md documentation

## Success Criteria
1. Single command starts watcher and server
2. WASM rebuilds automatically on file changes
3. Server continues running with rebuilt files
4. Clean exit on Ctrl+C
5. Clear terminal output showing status