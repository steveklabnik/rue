# Contributing to Rue

This guide is for human developers contributing to the Rue programming language. For AI assistant guidance, see [CLAUDE.md](./CLAUDE.md).

## Project Overview

Rue is a programming language that starts as a minimal subset of Rust, designed to explore cutting-edge compiler implementation techniques. The compiler is written in Rust and uses Buck2 as its build system.

**Platform Support**: Linux x86-64 only (generates ELF executables)

Key features:
- Compiles to x86-64 native code (ELF executables)
- Incremental compilation using Salsa
- ECS-inspired flat AST with integer indices
- All variables are 64-bit integers
- Supports functions, arithmetic, if/else, while loops, and assignments

For complete language specification, see [docs/spec.md](./docs/spec.md).
For implementation details, see [docs/implementation.md](./docs/implementation.md).

## Development Commands

### Building
- `buck2 build //crates/rue:rue` - Build the main rue compiler
- `buck2 build //crates/...` - Build all crates

### Testing  
- `buck2 test //crates/rue-lexer:test` - Run lexer tests
- `buck2 test //crates/rue-parser:test` - Run parser tests
- `buck2 test //crates/rue-semantic:test` - Run semantic analysis tests
- `buck2 test //crates/rue-codegen:test` - Run code generation tests
- `buck2 test //crates/rue:test` - Run basic sample validation tests
- `cargo test -p rue` - Run end-to-end integration tests (compile and execute samples)
- `cargo test -p rue-lsp` - Run LSP server tests (Buck2 has third-party dependency compilation issues)
- `cargo test` - Run all tests across all packages

**Running Specific Test Subsets:**
- `cargo test -p rue-lexer test_name` - Run specific lexer test
- `cargo test -p rue-parser parse_` - Run all parser tests matching pattern
- `buck2 test //crates/rue-lexer:test -- --filter keyword` - Filter Buck2 tests by keyword
- `cargo test integration_tests` - Run only integration tests
- `cargo test -- --nocapture` - Show println! output during tests

### Compiling and Running Programs
- `buck2 run //crates/rue:rue samples/simple.rue` - Compile simple.rue to executable
- `buck2 run //crates/rue:rue <source.rue>` - Compile any rue source file
- `./simple` - Run the compiled executable (after compilation)

### Example Programs
- `samples/simple.rue` - Basic program that returns 42
- `samples/factorial.rue` - Recursive factorial function (returns 120 for factorial(5))
- `samples/simple_assignment.rue` - Basic assignment demonstration (returns 100)
- `samples/countdown.rue` - While loop demonstration counting down from 10
- Test compilation: `buck2 run //crates/rue:rue samples/simple.rue; ./simple; echo "Exit code: $?"`

### LSP and IDE Support
- `cargo run -p rue-lsp` - Start the Language Server Protocol server
- `./install-extension.sh` - Install VS Code extension for syntax highlighting and error detection
- The LSP provides real-time syntax error detection in any compatible editor
- **Note**: LSP currently only works with Cargo due to Buck2 third-party dependency compilation issues

### Managing Third-Party Dependencies
- `reindeer update` - Update Cargo.lock with new dependencies  
- `reindeer vendor` - Vendor crates needed for Buck2 build
- `reindeer buckify` - Generate Buck build rules for third-party Cargo packages
- When adding new dependencies to any crate, run the above commands to update Buck2 support
- **Current limitation**: Some third-party dependencies have compilation issues with Buck2 (e.g., serde_json, auto_impl)
- Use `fixups/<crate>/fixups.toml` to configure build script behavior for problematic dependencies

### Debugging Compiled Programs
When compiled programs crash or behave unexpectedly:

- `gdb ./simple` - Debug executable with gdb
  - `run` - run the program
  - `bt` - show backtrace on crash
  - `disas` - disassemble current function
  - `info registers` - show register values

- `hexdump -C simple` - Examine binary content
- `readelf -h simple` - Verify ELF header
- `objdump -d simple` - Disassemble machine code

**Common Issues:**
- Segmentation faults often indicate incorrect instruction sizes in assembler
- Wrong exit codes suggest incorrect System V ABI implementation
- Use `echo $?` after running to check exit code

### Debugging the Compiler Itself
When the rue compiler crashes, fails to compile, or produces incorrect output:

**Compiler Crashes:**
- `RUST_BACKTRACE=1 buck2 run //crates/rue:rue samples/simple.rue` - Get Rust backtrace
- `RUST_BACKTRACE=full buck2 run //crates/rue:rue samples/simple.rue` - Get full backtrace with line numbers
- `gdb --args ./target/debug/rue samples/simple.rue` - Debug with gdb if using cargo build

**Compilation Issues:**
- `buck2 run //crates/rue:rue samples/simple.rue -- --verbose` - Enable verbose output (if supported)
- Add `dbg!()` or `println!()` statements in compiler source for tracing
- Check lexer output by examining `rue-lexer` tests
- Check parser output by examining `rue-parser` tests

**Code Generation Issues:**
- Compare generated assembly against working examples
- Verify ELF structure: `readelf -a output_file`
- Check symbol table: `nm output_file`
- Disassemble generated code: `objdump -d output_file`

## Architecture Constraints

### Platform and ABI Requirements
- **Linux x86-64 only** - generates ELF executables
- **System V AMD64 ABI compliance** - for C library compatibility
- **Stack-based evaluation** - prioritizes correctness over optimization
- **Direct ELF generation** - no external linker dependency
- **IDE-first design** - concrete syntax trees for better tooling support

### CI/CD Notes
- The rue compiler requires a source file argument - it cannot run with no arguments
- CI tests should use: `buck2 run //crates/rue:rue samples/simple.rue` 
- Integration tests should compile and run programs to verify correctness
- Always test both buck2 and cargo build systems for consistency

## Reindeer and Buck2 Dependency Management

Reindeer is used to convert Cargo.toml dependencies to Buck2 build files. Key commands and workflows:

**Basic Usage:**
- `reindeer buckify` - Generate Buck2 build files from Cargo dependencies
- Must be run after any changes to Cargo.toml or fixups/
- Warnings about build scripts indicate missing fixups

**Fixup Management:**
When adding new dependencies or getting build script warnings from `reindeer buckify`, check these repositories for existing fixup examples:
- https://github.com/dtolnay/buck2-rustc-bootstrap/tree/master/fixups - Official Rust bootstrap fixups
- https://github.com/gilescope/buck2-fixups/tree/main/fixups - Community-maintained fixups

**Important:** Always run `reindeer buckify` again after creating or modifying fixups to regenerate build files.

**Common fixup patterns:**
- `buildscript.run = true/false` - Whether to run the crate's build script
- `cargo_env = true` - Provide Cargo environment variables (e.g., CARGO_PKG_NAME) to build scripts

**Workflow for new dependencies:**
1. Add dependency to Cargo.toml
2. Run `reindeer buckify` - note any warnings
3. Create fixups/ directories and fixups.toml files for warned crates
4. Run `reindeer buckify` again to apply fixups
5. Test with `buck2 test //crates/...`

