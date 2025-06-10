# Rue Language Support for VS Code

A VS Code extension that provides language support for Rue through the Rue Language Server.

## Setup Instructions

1. **Install dependencies**:
   ```bash
   cd vscode-rue-extension
   npm install
   ```

2. **Compile the extension**:
   ```bash
   npm run compile
   ```

3. **Install the extension** (choose one method):

   **Method A: Install from folder**
   - Open VS Code
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
   - Type "Extensions: Install from VSIX..."
   - But first, package it: `vsce package` (install vsce with `npm install -g vsce`)
   
   **Method B: Development mode (easier for testing)**
   - Open VS Code
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)  
   - Type "Developer: Install Extension from Location..."
   - Select the `vscode-rue-extension` folder

   **Method C: From VS Code dev instance**
   - Open the `vscode-rue-extension` folder in VS Code
   - Press `F5` to launch a new Extension Development Host window
   - The extension will be active in the new window

## Testing the Extension

1. **Open the rue project** in VS Code (the folder containing the rue compiler)
2. **Create a test file** with a `.rue` extension
3. **Try valid syntax**:
   ```rue
   fn main() {
       42
   }
   ```
4. **Try invalid syntax** to see error reporting:
   ```rue
   fn main( {
       42
   }
   ```

## Features

- **Syntax highlighting** for Rue keywords, numbers, operators
- **Real-time error detection** via LSP integration  
- **Auto-completion** for brackets and quotes
- **Automatic language server startup** when opening .rue files

## Configuration

The extension can be configured in VS Code settings:

- `rue.languageServer.path`: Path to cargo (default: "cargo")
- `rue.languageServer.args`: Arguments for running rue-lsp (default: ["run", "-p", "rue-lsp", "--bin", "rue-lsp"])

## Troubleshooting

- **Language server not starting**: Check that you have the rue project open and cargo is in your PATH
- **No syntax highlighting**: Make sure the file has a `.rue` extension
- **No error detection**: Check the Output panel (View → Output) and select "Rue Language Server" to see logs