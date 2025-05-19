# WebFirst Language (WFL) VS Code Extension

This extension provides support for the WebFirst Language (WFL) in Visual Studio Code.

## Features

- Syntax highlighting for WFL files
- Autocompletion and snippets for WFL keywords and constructs
- Go-to-definition and find-all-references support
- Real-time diagnostics to catch errors as you type
- Hover information for symbols

## Requirements

- VS Code 1.80.0 or higher
- WFL Language Server (`wfl-lsp` executable)

## Setup

### Quick Start

1. Install the extension from the VS Code marketplace
2. If you already have `wfl-lsp` in your PATH, you're all set!
3. Otherwise, use the "WFL: Select LSP Executable…" command to select your `wfl-lsp` executable

### Manual Configuration

You can manually configure the extension in VS Code settings:

- `wfl-lsp.serverPath`: Path to the WFL language server executable
- `wfl-lsp.serverArgs`: Additional arguments to pass to the server
- `wfl-lsp.versionMode`: How to handle version mismatches (warn/block/ignore)

## Commands

- **WFL: Restart Language Server**: Restart the language server if it's not working correctly
- **WFL: Select LSP Executable…**: Open a file dialog to select the WFL language server executable

## Building the WFL Language Server

If you don't have the WFL language server, you can build it from source:

```bash
git clone https://github.com/WebFirstLanguage/wfl.git
cd wfl
cargo build -p wfl-lsp
```

The executable will be available at `./target/debug/wfl-lsp`.

## Troubleshooting

- If you see "WFL LSP Server version is incompatible" warnings, make sure you're using a compatible version of the language server
- If the server doesn't start, check that the executable path is correct and that you have the necessary permissions
- If features like autocompletion or go-to-definition aren't working, try restarting the language server with the "WFL: Restart Language Server" command

## Development

### Building the Extension

To build the extension from source:

```bash
git clone https://github.com/WebFirstLanguage/wfl.git
cd wfl/vscode-wfl
npm install
npm run compile
```

### Testing the Extension

To test the extension:

```bash
npm test
```

### Packaging the Extension

To package the extension for distribution:

```bash
npm run vscode:prepublish
npx vsce package
```

This will create a `.vsix` file that can be installed in VS Code.

## License

This extension is released under the MIT License.
