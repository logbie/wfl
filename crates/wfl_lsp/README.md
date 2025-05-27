# WFL Language Server Protocol (LSP) Implementation

This crate provides the Language Server Protocol implementation for the WebFirst Language (WFL), enabling IDE integration with syntax highlighting, error reporting, and code completion.

## Usage

### As a standalone LSP server

```bash
# Install the LSP server
cargo install wfl_lsp

# Run the server (will communicate over stdin/stdout)
wfl-lsp
```

### As a dependency with lsp-bridge feature

When used as a dependency in the `wfl_editor` crate, this LSP implementation is available behind the optional `lsp-bridge` feature flag. This design keeps the editor binary lean for users who only want embedded editing without LSP out-of-process reuse.

```toml
# In your Cargo.toml
[dependencies]
wfl_editor = { version = "0.1.0", features = ["lsp-bridge"] }
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `lsp-bridge` | Enables the LSP façade for integration with external editors | No |

The `lsp-bridge` feature allows VS Code, Helix, or Neovim to reuse the same logic without embedding the GUI. When this feature is enabled, the `wfl_editor::lsp::Server` struct provides a façade that wraps this crate's functionality.

## Implementation Details

The LSP server provides:
- Syntax highlighting based on WFL's lexer
- Error reporting from the parser, analyzer, and type checker
- Code completion for keywords and in-file symbols
- Hover information for symbols

For more information on the WebFirst Language, visit the [main repository](https://github.com/WebFirstLanguage/wfl).
