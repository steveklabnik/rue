# rue

> [!CAUTION]
> Listen, this repo is just for fun. I had it private, but I care more about
> being able to run GitHub Actions to make sure that things are good, so I'm
> open sourcing this repo. Not everything in here is good, or accurate, or
> anything: I'm just messing around. Feel free to take a look but don't look too
> much into this just yet. Someday I'll actually talk about this.


A programming language that starts as a minimal subset of Rust, designed to explore cutting-edge compiler implementation techniques.

**Platform Support**: Linux x86-64 only

## About

Rue is an experimental programming language that begins with a very small, simple subset of Rust syntax but focuses on modern compiler architecture:

- **Incremental compilation** using Salsa with expression-level granularity
- **IDE-first design** with concrete syntax trees that preserve all source information
- **ECS-inspired flat AST** with integer indices for memory efficiency
- **Dual build systems** supporting both Cargo and Buck2
- **x86-64 native code generation** targeting Linux ELF executables

## Current Status

🎉 **Fully Working Compiler** - Complete implementation:
- ✅ Complete lexer for all Rue language tokens
- ✅ Hand-written recursive descent parser with CST
- ✅ Salsa-based incremental compilation pipeline
- ✅ Comprehensive semantic analysis with error reporting
- ✅ x86-64 native code generation with direct ELF output
- ✅ LSP server for IDE integration
- ✅ VS Code extension with syntax highlighting and error detection
- ✅ Multi-crate workspace architecture with dual build systems

## Language Features (v0.1)

The initial version supports a minimal subset:
- **Variables**: All variables are 64-bit integers
- **Arithmetic**: Basic operations (+, -, *, /, %)
- **Control Flow**: if/else statements
- **Functions**: Single parameter, single return value
- **No explicit types**: Everything is implicitly i64

### Example Program

```rue
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    factorial(5)  // Returns 120 as the exit code
}
```

## Building and Running

### Compile Rue Programs
```bash
# Build the compiler
cargo build -p rue

# Compile a rue program to native executable
cargo run -p rue samples/simple.rue

# Run the compiled program
./simple
echo "Exit code: $?"  # Shows the program's return value
```

### With Buck2
```bash
buck2 build //crates/rue:rue
buck2 run //crates/rue:rue samples/simple.rue
./simple
```

### Running Tests
```bash
# Run all tests
cargo test

# Test specific components
cargo test -p rue-lexer
cargo test -p rue-parser
cargo test -p rue-semantic
cargo test -p rue-codegen
```

## IDE Support

Rue includes a complete Language Server Protocol (LSP) implementation for modern IDE integration:

### VS Code Extension
```bash
# Install the VS Code extension
./install-extension.sh

# Then open any .rue file to get:
# - Syntax highlighting
# - Real-time error detection
# - Auto-completion for brackets/quotes
```

### Other Editors
The LSP server works with any LSP-compatible editor:
```bash
# Start the language server
cargo run -p rue-lsp
```

See `crates/rue-lsp/README.md` for integration details.

## Development

- **Architecture**: See [spec.md](./spec.md) for complete language and implementation details
- **IDE Support**: See [CLAUDE.md](./CLAUDE.md) for development guidance
- **Version Control**: This project uses jj (Jujutsu) instead of git

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.