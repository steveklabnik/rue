#!/bin/bash
# Install the Rue VS Code extension directly

set -e

echo "🔧 Installing Rue VS Code extension..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "crates/rue-lsp" ]; then
    echo "❌ Error: Please run this script from the rue project root directory"
    exit 1
fi

# Check if Node.js is installed
if ! command -v npm &> /dev/null; then
    echo "❌ Error: Node.js and npm are required but not installed"
    echo "   Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Navigate to extension directory
cd vscode-rue-extension

echo "📦 Installing dependencies..."
npm install

echo "🔨 Compiling extension..."
npm run compile

# Check if vsce is installed
if ! command -v vsce &> /dev/null; then
    echo "📦 Installing vsce (VS Code Extension manager)..."
    npm install -g vsce
fi

echo "📦 Packaging extension..."
vsce package

# Find the generated .vsix file
VSIX_FILE=$(ls -t *.vsix | head -1)

if [ -z "$VSIX_FILE" ]; then
    echo "❌ Error: No .vsix file generated"
    exit 1
fi

echo "🚀 Installing extension: $VSIX_FILE"
code --install-extension "$VSIX_FILE"

echo "✅ Rue VS Code extension installed successfully!"
echo ""
echo "🎉 To use it:"
echo "   1. Open VS Code"
echo "   2. Open the rue project folder"  
echo "   3. Create or open a .rue file"
echo "   4. You should see syntax highlighting and error detection!"
echo ""
echo "📝 Try this valid syntax in samples/simple.rue:"
echo "   fn main() { 42 }"
echo ""
echo "🐛 Or test error detection with:"
echo "   fn main( { 42 }"