import * as path from 'path';
import * as vscode from 'vscode';
import * as child_process from 'child_process';
import * as semver from 'semver';
import { 
  LanguageClient, 
  LanguageClientOptions, 
  ServerOptions, 
  TransportKind 
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
  console.log('WebFirst Language (WFL) extension is now active');

  context.subscriptions.push(
    vscode.commands.registerCommand('wfl.restartLanguageServer', () => {
      restartClient(context);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('wfl.selectLspExecutable', async () => {
      const options: vscode.OpenDialogOptions = {
        canSelectMany: false,
        openLabel: 'Select WFL LSP Server Executable',
        filters: {
          'Executables': ['exe', '*'],
        }
      };

      const fileUri = await vscode.window.showOpenDialog(options);
      if (fileUri && fileUri[0]) {
        const config = vscode.workspace.getConfiguration('wfl-lsp');
        await config.update('serverPath', fileUri[0].fsPath, vscode.ConfigurationTarget.Global);
        vscode.window.showInformationMessage(`WFL LSP Server path set to: ${fileUri[0].fsPath}`);
        restartClient(context);
      }
    })
  );

  startClient(context);
}

async function startClient(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration('wfl-lsp');
  const serverPath = config.get<string>('serverPath', 'wfl-lsp');
  const serverArgs = config.get<string[]>('serverArgs', []);
  const versionMode = config.get<string>('versionMode', 'warn');

  const versionCompatible = await checkLspVersion(serverPath, versionMode);
  if (!versionCompatible && versionMode === 'block') {
    vscode.window.showErrorMessage(
      'WFL LSP Server version is incompatible. Server will not start. Change version mode or use a compatible server version.'
    );
    return;
  }

  const serverOptions: ServerOptions = {
    command: serverPath,
    args: serverArgs,
    transport: TransportKind.stdio
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'wfl' }],
    synchronize: {
      configurationSection: 'wfl-lsp',
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.wfl')
    },
    outputChannelName: 'WFL Language Server'
  };

  client = new LanguageClient(
    'wfl-language-server',
    'WFL Language Server',
    serverOptions,
    clientOptions
  );

  client.start();
  context.subscriptions.push(client);
}

async function restartClient(context: vscode.ExtensionContext) {
  if (client) {
    await client.stop();
    client.dispose();
    client = undefined;
  }
  
  startClient(context);
  vscode.window.showInformationMessage('WFL Language Server restarted');
}

async function checkLspVersion(serverPath: string, versionMode: string): Promise<boolean> {
  return new Promise((resolve) => {
    try {
      // Define the expected semver range from package.json
      const requiredVersionRange = vscode.extensions.getExtension('wfl.vscode-wfl')?.packageJSON?.engines?.wflLspServer || '>=0.1.0 <1.0.0';
      
      const process = child_process.spawn(serverPath, ['--version', '--quiet']);
      let output = '';
      
      process.stdout.on('data', (data) => {
        output += data.toString();
      });
      
      process.on('close', (code) => {
        if (code !== 0) {
          if (versionMode === 'warn') {
            vscode.window.showWarningMessage(`Failed to check WFL LSP Server version. Exit code: ${code}`);
          }
          resolve(versionMode !== 'block'); // Only block if mode is 'block'
          return;
        }
        
        const versionMatch = output.trim().match(/(\d+\.\d+\.\d+)/);
        if (!versionMatch) {
          if (versionMode === 'warn') {
            vscode.window.showWarningMessage('Could not determine WFL LSP Server version.');
          }
          resolve(versionMode !== 'block');
          return;
        }
        
        const serverVersion = versionMatch[1];
        const isCompatible = semver.satisfies(serverVersion, requiredVersionRange);
        
        if (!isCompatible && versionMode === 'warn') {
          vscode.window.showWarningMessage(
            `WFL LSP Server version ${serverVersion} does not satisfy the required range ${requiredVersionRange}.`
          );
        }
        
        resolve(isCompatible || versionMode === 'ignore');
      });
      
      process.on('error', (err) => {
        if (versionMode === 'warn') {
          vscode.window.showWarningMessage(`Failed to execute WFL LSP Server: ${err.message}`);
        }
        resolve(versionMode !== 'block');
      });
    } catch (err: any) {
      if (versionMode === 'warn') {
        vscode.window.showWarningMessage(`Error checking WFL LSP Server version: ${err.message}`);
      }
      resolve(versionMode !== 'block');
    }
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
