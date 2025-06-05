# Next Steps for Rue Development

## Immediate Priorities (Next Session)

### 1. Complete CI Fixes
- [ ] Verify all CI checks pass on PR #1
- [ ] Merge foundational infrastructure PR

### 2. Parser Implementation
- [ ] Define AST node types in `rue-ast`
- [ ] Implement recursive descent parser in `rue-parser`
- [ ] Parse the factorial example from spec.md
- [ ] Comprehensive parser tests

**Key Design Decisions:**
- Hand-written recursive descent parser (not parser combinator or generator)
- IDE-friendly concrete syntax tree that preserves all tokens and whitespace
- Flat AST with integer indices for efficient incremental updates

### 3. AST Structure Design
- [ ] Define flat AST storage with separate arrays for different node types
- [ ] Implement integer-based indices with generational safety
- [ ] String interning for identifiers
- [ ] Span tracking for error reporting

**Architecture:**
```rust
// Example structure (to be refined)
struct FlatAst {
    expressions: Vec<Expression>,
    statements: Vec<Statement>, 
    functions: Vec<Function>,
    strings: StringInterner,
}
```

### 4. Salsa Integration
- [ ] Set up basic Salsa database
- [ ] Define incremental compilation queries
- [ ] Implement file parsing as Salsa query
- [ ] Expression-level change tracking

## Medium-term Goals

### 5. Semantic Analysis
- [ ] Type checking (everything is i64, but still validate)
- [ ] Name resolution and scope analysis
- [ ] Function signature validation
- [ ] Control flow analysis

### 6. Code Generation Setup
- [ ] Design x86-64 code generation interface
- [ ] Stack-based expression evaluation
- [ ] Basic function calling convention
- [ ] ELF executable generation

### 7. End-to-End Integration
- [ ] CLI argument parsing
- [ ] File reading and compilation pipeline
- [ ] Error reporting and diagnostics
- [ ] Compile and run factorial example

## Future Enhancements

- [ ] LSP implementation for IDE support
- [ ] Debug information generation (DWARF)
- [ ] More sophisticated error recovery in parser
- [ ] Multi-platform target support
- [ ] Advanced optimization passes

## Testing Strategy

- Unit tests for each compiler phase
- Integration tests with example rue programs  
- Performance benchmarks for incremental compilation
- Cross-platform validation