#!/usr/bin/env bash
cd workspace

set -euo pipefail

echo "Running cargo fmt check"
cargo fmt --all -- --check

echo "Running cargo clippy..."
cargo clippy --workspace --all-targets -- -D warnings

echo "Running cargo test..."
cargo test --workspace --all-features

echo "All checks passed."

cd ..