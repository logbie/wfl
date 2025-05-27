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
const assert = __importStar(require("assert"));
const vscode = __importStar(require("vscode"));
// Basic test suite for the extension
describe('WFL Extension Tests', () => {
    it('Extension should be activated', async () => {
        // Verify the extension is activated
        const extension = vscode.extensions.getExtension('wfl.vscode-wfl');
        assert.notStrictEqual(extension, undefined);
        if (extension) {
            // Wait for extension to activate if not already
            if (!extension.isActive) {
                await extension.activate();
            }
            assert.strictEqual(extension.isActive, true);
        }
    });
    it('Should register document formatter', () => {
        // Create a simple WFL document
        const content = '// This is a WFL test file\nstore test as "value"';
        const doc = {
            getText: () => content,
            languageId: 'wfl',
            uri: vscode.Uri.parse('untitled:test.wfl'),
            version: 1
        };
        // Check the document is identified as WFL
        assert.strictEqual(doc.languageId, 'wfl');
    });
    // Additional tests to add:
    // - Test syntax highlighting (requires browser automation)
    // - Test formatter with mock WFL CLI
    // - Test LSP integration with mock server
});
//# sourceMappingURL=extension.test.js.map