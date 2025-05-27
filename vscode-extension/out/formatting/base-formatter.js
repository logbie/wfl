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
exports.BaseFormatter = void 0;
const vscode = __importStar(require("vscode"));
/**
 * BaseFormatter provides independent formatting functionality for WFL
 * without requiring the WFL toolchain to be installed.
 */
class BaseFormatter {
    /**
     * Format the entire document
     */
    provideDocumentFormattingEdits(document, options) {
        return this.formatDocument(document, options);
    }
    /**
     * Format a specific range in the document
     */
    provideDocumentRangeFormattingEdits(document, range, options) {
        // Extract the text in the range
        const text = document.getText(range);
        // Format just this range
        const lines = text.split(/\r?\n/);
        const formattedLines = this.formatLines(lines, options);
        const formattedText = formattedLines.join('\n');
        // If nothing changed, return empty array
        if (text === formattedText) {
            return [];
        }
        return [vscode.TextEdit.replace(range, formattedText)];
    }
    /**
     * Format an entire document
     */
    formatDocument(document, options) {
        const text = document.getText();
        const lines = text.split(/\r?\n/);
        const formattedLines = this.formatLines(lines, options);
        const formattedText = formattedLines.join('\n');
        // If nothing changed, return empty array
        if (text === formattedText) {
            return [];
        }
        const fullRange = new vscode.Range(0, 0, document.lineCount - 1, document.lineAt(document.lineCount - 1).text.length);
        return [vscode.TextEdit.replace(fullRange, formattedText)];
    }
    /**
     * Format an array of lines
     */
    formatLines(lines, options) {
        // Track indentation level as we process lines
        let indentLevel = 0;
        const tabSize = options.tabSize;
        const insertSpaces = options.insertSpaces;
        const indent = insertSpaces ? ' '.repeat(tabSize) : '\t';
        // Process each line
        return lines.map((line) => {
            // Trim trailing whitespace
            let trimmedLine = line.trimRight();
            // Skip empty lines or comment-only lines
            if (trimmedLine === '' || trimmedLine.trim().startsWith('//') || trimmedLine.trim().startsWith('/*')) {
                return trimmedLine;
            }
            // Check if this line should decrease the indent level before indenting
            // (for 'end', 'otherwise', 'else', etc.)
            if (this.isDecrementIndentLine(trimmedLine)) {
                indentLevel = Math.max(0, indentLevel - 1);
            }
            // Apply current indentation
            const indentedLine = indent.repeat(indentLevel) + trimmedLine.trimLeft();
            // Check if this line should increase the indent level for the next line
            // (for lines ending with ':')
            if (this.isIncrementIndentLine(trimmedLine)) {
                indentLevel++;
            }
            // Format operators with spaces
            return this.formatOperators(indentedLine);
        });
    }
    /**
     * Check if a line should increase the indent level
     */
    isIncrementIndentLine(line) {
        // Check for block start patterns (ending with ':')
        return /:\s*$/.test(line) && !this.isCommentLine(line);
    }
    /**
     * Check if a line should decrease the indent level
     */
    isDecrementIndentLine(line) {
        // Check for block end patterns like "end if", "end action", "otherwise", etc.
        const matches = line.match(/^\s*(end\s+\w+|otherwise|else|when)\b/);
        return !!matches && !this.isCommentLine(line);
    }
    /**
     * Check if a line is a comment line
     */
    isCommentLine(line) {
        const trimmed = line.trim();
        return trimmed.startsWith('//') || trimmed.startsWith('/*');
    }
    /**
     * Format operators with proper spacing
     */
    formatOperators(line) {
        // Regular expression to add spaces around operators
        // but preserve spaces in strings
        // Handle arithmetic operators
        let formatted = line
            .replace(/([^"'])\+([^"'])/g, '$1 + $2')
            .replace(/([^"'])-([^"'])/g, '$1 - $2')
            .replace(/([^"'])\*([^"'])/g, '$1 * $2')
            .replace(/([^"'])\/([^"'])/g, '$1 / $2');
        // Handle comparison operators
        formatted = formatted
            .replace(/([^"'])=([^"'])/g, '$1 = $2')
            .replace(/([^"'])<([^"'])/g, '$1 < $2')
            .replace(/([^"'])>([^"'])/g, '$1 > $2');
        // Handle natural language operators
        formatted = formatted
            .replace(/\b(is|not|and|or)\b/g, ' $1 ')
            .replace(/\s{2,}/g, ' '); // Replace multiple spaces with a single space
        return formatted;
    }
}
exports.BaseFormatter = BaseFormatter;
//# sourceMappingURL=base-formatter.js.map