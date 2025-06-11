# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rue is a programming language that starts as a minimal subset of Rust, designed to explore cutting-edge compiler implementation techniques. The compiler is written in Rust and uses Buck2 as its build system.

**Platform Support**: Linux x86-64 only (generates ELF executables)

Key features:
- Compiles to x86-64 native code (ELF executables)
- Incremental compilation using Salsa
- ECS-inspired flat AST with integer indices
- All variables are 64-bit integers
- Supports functions, arithmetic, and if/else

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

### Version Control

**IMPORTANT**: This project uses jj (Jujutsu) exclusively. NEVER use git commands.

#### Basic jj Commands
- `jj status` - Show current changes and working copy status
- `jj commit -m "message"` - Commit current changes with a message
- `jj describe -m "message"` - Set/update description of current change
- `jj log` - View commit history (graphical view)
- `jj diff` - Show diff of current changes
- `jj files` - List files changed in working copy
- `jj show` - Show details of current commit

#### Branch Management
- `jj new` - Create new change (equivalent to git checkout -b)
- `jj new trunk` - Create new change based on trunk
- `jj edit <commit>` - Switch to editing a specific commit
- `jj abandon` - Abandon current change

#### Synchronization and Pull Requests
- `jj git fetch` - Fetch changes from remote repository
- `jj rebase -d trunk` - Rebase current change onto trunk
- `jj git push -c @` - Push current change and create bookmark (@ refers to current change)

**Pull Request Workflow:**
1. `jj new trunk` - Create new change from trunk
2. Make your changes and test them
3. `jj commit -m "descriptive message"` - Commit changes
4. `jj git push -c @` - Push to remote and create bookmark (note the bookmark name from output)
5. `gh pr create --head <bookmark-name-from-step-4>` - Create pull request using the bookmark name shown in previous step
6. After review: `jj git fetch && jj rebase -d trunk` if needed

**Pull Request Message Guidelines:**
- Keep descriptions concise and technical, avoid LLM-style verbosity
- Focus on what was changed, not implementation details
- Use bullet points for multiple changes
- Avoid phrases like "This PR", "I have implemented", or overly formal language
- Example: "Add while loops to parser and codegen" rather than "This pull request implements comprehensive while loop support across the compiler pipeline"

#### Commit Message Guidelines
- Write in imperative mood and present tense
- Be descriptive about what the change accomplishes
- **If the change modifies spec.md**: Use the language specification change as the primary basis for the commit message, as this describes the fundamental change being made to the language
- Examples: "Add while loop support to parser", "Fix segfault in code generation", "Add assignment operators to the language"

#### Advanced Operations
- `jj split` - Split current change into multiple commits
- `jj squash` - Squash changes into parent commit
- `jj duplicate` - Create duplicate of current change
- `jj restore <file>` - Restore file from parent commit

### Reindeer and Buck2 Dependency Management

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

## Documentation Maintenance

**CRITICAL**: Keep project documentation up to date with implementation changes:

### Language Specification
When implementing new language features or modifying existing language behavior:

1. **Update the formal specification FIRST** - Modify [docs/spec.md](./docs/spec.md) before implementing
2. **Specification-driven development** - The language spec is the authoritative definition of Rue
3. **Update examples and tests** - Ensure spec examples match implementation behavior
4. **Version the specification** - Track language version changes in the spec document
5. **Validate against spec** - Implementation must conform to the formal specification

The formal language specification in `docs/spec.md` is designed to be implementation-independent. Any alternative Rue compiler should be able to use this specification to achieve compatibility.

**Workflow for language changes:**
1. Propose specification changes in `docs/spec.md`
2. Update grammar, semantics, and examples as needed
3. Implement the feature in the compiler
4. Verify implementation matches specification exactly
5. Add conformance tests that validate spec compliance

### README Maintenance
When making significant changes to the project:

1. **Update README.md** - Keep the README accurate with current features and capabilities
2. **Verify technical accuracy** - Test all code examples and commands in the README
3. **Update build instructions** - Ensure both Cargo and Buck2 examples work correctly
4. **Language feature updates** - When adding/removing language features, update the feature list
5. **Sample programs** - Verify example programs still compile and run as described

### Design Documentation Workflow

When working on significant features or architectural changes:

**Documentation Structure:**
- `docs/sessions/session-N-feature-name/` - Contains all session-related documentation
  - `session-summary.md` - **Permanent** - High-level summary of what was accomplished
  - `design-decisions.md` - **Permanent** - Design rationale, alternatives considered, trade-offs
  - `implementation-plan.md` - **Temporary** - Step-by-step implementation tasks, deleted when complete
- `docs/architecture/feature-name/` - **Permanent** - Technical specifications and guides
- `docs/decisions/NNN-feature-name.md` - **Permanent** - Numbered Architecture Decision Records (ADRs)

**Workflow for Major Features:**
1. **Start new session** - Create `docs/sessions/session-N-feature-name/` directory
2. **Document design decisions** - Record why specific choices were made, alternatives considered
3. **Create implementation plan** - Break down work into concrete tasks with checkboxes (temporary document)
4. **Implement feature** - Follow the implementation plan, check off completed tasks
5. **Create permanent documentation** - Move key specs to `docs/architecture/`
6. **Write session summary** - Document what was accomplished and lessons learned
7. **Clean up** - Delete temporary implementation plans, keep design decisions and summaries

**Returning to Work:**
- If `implementation-plan.md` exists in a session directory, that's the current active work
- Read `design-decisions.md` first for context when returning to implementation
- Check the **Implementation Checklist** section for small, actionable tasks with checkboxes
- Update checkboxes as work progresses to track completion status
- Check `docs/architecture/` for formal specifications and technical details

**Implementation Plan Format:**
- Include an **Implementation Checklist** at the top with numbered, actionable tasks
- Use checkboxes `- [ ]` for easy progress tracking
- Break large tasks into small steps (15-30 minutes each)
- Number tasks (e.g., **1.1a**, **1.1b**) for easy reference
- Keep checklist updated as work progresses
- **Commit each step** with concise messages for checkpoint tracking (can be squashed later)
- **Update checklist before committing** - mark tasks complete before making the commit

**Important:** Implementation plans are working documents that get deleted when complete. Design decisions and session summaries are permanent historical records.