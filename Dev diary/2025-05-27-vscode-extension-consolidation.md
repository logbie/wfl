### Goal  
Consolidate and enhance the VSCode extension for WFL to provide better syntax highlighting and auto-formatting, while ensuring it can operate independently from the WFL toolchain.

### Approach  
1. Use the TypeScript-based extension as the foundation
2. Enhance TextMate grammar for comprehensive syntax highlighting
3. Implement both built-in (independent) and WFL-based formatting
4. Design the extension to gracefully adapt based on WFL availability

This approach provides the best developer experience for both casual users (who may not have WFL installed) and power users (with full WFL toolchain).

### Gotchas  
- TextMate grammar limitations: Can't do true semantic analysis without LSP
- Maintaining formatter consistency: Independent formatter must follow the same rules as WFL's native formatter
- Configuration synchronization: Need clear UX for when features are in "independent mode" vs "enhanced mode"
- Editor extension performance: Complex TextMate grammars can impact editor performance

### Outcome  
Created a detailed design document: `wfl-extension-design.md`

Next steps:
1. Enhance TextMate grammar with comprehensive WFL syntax support
2. Implement independent formatter with indentation and alignment rules
3. Set up integration with WFL formatter when available