#!/bin/bash

# Build and Run Script for Rust-Go Integration Example

set -e

echo "=== Building Rust Library ==="
cd "$(dirname "$0")"
cargo build --release -p go_lib

echo "=== Running Tests ==="
cargo test -p go_lib

echo "=== Building and Running Go Example ==="
cd go_example
LD_LIBRARY_PATH=../target/release go run main.go

echo "=== Done! ==="
