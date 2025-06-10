# Next Steps for Rue Development

## Completed Milestones ✅

### Core Compiler Infrastructure
- ✅ Complete lexer with all tokens and keywords
- ✅ Hand-written recursive descent parser with CST
- ✅ Salsa-based incremental compilation pipeline  
- ✅ Comprehensive semantic analysis with error reporting
- ✅ x86-64 native code generation with direct ELF output
- ✅ End-to-end compilation pipeline (rue source → native executable)
- ✅ Multi-crate workspace with dual build systems (Cargo + Buck2)

### Developer Experience
- ✅ LSP server implementation with real-time error detection
- ✅ VS Code extension with syntax highlighting and diagnostics
- ✅ Installation scripts and comprehensive documentation
- ✅ Professional IDE integration comparable to major languages

## Immediate Priorities (Next Session)

### 1. Language Feature Enhancements
- [ ] **Multiple function parameters**: `fn add(a, b) { a + b }`
- [ ] **While loops**: `while condition { body }`
- [ ] **Local variables beyond let**: Scope and lifetime management
- [ ] **Better error messages**: Source spans with line/column information

### 2. LSP Server Improvements  
- [ ] **Line/column diagnostics**: Convert from character offsets
- [ ] **Semantic diagnostics**: Undefined variables, type mismatches
- [ ] **Code completion**: Function names, keywords, variables in scope
- [ ] **Hover information**: Show variable types and function signatures

### 3. Code Generation Enhancements
- [ ] **Basic optimizations**: Constant folding, dead code elimination
- [ ] **Better calling convention**: Multiple parameters, local variables
- [ ] **Debug information**: DWARF generation for debugger support
- [ ] **Better error handling**: Improved compilation error messages

## Medium-term Goals

### 4. Advanced Language Features
- [ ] **Structs and data types**: Basic aggregate types
- [ ] **Arrays and collections**: Fixed-size arrays to start
- [ ] **String support**: Basic string literals and operations
- [ ] **Pattern matching**: Simple match expressions

### 5. Performance and Tooling
- [ ] **Optimization passes**: More sophisticated code optimization
- [ ] **Incremental compilation**: File-level and module-level caching
- [ ] **Multi-platform targets**: ARM64, WASM support
- [ ] **Package manager**: Basic dependency management

### 6. Advanced IDE Features
- [ ] **Go-to-definition**: Navigate to function/variable definitions
- [ ] **Find references**: Show all uses of a symbol
- [ ] **Refactoring**: Rename symbol, extract function
- [ ] **Code formatting**: Automatic code formatting

## Future Enhancements

### Language Evolution
- [ ] **Advanced type system**: Generics, traits, type inference
- [ ] **Memory management**: Ownership and borrowing concepts
- [ ] **Macros and metaprogramming**: Code generation facilities
- [ ] **Async/await**: Asynchronous programming support

### Ecosystem Development  
- [ ] **Standard library**: Core data structures and algorithms
- [ ] **Package registry**: Central package repository
- [ ] **Documentation generator**: Automatic API documentation
- [ ] **Testing framework**: Built-in unit testing support

## Testing Strategy

- ✅ Unit tests for each compiler phase (lexer, parser, semantic, codegen)
- ✅ Integration tests with example rue programs
- [ ] Performance benchmarks for incremental compilation
- [ ] Cross-platform validation (Linux, macOS, Windows)
- [ ] LSP integration tests with various editors
- [ ] Regression test suite for language features