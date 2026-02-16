#!/usr/bin/env bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."

# Watch & serve script for blog app development
# Automatically rebuilds WASM and serves on http://localhost:8766

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
    cd web_blog
    basic-http-server --addr 0.0.0.0:8766 . &
    SERVER_PID=$!
    cd ..

    echo "Server started with PID: $SERVER_PID"
    echo "Serving at: http://localhost:8766"
}

# Cleanup function
cleanup() {
    # Prevent multiple cleanup calls
    if [ "${CLEANUP_DONE:-false}" = "true" ]; then
        return
    fi
    CLEANUP_DONE=true

    echo "Cleaning up..."
    if [ ! -z "${SERVER_PID:-}" ] && kill -0 $SERVER_PID 2>/dev/null; then
        echo "Stopping server (PID: $SERVER_PID)..."
        kill $SERVER_PID 2>/dev/null || true
        sleep 1
        if kill -0 $SERVER_PID 2>/dev/null; then
            echo "Server not responding to SIGTERM, forcing shutdown..."
            kill -9 $SERVER_PID 2>/dev/null || true
        fi
        wait $SERVER_PID 2>/dev/null || true
    fi
    echo "Goodbye!"
    exit 0
}

# Trap Ctrl+C and other exits
trap cleanup INT TERM EXIT

start_server

echo ""
echo "Blog development watcher is running..."
echo "Server is serving at: http://localhost:8766"
echo "Press Ctrl+C to stop"
echo ""

# Keep script running until interrupted
# (In Task 4, this will be replaced with start_watching)
# Wait for server process to exit
wait $SERVER_PID 2>/dev/null || true
