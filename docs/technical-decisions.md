# Technical Decisions

## Architecture Choices

### Multi-Crate Design
**Decision:** 6-crate workspace structure  
**Rationale:** 
- Clear separation of concerns
- Faster incremental builds
- Easier testing and maintenance
- Follows Rust ecosystem patterns

### Flat AST with Integer Indices
**Decision:** ECS-inspired design with separate arrays and integer indices  
**Rationale:**
- Memory efficiency (no pointer overhead)
- Cache-friendly data layout
- Enables memory-mapped persistence
- Better for incremental compilation
- Inspired by data-oriented design principles

### Salsa for Incremental Compilation
**Decision:** Use Salsa 0.22 for query-based incremental compilation  
**Rationale:**
- Expression-level granularity by default
- Proven architecture (used by rust-analyzer)
- Automatic dependency tracking
- Efficient caching and invalidation

### Dual Build Systems
**Decision:** Support both Cargo and Buck2  
**Rationale:**
- Cargo for standard Rust ecosystem compatibility
- Buck2 for advanced incremental compilation features
- Learning opportunity for cutting-edge build systems
- Future cross-compilation capabilities

### Hand-Written Parser
**Decision:** Recursive descent parser instead of parser combinators or generators  
**Rationale:**
- Full control over error recovery
- IDE-friendly concrete syntax tree
- Easier to maintain and debug
- Better integration with incremental compilation

## Language Design Choices

### Minimal Initial Subset
**Decision:** Start with functions, if/else, arithmetic, single parameters  
**Rationale:**
- Tests all major compiler phases
- Small enough to implement quickly
- Sufficient complexity for real testing
- Clear upgrade path to full language

### Everything is i64
**Decision:** No type annotations, all variables are 64-bit integers  
**Rationale:**
- Simplifies initial implementation
- Focus on compiler architecture over type system
- Easy to extend later
- Sufficient for mathematical programs

### Exit Code as Return Value
**Decision:** main() return value becomes program exit code  
**Rationale:**
- No I/O needed initially
- Verifiable program execution
- Simple testing mechanism
- Unix-friendly design

## Development Workflow Choices

### Jujutsu (jj) over Git
**Decision:** Use jj for version control  
**Rationale:**
- Better support for modern development workflows
- Easier branch management
- More intuitive conflict resolution
- Educational value

### Comprehensive CI from Start
**Decision:** Set up extensive CI before first commit  
**Rationale:**
- Catch issues early
- Validate both build systems
- Cross-platform compatibility
- Quality enforcement from day one

### MIT/Apache 2.0 Licensing
**Decision:** Dual license like Rust ecosystem  
**Rationale:**
- Maximizes adoption potential
- Patent protection with Apache 2.0
- Simple with MIT
- Rust ecosystem standard