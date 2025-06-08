# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rue is a programming language that starts as a minimal subset of Rust, designed to explore cutting-edge compiler implementation techniques. The compiler is written in Rust and uses Buck2 as its build system.

Key features:
- Compiles to x86-64 native code (ELF executables)
- Incremental compilation using Salsa
- ECS-inspired flat AST with integer indices
- All variables are 64-bit integers
- Supports functions, arithmetic, and if/else

For complete language and implementation details, see [spec.md](./spec.md).

## Development Commands

### Building
- `buck2 build //crates/rue:rue` - Build the main rue compiler
- `buck2 build //crates/...` - Build all crates

### Testing  
- `buck2 test //crates/...` - Run all tests
- `buck2 test //crates/rue-lexer:test` - Run lexer tests
- `buck2 test //crates/rue-parser:test` - Run parser tests
- `buck2 test //crates/rue-semantic:test` - Run semantic analysis tests
- `buck2 test //crates/rue-codegen:test` - Run code generation tests

### Compiling and Running Programs
- `buck2 run //crates/rue:rue simple.rue` - Compile simple.rue to executable
- `buck2 run //crates/rue:rue <source.rue>` - Compile any rue source file
- `./simple` - Run the compiled executable (after compilation)

### Example Programs
- `simple.rue` - Basic program that returns 42
- Test compilation: `buck2 run //crates/rue:rue simple.rue && ./simple && echo "Exit code: $?"`

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

## Architecture

### Compiler Pipeline
- **Lexer** → **Parser** → **Semantic Analysis** → **Code Generation** → **Assembly** → **ELF Generation**
- Hand-written recursive descent parser produces concrete syntax trees
- Salsa-based incremental compilation for semantic analysis
- Stack-based expression evaluation with x86-64 instruction generation
- Two-pass assembler with symbol resolution and relocation
- Direct ELF executable generation (no external linker required)

### Key Design Decisions
- IDE-first design with concrete syntax trees
- Expression-level incremental computation
- Separate arrays for different AST node types (ECS-inspired)
- No traditional compiler/linker split - generates executables directly
- System V AMD64 ABI compliance for C library compatibility
- Stack-based evaluation prioritizing correctness over optimization

### CI/CD Notes
- The rue compiler requires a source file argument - it cannot run with no arguments
- CI tests should use: `buck2 run //crates/rue:rue simple.rue` 
- Integration tests should compile and run programs to verify correctness
- Always test both buck2 and cargo build systems for consistency

### Version Control
This project uses jj (Jujutsu) instead of git. Common commands:
- `jj status` - see current changes
- `jj commit -m "message"` - commit changes
- `jj log` - view commit history