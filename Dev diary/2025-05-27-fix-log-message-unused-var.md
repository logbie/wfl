### Goal
- Resolve the `ANALYZE-UNUSED` warning for the `message_text` variable in the `log_message` action in `Nexus/nexus_dev.wfl`.

### Approach
1.  Consult WFL documentation (`Docs/wfl-IO.md`, `Docs/wfl-vars.md`) to verify the correct syntax for variable usage within the `append content` statement.
2.  If the syntax in `Nexus/nexus_dev.wfl` is incorrect, update it.
3.  If the syntax is correct, investigate the static analyzer (`src/analyzer/static_analyzer.rs`) for a potential bug in tracking variable usage.
4.  Ensure the fix makes the static analysis warning disappear.
5.  Add a regression test to `Test Programs/` to prevent this issue from recurring.

### Gotchas
- The `append content` statement might have specific requirements for how variables are interpolated or referenced.
- If it's an analyzer bug, fixing it might involve changes to Rust code in `src/analyzer/static_analyzer.rs`.

### Outcome
- **Fixed**: Updated `src/analyzer/static_analyzer.rs` to properly track variable usage within action definitions and I/O statements.
- **Root Cause**: The static analyzer was not recursively checking variable usage within `ActionDefinition` bodies and was missing several statement types like `WriteFileStatement`, `ActionCall` expressions, and `Concatenation` expressions.
- **Changes Made**:
  1. Added `Statement::ActionDefinition { body, .. } => { for stmt in body { self.mark_used_variables(stmt, usages); } }` to `mark_used_variables()`
  2. Added handlers for I/O statements (`WriteFileStatement`, `OpenFileStatement`, etc.) in `mark_used_variables()`
  3. Added `Expression::ActionCall { arguments, .. }` handler in `mark_used_in_expression()`
- **Test Result**: `cargo run -- --analyze Nexus/nexus_dev.wfl` now returns "No static analysis warnings found" - the `message_text` parameter is correctly detected as used within the concatenation expression.
