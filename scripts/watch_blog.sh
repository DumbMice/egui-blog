#!/usr/bin/env bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."

# Watch & serve script for blog app development
# Automatically rebuilds WASM and serves on http://localhost:8766

PORT=8766

# Check if cargo-watch is installed, install it if not
check_cargo_watch() {
    if ! command -v cargo-watch &> /dev/null; then
        echo "cargo-watch not found. Installing..."
        if ! cargo install cargo-watch --locked; then
            echo "Failed to install cargo-watch. Please install manually: cargo install cargo-watch"
            exit 1
        fi
        echo "cargo-watch installed successfully."
    fi
}

check_cargo_watch

# Start HTTP server in background
start_server() {
    echo "Starting HTTP server on port $PORT..."

    # Check if port is already in use
    if lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null ; then
        echo "Port $PORT already in use. Is another server running?"
        echo "Please stop the other server or use a different port."
        exit 1
    fi

    # Small sleep to reduce race condition window
    sleep 0.1

    # Install basic-http-server if needed
    if ! command -v basic-http-server &> /dev/null; then
        echo "Installing basic-http-server..."
        if ! cargo install basic-http-server; then
            echo "Failed to install basic-http-server. Please install manually: cargo install basic-http-server"
            exit 1
        fi
        echo "basic-http-server installed successfully."
    fi

    # Start server in background, capture PID
    cd web_blog
    basic-http-server --addr 0.0.0.0:$PORT . &
    SERVER_PID=$!
    cd ..

    echo "Server started with PID: $SERVER_PID"
    echo "Serving at: http://localhost:$PORT"
}

# Cleanup function
cleanup() {
    # Prevent multiple cleanup calls
    if [ "${CLEANUP_DONE:-false}" = "true" ]; then
        return
    fi
    CLEANUP_DONE=true

    echo "Cleaning up..."
    if [ ! -z "${SERVER_PID:-}" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
        echo "Stopping server (PID: $SERVER_PID)..."
        kill "$SERVER_PID" 2>/dev/null || true
        sleep 1
        if kill -0 "$SERVER_PID" 2>/dev/null; then
            echo "Server not responding to SIGTERM, forcing shutdown..."
            kill -9 "$SERVER_PID" 2>/dev/null || true
        fi
        wait "$SERVER_PID" 2>/dev/null || true
    fi
    echo "Goodbye!"
    exit 0
}

# Trap Ctrl+C and other exits
trap cleanup INT TERM EXIT

start_server

start_watching() {
    # Note: Includes validation and error handling from Task 4
    # Task 5 adds: manual refresh message, --shell flag with timestamps
    echo "Watching for file changes in crates/blog_app..."
    echo "Files will be automatically rebuilt on changes."
    echo "Manual browser refresh required after rebuild."
    echo ""

    # Validate build script exists and is executable
    if [ ! -f "./scripts/build_blog_web.sh" ]; then
        echo "ERROR: Build script not found: ./scripts/build_blog_web.sh"
        echo "Please ensure the build script exists in the scripts directory."
        cleanup
        exit 1
    fi

    if [ ! -x "./scripts/build_blog_web.sh" ]; then
        echo "ERROR: Build script is not executable: ./scripts/build_blog_web.sh"
        echo "Please run: chmod +x ./scripts/build_blog_web.sh"
        cleanup
        exit 1
    fi

    echo "Build script validated successfully."
    echo "Starting file watcher..."
    echo ""

    # Start cargo watch with error handling
    if ! cargo watch \
        -w crates/blog_app \
        --postpone \
        --delay 1 \
        --shell 'echo "===[ $(date +%H:%M:%S) ] Building... ===" && ./scripts/build_blog_web.sh'; then
        echo "ERROR: cargo watch failed to start or crashed."
        echo "Please check if cargo-watch is properly installed."
        echo "You can install it with: cargo install cargo-watch"
        cleanup
        exit 1
    fi

    # If cargo watch exits normally, clean up
    echo "File watcher stopped."
    cleanup
}

start_watching
