"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const cp = __importStar(require("child_process"));
const node_1 = require("vscode-languageclient/node");
const base_formatter_1 = require("./formatting/base-formatter");
const wfl_formatter_1 = require("./formatting/wfl-formatter");
// State tracking
let client;
let wflAvailable = false;
let wflPath;
let lspAvailable = false;
/**
 * Main extension activation
 */
function activate(context) {
    const outputChannel = vscode.window.createOutputChannel('WFL');
    outputChannel.appendLine('WFL extension is now active');
    // Detect WFL tools availability
    detectWflAvailability().then(({ wflFound, lspFound, wflCliPath }) => {
        wflAvailable = wflFound;
        lspAvailable = lspFound;
        wflPath = wflCliPath;
        outputChannel.appendLine(`WFL CLI available: ${wflAvailable}`);
        outputChannel.appendLine(`WFL LSP available: ${lspAvailable}`);
        if (wflAvailable) {
            outputChannel.appendLine(`WFL CLI path: ${wflPath}`);
        }
        // Register formatters
        registerFormatters(context);
        // Start LSP client if available
        if (lspAvailable) {
            startLspClient(context, outputChannel);
        }
        // Register commands
        registerCommands(context, outputChannel);
    });
}
/**
 * Detect WFL CLI and LSP availability
 */
async function detectWflAvailability() {
    const config = vscode.workspace.getConfiguration('wfl');
    const cliConfig = config.get('cli', { path: 'wfl', autoDetect: true });
    if (!cliConfig.autoDetect) {
        // Use the configured path directly
        return {
            wflFound: await checkWflCli(cliConfig.path),
            lspFound: await checkWflLsp(config.get('serverPath', 'wfl-lsp')),
            wflCliPath: cliConfig.path
        };
    }
    // Try to auto-detect WFL CLI
    // First, try the configured path
    if (await checkWflCli(cliConfig.path)) {
        // Also check for LSP
        return {
            wflFound: true,
            lspFound: await checkWflLsp(config.get('serverPath', 'wfl-lsp')),
            wflCliPath: cliConfig.path
        };
    }
    // Try to find in PATH
    const isWindows = process.platform === 'win32';
    const wflCmd = isWindows ? 'wfl.exe' : 'wfl';
    try {
        // Try to find in PATH using 'which' on Unix or 'where' on Windows
        const whichCmd = isWindows ? 'where' : 'which';
        const { stdout } = await execPromise(whichCmd, [wflCmd]);
        if (stdout.trim()) {
            const detectedPath = stdout.trim().split('\n')[0];
            return {
                wflFound: true,
                lspFound: await checkWflLsp(config.get('serverPath', 'wfl-lsp')),
                wflCliPath: detectedPath
            };
        }
    }
    catch {
        // Command failed, WFL CLI not found in PATH
    }
    // Not found
    return {
        wflFound: false,
        lspFound: await checkWflLsp(config.get('serverPath', 'wfl-lsp')),
        wflCliPath: undefined
    };
}
/**
 * Check if WFL CLI is available
 */
async function checkWflCli(path) {
    try {
        const { stdout } = await execPromise(path, ['--version']);
        // If we got a version output, WFL CLI is available
        return stdout.trim().startsWith('wfl version');
    }
    catch {
        return false;
    }
}
/**
 * Check if WFL LSP server is available
 */
async function checkWflLsp(path) {
    try {
        const { stdout } = await execPromise(path, ['--version']);
        // If we got a version output, WFL LSP is available
        return stdout.trim().startsWith('wfl-lsp version');
    }
    catch {
        return false;
    }
}
/**
 * Promise-based exec
 */
function execPromise(command, args) {
    return new Promise((resolve, reject) => {
        let stdout = '';
        let stderr = '';
        const process = cp.spawn(command, args);
        process.stdout.on('data', (data) => {
            stdout += data.toString();
        });
        process.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        process.on('close', (code) => {
            if (code === 0) {
                resolve({ stdout, stderr });
            }
            else {
                reject(new Error(`Command failed with code ${code}: ${stderr}`));
            }
        });
        process.on('error', (err) => {
            reject(err);
        });
    });
}
/**
 * Register document formatters
 */
function registerFormatters(context) {
    const config = vscode.workspace.getConfiguration('wfl.format');
    const formatEnabled = config.get('enable', true);
    if (!formatEnabled) {
        return;
    }
    // Determine which formatter to use
    const provider = config.get('provider', 'auto');
    const useWflFormatter = (provider === 'wfl' || (provider === 'auto' && wflAvailable)) && !!wflPath;
    // Register the selected formatter
    if (useWflFormatter && wflPath) {
        // Register WFL CLI-based formatter
        const wflFormatter = new wfl_formatter_1.WflFormatter(wflPath);
        context.subscriptions.push(vscode.languages.registerDocumentFormattingEditProvider('wfl', wflFormatter), vscode.languages.registerDocumentRangeFormattingEditProvider('wfl', wflFormatter));
        // Register format on save if enabled
        if (config.get('formatOnSave', true)) {
            context.subscriptions.push(vscode.workspace.onWillSaveTextDocument((event) => {
                if (event.document.languageId === 'wfl') {
                    event.waitUntil(vscode.commands.executeCommand('editor.action.formatDocument'));
                }
            }));
        }
        // Register format on type if enabled
        if (config.get('formatOnType', false)) {
            context.subscriptions.push(vscode.languages.registerOnTypeFormattingEditProvider('wfl', {
                provideOnTypeFormattingEdits: (document, position, ch, options) => {
                    // Only format on specific characters like ':'
                    if (ch === ':' || ch === '\n') {
                        const range = new vscode.Range(new vscode.Position(position.line, 0), position);
                        return wflFormatter.provideDocumentRangeFormattingEdits(document, range, options);
                    }
                    return [];
                }
            }, ':', '\n'));
        }
    }
    else {
        // Register built-in formatter
        const baseFormatter = new base_formatter_1.BaseFormatter();
        context.subscriptions.push(vscode.languages.registerDocumentFormattingEditProvider('wfl', baseFormatter), vscode.languages.registerDocumentRangeFormattingEditProvider('wfl', baseFormatter));
        // Register format on save if enabled
        if (config.get('formatOnSave', true)) {
            context.subscriptions.push(vscode.workspace.onWillSaveTextDocument((event) => {
                if (event.document.languageId === 'wfl') {
                    event.waitUntil(vscode.commands.executeCommand('editor.action.formatDocument'));
                }
            }));
        }
    }
}
/**
 * Start the LSP client
 */
function startLspClient(context, outputChannel) {
    const config = vscode.workspace.getConfiguration('wfl');
    const serverPath = config.get('serverPath', 'wfl-lsp');
    const serverArgs = config.get('serverArgs', []);
    // Configure the server options
    const serverOptions = {
        run: {
            command: serverPath,
            args: serverArgs,
            transport: node_1.TransportKind.stdio
        },
        debug: {
            command: serverPath,
            args: [...serverArgs, '--debug'],
            transport: node_1.TransportKind.stdio
        }
    };
    // Configure the client options
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'wfl' }],
        outputChannel: outputChannel,
        synchronize: {
            configurationSection: 'wfl'
        }
    };
    // Create and start the client
    client = new node_1.LanguageClient('wfl', 'WFL Language Server', serverOptions, clientOptions);
    // Fix: client.start() returns a Promise, but we need a Disposable
    const clientDisposable = client.start();
    context.subscriptions.push({ dispose: () => { clientDisposable.then(() => { }); } });
    outputChannel.appendLine('WFL Language Server started');
}
/**
 * Register extension commands
 */
function registerCommands(context, outputChannel) {
    // Restart language server command
    context.subscriptions.push(vscode.commands.registerCommand('wfl.restartLanguageServer', async () => {
        if (client) {
            await client.stop();
            startLspClient(context, outputChannel);
            vscode.window.showInformationMessage('WFL Language Server restarted');
        }
        else if (lspAvailable) {
            startLspClient(context, outputChannel);
            vscode.window.showInformationMessage('WFL Language Server started');
        }
        else {
            vscode.window.showErrorMessage('WFL Language Server is not available');
        }
    }));
    // Select LSP executable command
    context.subscriptions.push(vscode.commands.registerCommand('wfl.selectLspExecutable', async () => {
        const options = {
            canSelectMany: false,
            title: 'Select WFL LSP Executable',
            filters: {
                'Executable Files': ['exe', 'sh', ''],
                'All Files': ['*']
            }
        };
        const fileUri = await vscode.window.showOpenDialog(options);
        if (fileUri && fileUri.length > 0) {
            const filePath = fileUri[0].fsPath;
            // Update the configuration
            await vscode.workspace.getConfiguration('wfl')
                .update('serverPath', filePath, vscode.ConfigurationTarget.Global);
            // Check if it's a valid LSP server
            const isValidLsp = await checkWflLsp(filePath);
            if (isValidLsp) {
                vscode.window.showInformationMessage('WFL LSP executable updated. Restart the language server to apply changes.');
            }
            else {
                vscode.window.showWarningMessage('The selected file does not appear to be a valid WFL LSP executable.');
            }
        }
    }));
    // Format document command
    context.subscriptions.push(vscode.commands.registerCommand('wfl.format', async () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'wfl') {
            await vscode.commands.executeCommand('editor.action.formatDocument');
        }
        else {
            vscode.window.showErrorMessage('No active WFL document to format');
        }
    }));
}
/**
 * Extension deactivation
 */
function deactivate() {
    if (client) {
        return client.stop();
    }
    return undefined;
}
//# sourceMappingURL=extension.js.map