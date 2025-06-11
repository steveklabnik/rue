# Rue Language Specification

## Overview

Rue is a programming language that begins as a minimal subset of Rust, designed to explore cutting-edge compiler implementation techniques while maintaining simplicity in its initial version.

## Language Features (v0.1)

### Core Language
- **Variables**: All variables are 64-bit integers, declared with `let`
- **Assignment**: Variables can be reassigned with `=` after declaration
- **Arithmetic**: Basic operations (+, -, *, /, %)
- **Control Flow**: if/else statements and while loops
- **Functions**: Single parameter, single return value
- **Type System**: No explicit type annotations (everything is i64)
- **Error Handling**: Abort on errors (e.g., division by zero)

### Syntax
- Subset of Rust syntax using `fn`, `let`, and curly braces
- File extension: `.rue`
- No type annotations in initial version

### First Milestone
The compiler should be able to compile a factorial function that returns its result as the program's exit code:
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

### Assignment Example
```rue
fn main() {
    let x = 10
    x = x + 5  // Reassign x to 15
    x          // Returns 15 as exit code
}
```

## Compiler Architecture

### Implementation Language
- Written in Rust
- Built with Buck2 as the build system

### Compilation Model
- Direct compilation to x86-64 native code
- Output minimal ELF executables directly
- No traditional compiler/linker split
- Single platform support initially (x86-64)
- Future goals: multi-platform support and cross-compilation

### Incremental Compilation
- Query-based architecture using Salsa
- Similar to rust-analyzer's approach
- Expression-level incremental computation granularity
- Everything incremental by default

### Parser
- Hand-written recursive descent parser
- Produces IDE-friendly concrete syntax tree
- Preserves all tokens and whitespace

### AST Design
- Flat AST inspired by Roslyn's red-green trees
- Integer indices instead of pointers for smaller memory footprint
- ECS-inspired with separate arrays for different node types
- Generational indices for safe node management
- String interning for identifiers

### Code Generation
- Initial implementation: stack-based x86-64 code generation
- Abstract backend interface for future alternative backends (LLVM, Cranelift)
- Direct ELF binary generation

### Error Handling
- Focus on good error messages from the start
- Include source locations
- Start simple but maintain quality

## Standard Library
- Initial version: only supports returning exit codes from main
- The return value of main becomes the program's exit code
- No I/O functionality in v0.1

## Development Infrastructure

### Command Line Interface
- Minimal interface initially
- Basic compilation command

### Testing
- Unit tests for compiler components
- Integration tests with example rue programs
- Continuous Integration from the start

### Version Control
- Open source (eventually)
- MIT/Apache 2.0 dual license

### Documentation
- Maintain language reference as the language grows
- Implementation documentation

## Design Priorities
1. **Fast compilation speed** - primary performance goal
2. **Incremental by default** - all computations should be incremental
3. **IDE-first design** - AST and architecture designed for IDE features
4. **Future extensibility** - designed to grow into a full language

## Future Considerations
- Language Server Protocol (LSP) implementation
- Debug information generation (DWARF)
- I/O and standard library expansion
- Additional control flow (loops)
- More data types
- Multiple function parameters
- Pattern matching
- Memory management strategy

## Build System Integration
- Use Buck2 for building the rue compiler itself
- Explore using Buck2's capabilities for rue's own compilation model
- Investigate sharing dependency graphs between build system and compiler
- Potential for treating individual functions as build targets