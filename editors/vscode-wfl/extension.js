const { workspace, ExtensionContext } = require('vscode');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');
const path = require('path');

let client;

function activate(context) {
  const serverOptions = {
    command: workspace.getConfiguration().get('wfl.serverPath', 'wfl-lsp'),
    transport: TransportKind.stdio
  };

  const clientOptions = {
    documentSelector: [{ scheme: 'file', language: 'wfl' }],
    synchronize: {
      configurationSection: 'wfl'
    }
  };

  client = new LanguageClient(
    'wfl',
    'WFL Language Server',
    serverOptions,
    clientOptions
  );

  client.start();
}

function deactivate() {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

module.exports = { activate, deactivate };
