# WFL Architecture Documentation

## System Architecture Overview

WFL is implemented as a traditional language processing pipeline with modern enhancements for developer experience and runtime capabilities.

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    Lexer    │────>│   Parser    │────>│  Analyzer   │────>│ TypeChecker │────>│ Interpreter │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
     │                    │                   │                   │                    │
     ▼                    ▼                   ▼                   ▼                    ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   Error Reporting System                                    │
└─────────────────────────────────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Lexer (`src/lexer/`)
- **Purpose**: Converts source code text into tokens
- **Implementation**: Based on Logos for efficient tokenization
- **Key Features**:
  - Full support for natural language constructs
  - Error recovery with context-aware diagnostics
  - Position tracking for accurate error reporting

### 2. Parser (`src/parser/`)
- **Purpose**: Transforms token stream into Abstract Syntax Tree (AST)
- **Key Features**:
  - Recursive descent parsing with error recovery
  - Enhanced end token handling (critical stability fix, May 2025)
  - Support for natural language function calls
  - Comprehensive token lookahead with proper borrow checking

### 3. Semantic Analyzer (`src/analyzer/`)
- **Purpose**: Analyzes AST for semantic correctness
- **Key Features**:
  - Unused variable detection
  - Unreachable code detection
  - Variable shadowing analysis
  - Inconsistent return path detection
  - Control flow graph generation and analysis

### 4. Type Checker (`src/typechecker/`)
- **Purpose**: Performs static type analysis
- **Key Features**:
  - Type inference
  - Type compatibility checking
  - Error reporting with suggestions

### 5. Interpreter (`src/interpreter/`)
- **Purpose**: Executes the AST
- **Implementation**: Direct AST interpretation with Tokio integration
- **Key Features**:
  - Full async/await support
  - HTTP requests via Reqwest
  - Database access via SQLx
  - Try/when/otherwise exception handling
  - Environment management with proper scoping

### 6. Standard Library (`src/stdlib/`)
- **Modules**:
  - Core: Basic operations and utilities
  - Math: Mathematical operations
  - Text: String manipulation
  - List: Collection operations
  - Pattern: Regular expression and pattern matching
  - I/O: File operations and network access

### 7. Error Reporting System (`src/diagnostics/`)
- **Purpose**: Comprehensive error reporting
- **Implementation**: Based on codespan-reporting
- **Key Features**:
  - Source context with highlighting
  - Actionable suggestions
  - Unified error formatting

## Development Tools

### 1. Linter & Code Fixer (`src/linter/`, `src/fixer/`)
- **Purpose**: Code quality tools
- **Key Features**:
  - Style checking
  - Best practice enforcement
  - Automatic code fixes

### 2. Logging System (`src/logging/`)
- **Purpose**: Debug output and tracing
- **Key Features**:
  - Standardized exec_trace! macro
  - Clean separation of debug and program output
  - Memory optimization

### 3. REPL (`src/repl/`)
- **Purpose**: Interactive development
- **Key Features**:
  - Command history
  - Multi-line editing
  - Immediate feedback

### 4. LSP Server (`wfl-lsp/`)
- **Purpose**: IDE integration
- **Key Features**:
  - Real-time diagnostics
  - Auto-completion
  - Go-to definition
  - Hover information

## File Organization

```
wfl/
├── src/                # Source code
│   ├── lexer/          # Lexical analyzer
│   ├── parser/         # Parser and AST definition
│   ├── analyzer/       # Semantic analyzer
│   ├── typechecker/    # Type checker
│   ├── interpreter/    # Runtime interpreter
│   ├── stdlib/         # Standard library
│   ├── diagnostics/    # Error reporting
│   ├── linter/         # Code quality tools
│   ├── fixer/          # Automatic code fixes
│   ├── logging/        # Logging system
│   └── repl/           # Interactive shell
├── Docs/               # Documentation
├── Test Programs/      # Example programs and tests
├── wfl-lsp/            # Language Server Protocol implementation
├── vscode-wfl/         # VSCode extension
└── Tools/              # Utility scripts
```

## Key Design Patterns

1. **Visitor Pattern**: Used in the analyzer and interpreter to traverse the AST
2. **Builder Pattern**: Used in AST construction
3. **Command Pattern**: Used in the REPL for command history
4. **Observer Pattern**: Used in error reporting and logging
5. **Factory Pattern**: Used in standard library function registration

## Critical Implementation Paths

1. **Execution Pipeline**:
   - Source → Lexer → Parser → Analyzer → Type Checker → Interpreter
   - All runs are type-checked and semantically analyzed by default

2. **Error Recovery**:
   - Parser synchronization points
   - Context-aware error reporting
   - Graceful degradation in analysis

3. **Async Execution**:
   - Tokio runtime initialization
   - Task spawning and management
   - Future resolution and handling

4. **Memory Management**:
   - Environment hierarchies with weak references
   - Efficient string and value representation
   - Memory leak prevention in closures

## Future Architecture Plans

1. **Bytecode Compiler**:
   - Convert AST to bytecode instructions
   - Optimization passes
   - Constant folding and dead code elimination

2. **Virtual Machine**:
   - Register-based VM
   - JIT compilation support
   - Performance optimizations