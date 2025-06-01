### Goal  
Fix compilation errors and warnings that were preventing the MSI build from completing successfully.

### Approach  
1. Replaced the missing `exec_trace_always!` macro with the existing `exec_trace!` macro
2. Prefixed unused variables with underscores to indicate they are intentionally unused

### Specific Changes
1. In `src/interpreter/mod.rs:822`:
   - Changed `crate::exec_trace_always!("Count loop: resetting state before evaluation")` to 
   - `crate::exec_trace!("Count loop: resetting state before evaluation")`
   - The `exec_trace_always!` macro exists in logging.rs but isn't re-exported in lib.rs

2. In `src/interpreter/mod.rs:2174`:
   - Changed `for (i, (param, arg)) in func.params.iter().zip(args.clone()).enumerate() {` to
   - `for (_i, (param, arg)) in func.params.iter().zip(args.clone()).enumerate() {`

3. In `src/parser/mod.rs:1883-1888`:
   - Changed `let keyword = ...` to `let _keyword = ...`
   - Updated the reference in the exec_trace! call to use `_keyword`

### Outcome  
Successfully built the MSI installer at target/x86_64-pc-windows-msvc/release/wfl-2025.30.msi without any compilation errors or warnings.