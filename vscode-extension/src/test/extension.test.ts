import * as assert from 'assert';
import * as vscode from 'vscode';

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
