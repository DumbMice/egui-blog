#!/usr/bin/env bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."

# Watch & serve script for blog app development
# Automatically rebuilds WASM and serves on http://localhost:8766

echo "Starting blog development watcher..."
echo "Server will run on http://localhost:8766"
echo "Press Ctrl+C to stop"
