#!/bin/bash
# Build script for stspec-cli Rust project

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/stspec-cli"
OUTPUT_DIR="$SCRIPT_DIR"

echo "🔨 Building stspec-cli..."

if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found at $PROJECT_DIR"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed"
    echo "   Install from: https://rustup.rs/"
    exit 1
fi

# Build for current platform (release)
cd "$PROJECT_DIR"
echo "📦 Building release binary..."
cargo build --release

# Determine binary name
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    BINARY_NAME="stspec.exe"
else
    BINARY_NAME="stspec"
fi

BINARY_PATH="$PROJECT_DIR/target/release/$BINARY_NAME"

if [ -f "$BINARY_PATH" ]; then
    # Copy to scripts directory
    cp "$BINARY_PATH" "$OUTPUT_DIR/$BINARY_NAME"
    chmod +x "$OUTPUT_DIR/$BINARY_NAME"

    echo "✅ Build complete!"
    echo "📁 Binary: $OUTPUT_DIR/$BINARY_NAME"

    # Show binary info
    if command -v file &> /dev/null; then
        file "$OUTPUT_DIR/$BINARY_NAME"
    fi

    # Show size
    if [[ "$OSTYPE" == "linux-gnu"* || "$OSTYPE" == "darwin"* ]]; then
        SIZE=$(du -h "$OUTPUT_DIR/$BINARY_NAME" | cut -f1)
        echo "📊 Size: $SIZE"
    fi
else
    echo "❌ Build failed: Binary not found at $BINARY_PATH"
    exit 1
fi
