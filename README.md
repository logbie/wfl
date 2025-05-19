# WFL (WebFirst Language)

**⚠️ IMPORTANT: This software is alpha quality at best and should not be relied on for production use. ⚠️**

WFL is a programming language designed to be readable and intuitive, using natural language constructs to lower the barrier to entry for new programmers while still providing powerful features for experienced developers.

## Overview

WFL features a syntax that resembles English sentences, indentation-based structure, and modern programming concepts like containers (classes), actions (functions), and collections. The language is designed to be approachable for beginners while still being powerful enough for real-world applications.

## Project Status

The WFL compiler is currently under development. Here's the current status:

- ✅ **Lexer**: Complete - Converts source code into tokens
- ✅ **Parser**: Complete - Transforms tokens into an Abstract Syntax Tree (AST)
  - ✅ Enhanced to support natural language function calls (e.g., `typeof of value`)
- ✅ **Semantic Analyzer**: Complete - Analyzes the AST for semantic correctness
- ✅ **Type Checker**: Complete - Performs static type analysis on the AST
- ✅ **Standard Library**: Complete - Core functions, math, text, and list operations
- ✅ **Language Server Protocol (LSP)**: Complete - Provides editor integration with real-time diagnostics and auto-completion
- ✅ **Interpreter**: Complete - Executes the AST directly
  - ✅ Supports all basic language features
  - ✅ Includes runtime error handling and reporting
  - ✅ HTTP GET/POST support
  - ✅ Try/when/otherwise exception handling
  - 🔄 Asynchronous operations support (in progress, tracked in issue #51)
- ✅ **Error Reporting System**: Complete - Comprehensive diagnostics with actionable messages
- ✅ **Linter and Code Fixer**: Complete - Code quality tools with CLI integration
- 🔄 **Bytecode Compiler**: Planned - Will convert the AST into bytecode instructions
- 🔄 **Virtual Machine**: Planned - Will execute bytecode instructions

### Known Issues - FIXED

- There's a specific issue with how the interpreter handles the "count" keyword in expressions inside count loops. When using "count" directly in a display statement inside a count loop (e.g., `display "Count: " with count`), the interpreter may hang. A workaround is to store the count value in a separate variable and use that variable in expressions.

## Current Limitations

- The `wait for ... and ...` construct is currently sequential until real concurrency is implemented in a future release (tracked in issue #51).
- The `open file` command creates the file if it doesn't exist. A future `create file` syntax is planned.

## Execution Pipeline

All runs are now type‑checked and semantically analyzed by default. This ensures that scripts are validated for semantic correctness and type safety before execution, preventing many common runtime errors.

## AI-Assisted Development

This project is developed with the assistance of AI:

- **Devin.ai**: Primary AI developer responsible for core implementation
- **ChatGPT (GPT-o3)**: Assisted with code reviews and optimization
- **Claude (via Cline)**: Assisted with documentation and architectural design
- **Grok**: Indirectly contributed to the project through knowledge base

The combination of AI assistance with human oversight has allowed for rapid development while maintaining high code quality and documentation standards.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/logbie/wfl.git
   cd wfl
   ```

2. Build the project:
   ```
   cargo build
   ```

### Usage

To run a WFL program:

```
cargo run -- path/to/your/program.wfl
```

Or, after building:

```
./target/debug/wfl path/to/your/program.wfl
```

## Standard Library

WFL includes a comprehensive standard library with the following modules:

### Core Module
- `print`: Outputs text to the console
- `typeof`: Returns the type of a value as text
- `isnothing`: Checks if a value is nothing (null)

### Math Module
- `abs`: Returns the absolute value of a number
- `round`, `floor`, `ceil`: Rounding functions
- `random`: Generates a random number between 0 and 1
- `clamp`: Constrains a value between a minimum and maximum

### Text Module
- `length`: Returns the length of a text string
- `touppercase`, `tolowercase`: Case conversion functions
- `contains`: Checks if a text string contains another string
- `substring`: Extracts a portion of a text string

### List Module
- `length`: Returns the number of items in a list
- `push`: Adds an item to the end of a list
- `pop`: Removes and returns the last item from a list
- `contains`: Checks if a list contains a specific item
- `indexof`: Returns the position of an item in a list

## Example WFL Program

```
store greeting as "Hello, World!"
display greeting

check if 5 is greater than 3:
  display "Math works!"
otherwise:
  display "Something is wrong with the universe."
end check

count from 1 to 5:
  display "Counting: " with the current count
end count

// Using standard library functions
store my list as [1, 2, 3, 4, 5]
display "List length: " with length of my list
display "Type of list: " with typeof of my list
```

## Project Structure

- `src/`: Source code
  - `lexer/`: Lexical analyzer
  - `parser/`: Parser and AST
  - `analyzer/`: Semantic analyzer
  - `typechecker/`: Static type checker
  - `interpreter/`: Runtime interpreter
  - `stdlib/`: Standard library implementation
  - `logging/`: Structured logging system
  - `diagnostics/`: Error diagnostic and reporting system
  - `debug_report/`: Debugging tools and runtime error reports
  - `bytecode/`: Bytecode compiler (planned)
- `Docs/`: Documentation
  - `wfl-spec.md`: Language specification
  - `wfl-foundation.md`: Design principles
  - `wfl-error.md`: Error handling philosophy
  - `wfl-staticTypeChecker.md`: Type system design
  - `wfl-interpretor.md`: Interpreter design
  - `error_catalog.md`: Comprehensive error message documentation
  - `implementation_progress_*.md`: Implementation status reports
- `Test Programs/`: Example WFL programs
  - Various test scripts demonstrating language features
  - `error_examples/`: Sample scripts demonstrating different error types
- `wfl-lsp/`: Language Server Protocol implementation
- `Tools/`: Utility scripts for development

## Error Reporting and Diagnostics

WebFirst Language includes a comprehensive error reporting system that provides clear, actionable error messages to help developers quickly identify and fix issues:

- **User-Friendly Error Messages**: Inspired by Elm's approach to error messages, WebFirst Language provides detailed, human-readable error messages
- **Source Context**: Error messages include the relevant source code snippets with precise highlighting
- **Actionable Suggestions**: For common errors, WebFirst Language suggests specific fixes and corrections
- **Unified Error System**: Consistent error formatting across all error types (syntax, semantic, type, runtime)
- **Contextual Hints**: Special handling for common mistakes like missing keywords in variable declarations

## Code Quality Suite

WebFirst Language includes a built-in code quality suite with three main components:

### Linter (`--lint`)

The linter checks your code for style issues and best practices:

```bash
wfl --lint your_script.wfl
```

It enforces:
- Naming conventions (snake_case for variables and actions)
- 4-space indentation
- Consistent keyword casing (lowercase)
- No trailing whitespace
- Line length limits (default: 100 characters)
- Nesting depth limits (default: 5 levels)

### Static Analyzer (`--analyze`)

The static analyzer performs deeper code analysis:

```bash
wfl --analyze your_script.wfl
```

It detects:
- Unused variables and actions
- Unreachable code and dead branches
- Variable shadowing
- Inconsistent return paths

### Code Fixer (`--fix`)

The code fixer automatically formats your code and performs safe refactorings:

```bash
# Print fixed code to stdout
wfl --fix your_script.wfl

# Overwrite the file with fixed code
wfl --fix your_script.wfl --in-place

# Show a diff of the changes
wfl --fix your_script.wfl --diff
```

The fixer performs the following operations:
- Pretty-prints the code with consistent formatting
- Renames identifiers to follow snake_case convention
- Removes dead code
- Simplifies boolean expressions

#### Recent Improvements

- Fixed linter CLI behavior to allow `--lint --fix` combination
- Removed unconditional linter run during normal execution
- Updated CLI help text to reflect new flag behavior
- Added support for combined flags (e.g., `--lint --fix --diff`)
- Improved reporting methods for better fixer summaries

## Logging and Debugging

In addition to the error reporting system, WFL includes structured logging and automatic debug report generation to help with troubleshooting.

### Configuration

These features can be configured in a `.wflcfg` file in the same directory as your script:

```
# Enable structured logging (default: false)
logging_enabled = true

# Set log level: debug, info, warn, error (default: info)
log_level = debug

# Enable automatic debug reports on errors (default: true)
debug_report_enabled = true

# Set execution timeout in seconds (default: 60)
timeout_seconds = 120

# Code quality settings
max_line_length = 100
max_nesting_depth = 5
indent_size = 4
snake_case_variables = true
trailing_whitespace = false
consistent_keyword_case = true
```

### Logging

When enabled, logs are written to both the console (info level and above) and to a `wfl.log` file (all levels).
Each log entry includes a timestamp, message, source location, and elapsed time.

### Debug Reports

When a runtime error occurs, WFL automatically generates a `<script>_debug.txt` file containing:
- Error summary
- Stack trace
- Source code around the error
- Full action body (if inside an action)
- Local variables at the time of the error

This makes it easier to diagnose and fix issues in your WFL scripts.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
