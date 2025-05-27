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
exports.WflFormatter = void 0;
const vscode = __importStar(require("vscode"));
const cp = __importStar(require("child_process"));
const path = __importStar(require("path"));
const fs = __importStar(require("fs"));
const os = __importStar(require("os"));
/**
 * WflFormatter integrates with the WFL CLI to provide enhanced formatting
 * when the WFL toolchain is available.
 */
class WflFormatter {
    constructor(wflPath) {
        this.wflPath = wflPath;
    }
    /**
     * Format the entire document using the WFL CLI
     */
    async provideDocumentFormattingEdits(document, _options // Unused parameter
    ) {
        return this.formatDocument(document);
    }
    /**
     * Format a specific range in the document
     * Note: WFL CLI doesn't directly support range formatting,
     * so we extract the range, format it, and reintegrate.
     */
    async provideDocumentRangeFormattingEdits(document, range, _options // Unused parameter
    ) {
        // Extract the text in the range
        const text = document.getText(range);
        // Create a temporary file for the range
        const tempFile = path.join(os.tmpdir(), `wfl-format-${Date.now()}.wfl`);
        fs.writeFileSync(tempFile, text);
        try {
            // Format the temporary file
            const formattedText = await this.formatFile(tempFile);
            // If nothing changed, return empty array
            if (text === formattedText) {
                return [];
            }
            return [vscode.TextEdit.replace(range, formattedText)];
        }
        catch (error) {
            vscode.window.showErrorMessage(`WFL formatting failed: ${String(error)}`);
            return [];
        }
        finally {
            // Clean up the temporary file
            try {
                fs.unlinkSync(tempFile);
            }
            catch (e) {
                // Ignore cleanup errors
            }
        }
    }
    /**
     * Format an entire document
     */
    async formatDocument(document) {
        // For unsaved documents, create a temporary file
        if (document.isUntitled) {
            return this.formatUntitledDocument(document);
        }
        try {
            // Format the file directly
            const formattedText = await this.formatFile(document.fileName);
            const text = document.getText();
            // If nothing changed, return empty array
            if (text === formattedText) {
                return [];
            }
            const fullRange = new vscode.Range(0, 0, document.lineCount - 1, document.lineAt(document.lineCount - 1).text.length);
            return [vscode.TextEdit.replace(fullRange, formattedText)];
        }
        catch (error) {
            vscode.window.showErrorMessage(`WFL formatting failed: ${String(error)}`);
            return [];
        }
    }
    /**
     * Format an untitled document by creating a temporary file
     */
    async formatUntitledDocument(document) {
        const text = document.getText();
        // Create a temporary file
        const tempFile = path.join(os.tmpdir(), `wfl-format-${Date.now()}.wfl`);
        fs.writeFileSync(tempFile, text);
        try {
            // Format the temporary file
            const formattedText = await this.formatFile(tempFile);
            // If nothing changed, return empty array
            if (text === formattedText) {
                return [];
            }
            const fullRange = new vscode.Range(0, 0, document.lineCount - 1, document.lineAt(document.lineCount - 1).text.length);
            return [vscode.TextEdit.replace(fullRange, formattedText)];
        }
        catch (error) {
            vscode.window.showErrorMessage(`WFL formatting failed: ${String(error)}`);
            return [];
        }
        finally {
            // Clean up the temporary file
            try {
                fs.unlinkSync(tempFile);
            }
            catch (e) {
                // Ignore cleanup errors
            }
        }
    }
    /**
     * Execute the WFL CLI to format a file
     */
    async formatFile(filePath) {
        return new Promise((resolve, reject) => {
            // Build the WFL CLI command
            const args = [
                '--lint',
                '--fix',
                '--diff', // Use diff to get the formatted output
                filePath
            ];
            // Add configuration options if specified
            const config = vscode.workspace.getConfiguration('wfl.format');
            if (config.has('indentSize')) {
                args.push('--indent-size', config.get('indentSize', 4).toString());
            }
            if (config.has('maxLineLength')) {
                args.push('--max-line-length', config.get('maxLineLength', 80).toString());
            }
            // Execute the WFL CLI
            const process = cp.spawn(this.wflPath, args);
            let stdout = '';
            let stderr = '';
            process.stdout.on('data', (data) => {
                stdout += data.toString();
            });
            process.stderr.on('data', (data) => {
                stderr += data.toString();
            });
            process.on('close', (code) => {
                if (code !== 0) {
                    reject(stderr || `Process exited with code ${code}`);
                    return;
                }
                // Parse the diff output to get the formatted text
                try {
                    const formatted = this.extractFormattedTextFromDiff(stdout, fs.readFileSync(filePath, 'utf8'));
                    resolve(formatted);
                }
                catch (error) {
                    reject(`Failed to parse WFL formatter output: ${error}`);
                }
            });
            process.on('error', (err) => {
                reject(`Failed to start WFL formatter: ${err.message}`);
            });
        });
    }
    /**
     * Extract the formatted text from a unified diff output
     */
    extractFormattedTextFromDiff(diffOutput, originalText) {
        // If there's no diff output, the file was already formatted
        if (!diffOutput.trim()) {
            return originalText;
        }
        // Simple implementation - in a real extension, we would use a proper diff parser
        // For now, just extract the formatted text sections (lines starting with '+')
        // and skip the context and removal lines (lines starting with '-' or ' ')
        // This is a simplified approach that works for basic diffs
        // In a production extension, use a proper diff parser library
        const lines = diffOutput.split('\n');
        const formattedLines = [];
        // Skip the diff header lines
        let headerDone = false;
        for (const line of lines) {
            // Skip until we find a line starting with '+++'
            if (!headerDone) {
                if (line.startsWith('+++')) {
                    headerDone = true;
                }
                continue;
            }
            // Skip diff metadata lines (starting with @@)
            if (line.startsWith('@@')) {
                continue;
            }
            // Include added lines (without the '+' prefix)
            if (line.startsWith('+')) {
                formattedLines.push(line.substring(1));
            }
            // Skip removed lines and context lines
        }
        return formattedLines.join('\n');
    }
}
exports.WflFormatter = WflFormatter;
//# sourceMappingURL=wfl-formatter.js.map