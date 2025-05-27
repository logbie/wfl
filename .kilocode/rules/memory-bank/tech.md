# WFL Technology Stack

## Core Technologies

### Programming Languages
- **Rust**: Primary implementation language, chosen for safety, performance, and modern language features
- **WFL**: The language itself (used for testing and examples)

### Core Libraries
- **Logos**: High-performance lexical analyzer that powers the tokenization process
- **Tokio**: Asynchronous runtime that enables non-blocking I/O operations
- **Reqwest**: HTTP client library for network operations
- **SQLx**: Database connectivity supporting SQLite, MySQL, and PostgreSQL
- **codespan-reporting**: Professional error message formatting with source context
- **Rustyline**: Interactive REPL with history and editing capabilities

## Development Environment

### Requirements
- **Rust Toolchain**: Latest stable version
- **Cargo**: Package manager and build tool (comes with Rust)
- **Git**: Version control system
- **Visual Studio Code**: Recommended IDE with WFL extension support

### Development Setup
1. Clone the repository: `git clone https://github.com/logbie/wfl.git`
2. Install Rust (latest stable): `rustup update stable`
3. Build the project: `cargo build`
4. Run tests: `cargo test`
5. Install VSCode extension: `scripts/install_vscode_extension.ps1`

## Build System

### Build Configurations
- **Debug**: `cargo build` - Includes debug symbols and assertions
- **Release**: `cargo build --release` - Optimized for performance
- **Test**: `cargo test` - Runs all unit and integration tests

### Cross-Platform Support
- **Windows**: Primary development platform with MSI installer support
- **Linux**: Supported with deb packages and tar.gz archives
- **macOS**: Supported with pkg installer

### Automated Builds
- Skip-if-unchanged logic to avoid unnecessary builds
- Nightly build pipeline for continuous integration
- Version management through `scripts/bump_version.py`

## Testing Framework

### Test Types
- **Unit Tests**: Located alongside source code
- **Integration Tests**: Located in `tests/` directory
- **Memory Tests**: Specialized tests for memory leak detection
- **Snapshot Tests**: Tests for diagnostics and error reporting

### Test Tools
- **Rust's built-in testing framework**: `cargo test`
- **DHAT**: Heap profiling via `dhat-heap` feature
- **Custom scripts**: `scripts/run_wfl_tests.sh`

## Deployment and Packaging

### Release Formats
- **Windows MSI**: Created via `Tools/launch_msi_build.py`
- **Debian Package**: Created via `cargo deb`
- **Portable Binary**: Created via standard release build

### Deployment Process
1. Bump version numbers
2. Run integration test suite
3. Build platform-specific installers
4. Generate documentation updates
5. Create release packages

## Technical Constraints

### Memory Management
- Careful handling of environment references to avoid leaks
- Efficient string interning for token storage
- Weak references for parent environments in closures

### Performance Considerations
- Efficient parsing with Logos-based lexer
- Memory-optimized AST representation
- Append-mode file operations instead of read-modify-write

### Compatibility Requirements
- Support for older Rust compiler versions
- Cross-platform filesystem paths
- Unicode support for source code

## Development Tools

### LSP Server
- **Purpose**: IDE integration
- **Implementation**: Custom Language Server Protocol server
- **Features**: Diagnostics, auto-completion, hover information

### Configuration System
- `.wflcfg` files for project settings
- Global and local configuration support
- Validation with `--configCheck` and `--configFix` flags

### Debugging Tools
- Structured logging with verbosity levels
- Automatic debug reports on errors
- Interactive step-by-step execution