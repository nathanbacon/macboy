#!/bin/bash

# Coverage generation script for macboy GameBoy emulator
# This script generates coverage reports both locally and matches what CI does

set -e

echo "🧹 Cleaning up previous coverage data..."
rm -rf coverage/
find . -name "*.profraw" -delete

echo "📦 Installing required tools..."
rustup component add llvm-tools-preview
cargo install grcov

echo "🔧 Building tests..."
cargo build --tests

echo "🧪 Running tests with coverage instrumentation..."
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw"
cargo test

echo "📊 Generating coverage reports..."
mkdir -p coverage

# Generate HTML report (main report for browsing)
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/html/

# Generate Cobertura XML (for CI/CD integration)
grcov . --binary-path ./target/debug/deps/ -s . -t cobertura --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/cobertura.xml

# Generate LCOV (for some tools)
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/lcov.info

echo "📈 Extracting coverage percentage..."
COVERAGE=$(grep -o 'line-rate="[0-9.]*"' coverage/cobertura.xml | head -1 | grep -o '[0-9.]*' | awk '{printf "%.1f", $1 * 100}')

echo "✅ Coverage generation complete!"
echo "📊 Overall coverage: ${COVERAGE}%"
echo "🔍 View detailed report: open coverage/html/index.html"
echo ""
echo "Files generated:"
echo "  📁 coverage/html/index.html    - Interactive HTML report"
echo "  📄 coverage/cobertura.xml      - Cobertura XML format"
echo "  📄 coverage/lcov.info          - LCOV format"
echo ""

# Open the HTML report if on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🚀 Opening coverage report in browser..."
    open coverage/html/index.html
fi
