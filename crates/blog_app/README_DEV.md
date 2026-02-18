# Blog App Development Workflow

## Quick Start
1. `./scripts/watch_blog.sh` - Starts watcher and server
2. Open http://localhost:8766 in browser
3. Edit `.rs` or `.md` files → automatic rebuild
4. Refresh browser (F5) to see changes

## Manual Workflow (alternative)
- `./scripts/build_blog_web.sh` - Build WASM
- `./scripts/start_server_blog.sh` - Start server

## Features
- Automatic rebuild on file changes
- HTTP server on port 8766
- Clean process cleanup (Ctrl+C)
- Requires: cargo-watch, basic-http-server