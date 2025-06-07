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

## Architecture

### Compiler Pipeline
- Hand-written recursive descent parser → Flat AST → Salsa-based incremental compilation → Stack-based x86-64 codegen → ELF output

### Key Design Decisions
- IDE-first design with concrete syntax trees
- Expression-level incremental computation
- Separate arrays for different AST node types (ECS-inspired)
- No traditional compiler/linker split

### Version Control
This project uses jj (Jujutsu) instead of git. Common commands:
- `jj status` - see current changes
- `jj commit -m "message"` - commit changes
- `jj log` - view commit history