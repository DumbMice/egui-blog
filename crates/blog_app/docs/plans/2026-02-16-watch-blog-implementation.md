# Watch Blog Script Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create `scripts/watch_blog.sh` that automatically rebuilds WASM when source files change and serves it via local HTTP server.

**Architecture:** Use `cargo-watch` to monitor Rust/markdown files, trigger existing `build_blog_web.sh`, run `basic-http-server` in background on port 8766, clean up processes on exit.

**Tech Stack:** Bash scripting, `cargo-watch`, `basic-http-server`, `trap` for cleanup, PID management.

---

### Task 1: Create basic watch script skeleton

**Files:**
- Create: `scripts/watch_blog.sh`

**Step 1: Create script with shebang and basic structure**

```bash
#!/usr/bin/env bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."

# Watch & serve script for blog app development
# Automatically rebuilds WASM and serves on http://localhost:8766

echo "Starting blog development watcher..."
echo "Server will run on http://localhost:8766"
echo "Press Ctrl+C to stop"
```

**Step 2: Make script executable**

```bash
chmod +x scripts/watch_blog.sh
```

**Step 3: Test script runs without errors**

```bash
./scripts/watch_blog.sh
```
Expected: Prints startup message, then exits (no functionality yet).

**Step 4: Commit**

```bash
git add scripts/watch_blog.sh
git commit -m "feat: add skeleton watch_blog.sh script"
```

---

### Task 2: Add cargo-watch installation check

**Files:**
- Modify: `scripts/watch_blog.sh:1-20`

**Step 1: Add function to check/install cargo-watch**

```bash
# After shebang and cd, before echo statements
check_cargo_watch() {
    if ! command -v cargo-watch &> /dev/null; then
        echo "cargo-watch not found. Installing..."
        cargo install cargo-watch
        if [ $? -ne 0 ]; then
            echo "Failed to install cargo-watch. Please install manually: cargo install cargo-watch"
            exit 1
        fi
        echo "cargo-watch installed successfully."
    fi
}
```

**Step 2: Call the function before startup message**

```bash
check_cargo_watch
```

**Step 3: Test installation check**

```bash
# If cargo-watch not installed
./scripts/watch_blog.sh
```
Expected: Shows installation message or continues if already installed.

**Step 4: Commit**

```bash
git add scripts/watch_blog.sh
git commit -m "feat: add cargo-watch installation check"
```

---

### Task 3: Start HTTP server in background

**Files:**
- Modify: `scripts/watch_blog.sh:1-40`

**Step 1: Add function to start server with PID tracking**

```bash
# After check_cargo_watch function
start_server() {
    echo "Starting HTTP server on port 8766..."

    # Check if port is already in use
    if lsof -Pi :8766 -sTCP:LISTEN -t >/dev/null ; then
        echo "Port 8766 already in use. Is another server running?"
        echo "Please stop the other server or use a different port."
        exit 1
    fi

    # Install basic-http-server if needed
    if ! command -v basic-http-server &> /dev/null; then
        echo "Installing basic-http-server..."
        cargo install basic-http-server
    fi

    # Start server in background, capture PID
    (cd web_blog && basic-http-server --addr 0.0.0.0:8766 .) &
    SERVER_PID=$!

    echo "Server started with PID: $SERVER_PID"
    echo "Serving at: http://localhost:8766"
}
```

**Step 2: Add cleanup function**

```bash
cleanup() {
    echo "Cleaning up..."
    if [ ! -z "$SERVER_PID" ] && kill -0 $SERVER_PID 2>/dev/null; then
        echo "Stopping server (PID: $SERVER_PID)..."
        kill $SERVER_PID 2>/dev/null || true
        wait $SERVER_PID 2>/dev/null || true
    fi
    echo "Goodbye!"
    exit 0
}

# Trap Ctrl+C and other exits
trap cleanup INT TERM EXIT
```

**Step 3: Call start_server function**

```bash
start_server
```

**Step 4: Test server starts and can be stopped**

```bash
./scripts/watch_blog.sh
```
Expected: Starts server, shows PID, port 8766 responds. Ctrl+C stops server.

**Step 5: Commit**

```bash
git add scripts/watch_blog.sh
git commit -m "feat: start HTTP server with cleanup"
```

---

### Task 4: Add file watching with cargo-watch

**Files:**
- Modify: `scripts/watch_blog.sh:1-60`

**Step 1: Add watch function after server start**

```bash
start_watching() {
    echo "Watching for file changes in crates/blog_app..."
    echo "Files will be automatically rebuilt on changes."
    echo ""

    cargo watch \
        -w crates/blog_app \
        -x "./scripts/build_blog_web.sh" \
        --postpone \
        --debounce 1000

    # If cargo watch exits, clean up
    cleanup
}
```

**Step 2: Call start_watching after start_server**

```bash
start_watching
```

**Step 3: Test file watching triggers build**

```bash
# In one terminal:
./scripts/watch_blog.sh

# In another terminal:
touch crates/blog_app/src/lib.rs
```
Expected: First terminal shows cargo-watch detecting change and running build script.

**Step 4: Commit**

```bash
git add scripts/watch_blog.sh
git commit -m "feat: add file watching with cargo-watch"
```

---

### Task 5: Improve build output and error handling

**Files:**
- Modify: `scripts/watch_blog.sh:1-70`

**Step 1: Modify start_watching to show clearer output**

```bash
start_watching() {
    echo "Watching for file changes in crates/blog_app..."
    echo "Files will be automatically rebuilt on changes."
    echo "Manual browser refresh required after rebuild."
    echo ""

    cargo watch \
        -w crates/blog_app \
        -x "./scripts/build_blog_web.sh" \
        --postpone \
        --debounce 1000 \
        --shell "echo '===[ $(date +%H:%M:%S) ] Building... ===' && {command}" \
        --no-title

    # If cargo watch exits, clean up
    cleanup
}
```

**Step 2: Test improved output**

```bash
./scripts/watch_blog.sh
touch crates/blog_app/src/lib.rs
```
Expected: Shows timestamped build messages.

**Step 3: Commit**

```bash
git add scripts/watch_blog.sh
git commit -m "feat: improve build output with timestamps"
```

---

### Task 6: Update documentation and TODO

**Files:**
- Modify: `crates/blog_app/TODO.md:9`
- Create: `crates/blog_app/README_DEV.md`

**Step 1: Mark file watcher as completed in TODO.md**

```markdown
- [x] Add file watcher for live reload (development)
```

**Step 2: Create development workflow documentation**

```markdown
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
```

**Step 3: Test complete workflow**

```bash
# Terminal 1:
./scripts/watch_blog.sh

# Terminal 2:
curl -s http://localhost:8766 | head -5

# Terminal 1: Press Ctrl+C
```
Expected: Server responds, cleanup runs on exit.

**Step 4: Commit**

```bash
git add crates/blog_app/TODO.md crates/blog_app/README_DEV.md
git commit -m "docs: update TODO and add development workflow guide"
```

---

### Task 7: Final testing and verification

**Files:**
- All modified files

**Step 1: Run full integration test**

```bash
# Start watcher
./scripts/watch_blog.sh &
WATCHER_PID=$!
sleep 3

# Check server is running
curl -s -o /dev/null -w "%{http_code}" http://localhost:8766
# Should return 200

# Trigger rebuild
touch crates/blog_app/src/lib.rs
sleep 5  # Wait for build

# Check WASM file was updated
stat web_blog/blog_app_bg.wasm | grep Modify

# Clean up
kill $WATCHER_PID
wait $WATCHER_PID 2>/dev/null
```

**Step 2: Verify port conflict handling**

```bash
# Start first server
./scripts/watch_blog.sh &
PID1=$!
sleep 2

# Try to start second (should fail)
./scripts/watch_blog.sh
# Should show "Port 8766 already in use"

# Clean up
kill $PID1
```

**Step 3: Commit final version**

```bash
git add scripts/watch_blog.sh
git commit -m "feat: complete watch blog script with testing"
```

---

## Success Criteria
1. Single command starts watcher and server: `./scripts/watch_blog.sh`
2. WASM rebuilds automatically on `.rs` or `.md` file changes
3. Server runs on port 8766, serves rebuilt files
4. Clean exit on Ctrl+C (stops server)
5. Clear terminal output showing status
6. Port conflict detection and helpful error message