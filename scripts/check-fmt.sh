#!/bin/bash

# Script to check formatting before build
set -e

echo "🔍 Checking code formatting..."
cargo fmt -- --check

echo "✅ Formatting check passed!"
echo "🔨 Building project..."
cargo build
