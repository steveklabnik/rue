# Rue Language Roadmap

## Current Status (v0.1)

✅ **Completed Features**:
- 64-bit integer variables with `let` declarations
- Variable assignment and reassignment
- Arithmetic operations (+, -, *, /, %)
- Comparison operations (<=, >=, <, >, ==, !=)
- Control flow: if/else statements and while loops
- Functions with single parameter and return value
- Native x86-64 compilation to ELF executables
- Language Server Protocol (LSP) support
- VS Code extension with syntax highlighting

## Near-Term Goals (v0.2-0.3)

### Language Features
- **Multiple function parameters**: `fn add(a, b, c) { a + b + c }`
- **Local variable scoping improvements**: Better nested scope handling
- **Comments**: Single-line `//` and multi-line `/* */` comments
- **Boolean literals**: `true` and `false` keywords
- **Improved error messages**: Better source location reporting

### Standard Library Expansion
- **I/O operations**: Basic print/input functionality
- **Mathematical functions**: abs, min, max, etc.
- **String literals**: Basic string support (UTF-8)

### Tooling Improvements
- **Formatter**: Automatic code formatting (`rue fmt`)
- **Documentation generator**: Extract docs from comments
- **Package manager**: Basic dependency management

## Medium-Term Goals (v0.4-0.6)

### Type System Evolution
- **Multiple data types**: 
  - Integers: i8, i16, i32, i64, u8, u16, u32, u64
  - Floating point: f32, f64
  - Boolean: bool
  - Strings: String type
- **Type annotations**: Optional explicit typing
- **Type inference**: Automatic type deduction
- **Arrays**: Fixed-size and dynamic arrays

### Advanced Language Features
- **Structs**: User-defined data types
- **Enums**: Algebraic data types
- **Pattern matching**: match expressions
- **Closures**: Anonymous functions with capture
- **Modules**: Code organization and namespacing

### Control Flow Extensions
- **for loops**: Iterator-based iteration
- **loop/break/continue**: Infinite loops with early exit
- **return statements**: Early function return

## Long-Term Vision (v1.0+)

### Advanced Type System
- **Generics**: Parameterized types and functions
- **Traits**: Interface-like behavior definition
- **Lifetime system**: Memory safety without garbage collection
- **Ownership system**: Rust-like memory management

### Concurrency
- **Async/await**: Asynchronous programming support
- **Channels**: Message passing between tasks
- **Threads**: Low-level threading primitives

### Advanced Features
- **Macros**: Compile-time metaprogramming
- **Unsafe code**: Low-level system programming
- **Foreign Function Interface (FFI)**: C library integration
- **Inline assembly**: Direct assembly code embedding

## Compiler Infrastructure Roadmap

### Performance Optimization
- **SSA-based IR**: Static Single Assignment intermediate representation
- **Optimization passes**: Dead code elimination, constant folding, etc.
- **Register allocation**: Efficient CPU register usage
- **LLVM backend**: Alternative high-performance backend

### Platform Support
- **Windows support**: PE executable generation
- **macOS support**: Mach-O executable generation
- **ARM64 support**: Apple Silicon and server ARM
- **WebAssembly target**: Browser and serverless deployment

### Development Experience
- **Incremental compilation**: File-level and module-level caching
- **Parallel compilation**: Multi-threaded compilation pipeline
- **IDE improvements**: Better autocomplete, refactoring, debugging
- **Package registry**: Central package repository

### Debug and Profiling
- **DWARF debug info**: GDB/LLDB integration
- **Built-in profiler**: Performance analysis tools
- **Memory debugging**: Leak detection and usage analysis
- **Trace generation**: Execution flow visualization

## Research and Exploration Areas

### Cutting-Edge Compiler Techniques
- **Query-based compilation**: Further Salsa integration
- **Persistent data structures**: Immutable AST representations
- **Parallel semantic analysis**: Multi-threaded type checking
- **Incremental linking**: Fast executable regeneration

### Language Design Experiments
- **Effect systems**: Tracking side effects in the type system
- **Dependent types**: Types that depend on values
- **Linear types**: Resource management through the type system
- **Capability-based security**: Fine-grained permission systems

### Integration Experiments
- **Build system integration**: Treating functions as build targets
- **Hot reloading**: Live code updates during development
- **Distributed compilation**: Network-based compilation clusters
- **IDE-compiler fusion**: Deeper editor integration

## Timeline Estimates

- **v0.2**: Multiple parameters, comments, basic I/O (2-3 months)
- **v0.3**: Multiple data types, arrays, better tooling (3-4 months)
- **v0.4**: Structs, enums, pattern matching (4-6 months)
- **v0.5**: Generics, traits, advanced features (6-8 months)
- **v1.0**: Ownership system, full language (12-18 months)

## Success Metrics

### Language Adoption
- **Developer satisfaction**: Easy learning curve and good ergonomics
- **Performance competitiveness**: Comparable to C/Rust for systems programming
- **Ecosystem growth**: Package ecosystem and community contributions
- **Production usage**: Real-world applications built with Rue

### Technical Excellence
- **Compilation speed**: Sub-second compilation for medium projects
- **Runtime performance**: Within 10% of equivalent C/Rust code
- **Memory safety**: Zero-cost abstractions with safety guarantees
- **IDE experience**: Best-in-class development environment

## Open Questions

### Language Design
- How aggressive should type inference be?
- Should Rue have garbage collection or ownership?
- What memory model should concurrent Rue use?
- How should Rue handle errors (exceptions, Result types, etc.)?

### Implementation Strategy  
- When to introduce LLVM vs. continue custom backend?
- How to balance compilation speed vs. runtime performance?
- Should Rue target existing packaging ecosystems (cargo, npm)?
- How to maintain incremental compilation with advanced features?

### Community and Ecosystem
- What governance model for language evolution?
- How to bootstrap a package ecosystem?
- What industries/domains to target first?
- How to balance innovation with stability?

---

This roadmap is a living document and will evolve based on user feedback, technical discoveries, and changing priorities. The goal is to build a language that combines the performance of systems languages with the ergonomics of modern high-level languages.