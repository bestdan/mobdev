#!/bin/bash

# Local linting and formatting check script
# Run this before committing to ensure CI will pass

set -e

echo "🔍 Running format check..."
cargo fmt --check

echo "✅ Format check passed!"
echo ""

echo "🔍 Running clippy linter..."
cargo clippy --all-targets --all-features -- -D warnings

echo "✅ Clippy passed!"
echo ""

echo "🔍 Running tests..."
cargo test

echo "✅ All tests passed!"
echo ""

echo "✨ All checks passed! Ready to commit."
