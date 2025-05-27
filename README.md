# WFL (WebFirst Language) v0.1.0

**⚠️ IMPORTANT: This software is alpha quality and should not be relied on for production use. ⚠️**

WFL is a programming language designed to be readable and intuitive, using natural language constructs to lower the barrier to entry for new programmers while still providing powerful features for experienced developers.

## Overview

WFL features a syntax that resembles English sentences, indentation-based structure, and modern programming concepts like containers (classes), actions (functions), and collections. The language is designed to be approachable for beginners while still being powerful enough for real-world applications.

## Installation Options

WFL offers two installation options to suit different needs:

| Feature | CLI-only (Default) | CLI + Editor |
|---------|-------------------|--------------|
| Binary Size | ~2.5 MB | ~12 MB |
| Compiler & Runtime | ✅ | ✅ |
| REPL | ✅ | ✅ |
| LSP Server | ✅ | ✅ |
| Integrated Editor | ❌ | ✅ |
| Git Integration | ❌ | ✅ |
| Syntax Highlighting | ❌ | ✅ |
| Live Diagnostics | ❌ | ✅ |

### Building WFL

```bash
# Default CLI-only build (small binary, no GUI dependencies)
cargo build --release

# Full IDE build with integrated editor
cargo build --release --features editor
```

### Integrated Editor

The WFL editor provides a modern, integrated development environment with syntax highlighting, live diagnostics, and Git integration.

![WFL Editor Screenshot](https://github.com/logbie/wfl/raw/main/docs/images/editor-screenshot.png)

*Note: Screenshot will be added in a future update*

## Project Status

The WFL compiler is currently in active development with most core components complete and stable. Here's the current status:

- ✅ **Lexer**: Complete - Converts source code into tokens with full support for natural language constructs
- ✅ **Parser**: Complete - Transforms tokens into an Abstract Syntax Tree (AST)
  - ✅ Enhanced to support natural language function calls (e.g., `typeof of value`)
  - ✅ **Critical Stability Fixes (May 2025)**: Comprehensive end token handling prevents infinite loops
- ✅ **Semantic Analyzer**: Complete - Analyzes the AST for semantic correctness
- ✅ **Type Checker**: Complete - Performs static type analysis on the AST
- ✅ **Standard Library**: Complete - Core functions, math, text, and list operations
- ✅ **Language Server Protocol (LSP)**: Complete - Provides editor integration with real-time diagnostics and auto-completion
- ✅ **Interpreter**: Complete - Executes the AST directly
  - ✅ Supports all basic language features
  - ✅ Includes runtime error handling and reporting
  - ✅ HTTP GET/POST support via Reqwest
  - ✅ Database integration (SQLite, MySQL, PostgreSQL) via SQLx
  - ✅ Try/when/otherwise exception handling
  - ✅ **Asynchronous operations support** - Full Tokio integration with async/await
- ✅ **Error Reporting System**: Complete - Comprehensive diagnostics with actionable messages using codespan-reporting
- ✅ **Linter and Code Fixer**: Complete - Code quality tools with CLI integration
- ✅ **Enhanced Logging System**: Complete - Standardized debug output with exec_trace! macro
- 🔄 **Bytecode Compiler**: Planned - Will convert the AST into bytecode instructions
- 🔄 **Virtual Machine**: Planned - Will execute bytecode instructions

## Recent Major Improvements (May 2025)

### Parser Stability Enhancement
- **Fixed critical infinite loop issue**: Comprehensive end token handling for all constructs (`end action`, `end check`, `end for`, `end count`, etc.)
- **Enhanced error recovery**: Improved synchronization and orphaned token consumption
- **Resolved borrow checker issues**: Stable compilation with proper token lookahead
- **Added comprehensive logging**: Better debugging and execution tracing

### Debug Output Refactoring
- **Standardized logging system**: All debug output now uses `exec_trace!` macro
- **Clean separation**: Program output no longer polluted by debug messages
- **Memory optimization**: Adjusted thresholds while maintaining efficiency
- **Enhanced traceability**: Improved execution flow analysis

## Current Capabilities

WFL now supports:

- **Asynchronous Programming**: Full async/await support with Tokio runtime
- **Network Operations**: HTTP requests with Reqwest integration
- **Database Access**: SQLite, MySQL, and PostgreSQL support via SQLx
- **File I/O**: Comprehensive file operations with async support
- **Natural Language Syntax**: English-like constructs for improved readability
- **Type Safety**: Static type checking with intelligent type inference
- **Error Handling**: Try/when/otherwise constructs for graceful error management
- **Real-time Development**: LSP server provides instant feedback in editors

## Current Limitations

- The `wait for ... and ...` construct executes sequentially (true concurrency planned for future release)
- The `open file` command creates the file if it doesn't exist (dedicated `create file` syntax planned)

## Execution Pipeline

All runs are type-checked and semantically analyzed by default. This ensures that scripts are validated for semantic correctness and type safety before execution, preventing many common runtime errors.

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
   ```bash
   # Default CLI-only build
   cargo build --release
   
   # Or build with integrated editor
   cargo build --release --features editor
   ```

### Usage

To run a WFL program:

```
cargo run -- path/to/your/program.wfl
```

Or, after building:

```
./target/release/wfl path/to/your/program.wfl
```

To launch the integrated editor (if built with `--features editor`):

```
./target/release/wfl editor [path/to/file.wfl]
```

### Creating a New Project with Editor

```bash
# Create a new project with editor scaffolding
wfl new myproject --with-editor

# Navigate to the project
cd myproject

# Launch the editor
wfl editor
```

### Development Tools

WFL includes comprehensive development tooling:

```bash
# Run with real-time error checking
wfl --interactive your_script.wfl

# Check code quality
wfl --lint your_script.wfl

# Perform static analysis
wfl --analyze your_script.wfl

# Auto-format and fix code
wfl --fix your_script.wfl --in-place

# Validate configuration
wfl --configCheck
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

### I/O and Network Module
- `open file`: Asynchronous file operations
- `open url`: HTTP requests with full async support
- `wait for`: Async/await operations for concurrent programming

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

// Asynchronous operations
try:
  wait for open url "https://api.example.com/data" and read response
  display "Data received: " with response
when error:
  display "Network error: " with error message
end try
```

## Project Structure

- `src/`: Source code
  - `lexer/`: Lexical analyzer with Logos integration
  - `parser/`: Parser and AST with comprehensive error handling
  - `analyzer/`: Semantic analyzer
  - `typechecker/`: Static type checker
  - `interpreter/`: Runtime interpreter with Tokio async support
  - `stdlib/`: Standard library implementation
  - `logging/`: Structured logging system with exec_trace! macro
  - `diagnostics/`: Error diagnostic and reporting system using codespan-reporting
  - `debug_report/`: Debugging tools and runtime error reports
- `Docs/`: Comprehensive documentation
  - `wfl-spec.md`: Language specification
  - `wfl-foundation.md`: Design principles
  - `wfl-error.md`: Error handling philosophy
  - `wfl-staticTypeChecker.md`: Type system design
  - `wfl-interpretor.md`: Interpreter design
  - `error_catalog.md`: Comprehensive error message documentation
  - `implementation_progress_*.md`: Implementation status reports
- `Test Programs/`: Example WFL programs and test cases
  - Various test scripts demonstrating language features
  - `error_examples/`: Sample scripts demonstrating different error types
- `wfl-lsp/`: Language Server Protocol implementation for editor integration
- `Tools/`: Utility scripts for development and deployment
  - `launch_msi_build.py`: MSI build launcher with version management
  - `wfl_config_checker.py`: Configuration validation tool
  - `rust_loc_counter.py`: Statistics for Rust code
  - `wfl_md_combiner.py`: Markdown documentation combiner
- `vscode-wfl/`: VSCode extension for WFL syntax highlighting and LSP integration

## Error Reporting and Diagnostics

WebFirst Language includes a comprehensive error reporting system that provides clear, actionable error messages to help developers quickly identify and fix issues:

- **User-Friendly Error Messages**: Inspired by Elm's approach to error messages, using codespan-reporting for professional formatting
- **Source Context**: Error messages include the relevant source code snippets with precise highlighting
- **Actionable Suggestions**: For common errors, WebFirst Language suggests specific fixes and corrections
- **Unified Error System**: Consistent error formatting across all error types (syntax, semantic, type, runtime)
- **Contextual Hints**: Special handling for common mistakes like missing keywords in variable declarations
- **Enhanced Debugging**: Standardized exec_trace! macro for consistent debug output

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

WFL includes structured logging and automatic debug report generation to help with troubleshooting.

### Enhanced Logging System

- **Standardized Debug Output**: All debug messages use the `exec_trace!` macro
- **Clean Separation**: Program output is separate from debugging information
- **Centralized Control**: Debug verbosity controlled through configuration
- **Memory Optimized**: Efficient logging with minimal overhead

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

### Configuration Validation

WFL provides tools to validate and fix configuration files:

```bash
# Check configuration files for issues
wfl --configCheck

# Check and automatically fix configuration issues
wfl --configFix
```

These commands validate `.wflcfg` files against expected settings and types. The `--configCheck` flag reports issues without making changes, while `--configFix` attempts to automatically correct problems.

Configuration files are searched in the following order:
1. Global configuration (environment variable `WFL_GLOBAL_CONFIG_PATH` or platform default)
   - Linux/macOS: `/etc/wfl/wfl.cfg`
   - Windows: `C:\wfl\config`
2. Local configuration (`.wflcfg` in the current directory)

Local settings override global ones for overlapping keys.

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

## Editor Integration

### Language Server Protocol (LSP)

WFL includes a fully functional LSP server (`wfl-lsp`) that provides:

- **Real-time Diagnostics**: Errors and warnings as you type
- **Auto-completion**: Context-aware suggestions for keywords, variables, and functions
- **Go-to Definition**: Navigate to symbol definitions
- **Hover Information**: Type and documentation information on hover
- **Symbol Search**: Find symbols across the project

### VSCode Extension

The project includes a VSCode extension with:

- Syntax highlighting for WFL files
- Integration with the LSP server
- Automatic error checking
- Code formatting and fixing

Install the extension by running:
```bash
scripts/install_vscode_extension.ps1
```

## Deployment and Packaging

WFL supports multiple deployment formats:

### Windows MSI Installer
```bash
python Tools/launch_msi_build.py
```

### Debian Package
```bash
cargo deb
```

### Portable Binary
```bash
cargo build --release
```

The binary includes:
- Complete WFL runtime
- Built-in standard library
- LSP server
- Development tools

## Performance and Memory

- **Efficient Parsing**: Logos-based lexer for fast tokenization
- **Memory Optimized**: Careful memory management with leak detection
- **Async Runtime**: Tokio integration for concurrent operations
- **Benchmarking**: Criterion-based performance tests

## Dependencies

Core dependencies include:
- **Logos**: High-performance lexical analysis
- **Tokio**: Async runtime for concurrent operations
- **Reqwest**: HTTP client for network operations
- **SQLx**: Database connectivity (SQLite, MySQL, PostgreSQL)
- **Codespan-reporting**: Professional error message formatting
- **Rustyline**: Interactive REPL with history and editing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Clone the repository
2. Install Rust (latest stable)
3. Run `cargo build` to compile
4. Run `cargo test` to execute tests
5. Use `cargo run -- test.wfl` to test with sample programs

## License

This project is licensed under the Apache-2.0 License - see the LICENSE file for details.

## Version History

- **v0.1.0**: Initial release with complete interpreter, LSP server, and development tools
