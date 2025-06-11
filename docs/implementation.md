# Rue Compiler Implementation

## Overview

The Rue compiler is written in Rust and implements a complete compilation pipeline from source code to native x86-64 ELF executables. This document describes the implementation architecture and design decisions.

## Compilation Pipeline

The compiler follows this pipeline:
**Lexer** → **Parser** → **Semantic Analysis** → **Code Generation** → **Assembly** → **ELF Generation**

## Implementation Language & Build System

- **Language**: Rust
- **Build System**: Buck2 (with Cargo support for LSP and some tests)
- **Platform Support**: Linux x86-64 only (generates ELF executables)

## Architecture Components

### Lexer (`rue-lexer`)
- Hand-written lexical analyzer
- Converts source text into tokens
- Preserves source location information for error reporting

### Parser (`rue-parser`)
- Hand-written recursive descent parser
- Produces IDE-friendly concrete syntax tree (CST)
- Preserves all tokens and whitespace for LSP features
- Error recovery for better IDE experience

### Abstract Syntax Tree (`rue-ast`)

#### Design Philosophy
- **Flat AST**: Inspired by Roslyn's red-green trees and ECS architecture
- **Integer indices**: Instead of pointers for smaller memory footprint and better cache locality
- **Separate arrays**: Different node types stored in separate arrays (ECS-inspired)
- **Generational indices**: Safe node management without lifetime complexity
- **String interning**: All identifiers are interned for memory efficiency

#### Structure
- Nodes are referenced by typed indices rather than pointers
- Each AST contains separate vectors for different node types
- Enables efficient bulk operations and memory layout control

### Semantic Analysis (`rue-semantic`)

#### Incremental Compilation
- **Query-based architecture**: Uses Salsa for incremental computation
- **Expression-level granularity**: Recomputes only changed expressions
- **IDE-first design**: Optimized for interactive development
- Similar to rust-analyzer's incremental approach

#### Analysis Phases
1. **Name Resolution**: Resolve all identifiers to their declarations
2. **Type Checking**: Verify all expressions are well-typed (trivial since everything is i64)
3. **Scope Analysis**: Validate variable scoping rules
4. **Call Graph**: Build function dependency graph

### Code Generation (`rue-codegen`)

#### Strategy
- **Stack-based evaluation**: Simple, correct code generation
- **x86-64 target**: Direct native code generation
- **System V ABI**: Compatible with C calling conventions
- **Two-pass assembler**: Symbol resolution and machine code generation

#### Instruction Generation
- Expressions compiled to stack operations
- Function calls use standard calling conventions
- Control flow implemented with conditional jumps
- Direct machine code emission (no external assembler)

#### Assembly Process
1. **First Pass**: Collect symbols and calculate addresses
2. **Second Pass**: Generate machine code with resolved addresses
3. **Relocation**: Handle forward references and jumps

### ELF Generation
- **Direct binary output**: No external linker required
- **Minimal ELF**: Only essential sections (text, data, symbol table)
- **Static linking**: Self-contained executables
- **Linux-specific**: Uses Linux system call ABI

## Design Priorities

1. **Fast compilation speed**: Primary performance goal
2. **Incremental by default**: All computations should be incremental  
3. **IDE-first design**: AST and architecture designed for IDE features
4. **Future extensibility**: Designed to grow into a full language

## Error Handling

### Philosophy
- **Quality first**: Good error messages from the start
- **Source locations**: All errors include precise source positions
- **Incremental friendly**: Errors don't block unrelated analysis
- **IDE integration**: Errors designed for real-time display

### Implementation
- Errors carry source spans for precise highlighting
- Multiple errors can be reported simultaneously
- Recovery strategies maintain partial AST for IDE features

## Testing Strategy

### Unit Tests
- Each compiler phase has comprehensive unit tests
- Property-based testing for core algorithms
- Error condition testing

### Integration Tests
- End-to-end compilation of sample programs
- Executable correctness verification
- Performance regression detection

### Continuous Integration
- Buck2 and Cargo build verification
- Cross-platform testing (when supported)
- Documentation generation and verification

## Development Infrastructure

### Language Server Protocol (LSP)
- Real-time syntax and semantic analysis
- IDE integration for VS Code and other editors
- Incremental compilation for responsive experience
- **Current limitation**: LSP only works with Cargo due to Buck2 dependency issues

### Debugging Support
- **Current**: GDB integration for compiled programs
- **Future**: DWARF debug information generation
- **Tools**: Built-in disassembly and binary inspection

### Version Control
- Designed for jj (Jujutsu) workflow
- Commit hooks for code quality
- Branching strategy for experimental features

## Build System Integration

### Buck2 Features
- **Incremental builds**: Only rebuild changed components
- **Dependency management**: Reindeer for Cargo.toml → Buck2 conversion
- **Parallel compilation**: Multi-core build execution
- **Target isolation**: Clean separation between compiler and samples

### Reindeer Workflow
1. `reindeer update` - Update Cargo.lock
2. `reindeer vendor` - Vendor dependencies  
3. `reindeer buckify` - Generate Buck2 build files
4. Use fixups/ for problematic dependencies

## Performance Characteristics

### Compilation Speed
- **Target**: Sub-second compilation for small programs
- **Incremental**: Expression-level change detection
- **Memory**: Flat AST reduces allocation overhead
- **I/O**: Minimal file system interaction

### Runtime Performance
- **Code Quality**: Stack-based code is simple but unoptimized
- **Binary Size**: Minimal ELF overhead
- **Startup**: Direct native execution, no runtime
- **Memory**: Static allocation only (no heap)

## Future Architecture Considerations

### Backend Abstraction
- Abstract code generation interface
- Pluggable backends (LLVM, Cranelift)
- Shared optimization passes
- Target-specific lowering

### Multi-Platform Support
- Platform-specific code generation
- Cross-compilation infrastructure  
- ABI compatibility layers
- Binary format abstraction

### Advanced Features
- **Optimization**: SSA-based optimization passes
- **Debug Info**: DWARF generation for debugging
- **Profiling**: Built-in performance profiling
- **Memory Management**: Garbage collection or ownership system