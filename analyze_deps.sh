#!/bin/bash
set -e

echo "=== Analyzing Dependencies for Size ==="

# Build with cargo-bloat to see size breakdown
if command -v cargo-bloat &> /dev/null; then
    echo "Running cargo-bloat..."
    cd crates/blog_app
    cargo bloat --release --target wasm32-unknown-unknown -n 20
else
    echo "cargo-bloat not installed. Install with: cargo install cargo-bloat"
    echo "Listing dependencies count..."
    cargo tree | grep -o "^[a-zA-Z0-9_-]*" | sort | uniq | wc -l
fi

echo ""
echo "=== Checking for unused features ==="
# Check for features that might not be needed
echo "Current features in blog_app/Cargo.toml:"
grep -A 10 "\[features\]" Cargo.toml