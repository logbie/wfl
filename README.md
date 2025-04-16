# WFL (WebFirst Language)

## TL;DR
WFL is a programming language designed to be readable and intuitive, using natural language constructs instead of special characters and symbols. It aims to lower the barriers to entry for beginners while providing powerful features for experienced developers.

### Key Features
- Natural language-like syntax
- Minimal use of special characters
- Strong type safety
- Modern features including async operations and pattern matching
- Clear and actionable error messages

### Project Structure
- `src/` - Source code for the WFL compiler/interpreter
  - `lexer/` - Tokenizes source code
  - `parser/` - Builds AST from tokens
  - `bytecode/` - Compiles AST to bytecode
- `benches/` - Performance benchmarks
- Documentation:
  - `wfl spec.md` - Complete language specification
  - `todo.md` - Development roadmap
  - `implementation_progress.md` - Current implementation status
  - `missing_parser_functions.md` - Detailed analysis of parser gaps
  - `missing_bytecode_features.md` - Required bytecode compiler features
  - `wfl_compatibility_issues.md` - Integration issues between components

### Implementation Status

The WFL interpreter is under active development. Currently:
- ✅ Lexer (complete)
- 🟡 Parser (partial implementation)
- 🟡 Bytecode Compiler (partial implementation) 
- 🔴 Virtual Machine (not started)

See [implementation_progress.md](implementation_progress.md) for detailed information about the current state of the project.

### Implementation Documentation

We maintain several specialized documents to track development progress:

| Document | Purpose | Status |
|----------|---------|--------|
| [Implementation Progress](implementation_progress.md) | Overall status of each component | Updated |
| [Missing Parser Functions](missing_parser_functions.md) | Detailed analysis of parser features to implement | Updated |
| [Missing Bytecode Features](missing_bytecode_features.md) | Required bytecode compiler components | Updated |
| [Compatibility Issues](wfl_compatibility_issues.md) | Integration challenges between components | Updated |
| [Todo List](todo.md) | Comprehensive task list by priority | Updated |

### Built With
- Rust
- logos (lexical analysis)
- chumsky (parsing)
- thiserror & anyhow (error handling)

## More Information
For a comprehensive understanding of WFL's syntax, features, and design principles, please refer to `wfl spec.md` in this repository.