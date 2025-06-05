# rue

A programming language that starts as a minimal subset of Rust, designed to explore cutting-edge compiler implementation techniques.

## About

Rue is an experimental programming language that begins with a very small, simple subset of Rust syntax but focuses on modern compiler architecture:

- **Incremental compilation** using Salsa with expression-level granularity
- **IDE-first design** with concrete syntax trees that preserve all source information
- **ECS-inspired flat AST** with integer indices for memory efficiency
- **Dual build systems** supporting both Cargo and Buck2
- **x86-64 native code generation** with plans for multi-platform support

## Current Status

🚧 **Early Development** - Currently implements:
- ✅ Complete lexer for basic tokens, keywords, operators
- ✅ Multi-crate workspace architecture  
- ✅ Comprehensive CI/CD with both build systems
- 🔧 Parser (in progress)
- 🔧 AST definitions (planned)
- 🔧 Salsa-based incremental compilation (planned)
- 🔧 x86-64 code generation (planned)

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

## Building

### With Cargo
```bash
cargo build
cargo test
cargo run
```

### With Buck2
```bash
buck2 build //crates/rue:rue
buck2 run //crates/rue:rue
```

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