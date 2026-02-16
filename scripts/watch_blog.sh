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

echo "Starting blog development watcher..."
echo "Server will run on http://localhost:8766"
echo "Press Ctrl+C to stop"
