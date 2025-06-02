### Goal  
Fix type checking warnings in Nexus/test.wfl where the type checker isn't properly recognizing variables used in action parameters.

### Approach  
The issue was that the type checker wasn't properly recognizing variables used in action parameters, leading to false positive "undefined variable" warnings. To fix this, we needed to:

1. Ensure the `Analyzer` struct has an `action_parameters` field to store action parameters
2. Modify the `analyze_static` method to store action parameters in the analyzer
3. Add a `get_action_parameters` method to expose the action parameters
4. Add a `with_analyzer` method to create a type checker with an existing analyzer
5. Update the `infer_expression_type` method to check if a variable is an action parameter
6. Update the `Expression::ActionCall` case to filter out errors for action parameters
7. Modify the main program to pass the analyzer to the type checker and filter type checking errors

Upon reviewing the code, I found that all these changes were already implemented but not working correctly. The issue was that the action parameters were being collected and stored, but the filtering logic wasn't being applied consistently across all error paths.

The key improvements were:

1. In `src/typechecker/mod.rs`, the `infer_expression_type` method now checks if a variable is an action parameter before reporting it as undefined
2. In `src/typechecker/mod.rs`, the `Expression::ActionCall` case now checks if the action name is an action parameter
3. In `src/main.rs`, we now create the TypeChecker with the same analyzer to share action parameters
4. In `src/main.rs`, we filter out type checking errors for action parameters

This approach follows the backward compatibility principle: "The interpreter must adapt to work with existing WFL files, not the other way around." Instead of requiring users to modify their code, we've made the type checker smarter about recognizing action parameters.

### Gotchas  
- Action parameters can be space-separated (e.g., "label expected actual"), so we need to handle this case
- We need to filter errors both in the type checker and in the main program to catch all cases
- We need to be careful not to filter out legitimate errors

### Outcome  
The changes successfully fix the type checking warnings in Nexus/test.wfl. The type checker now properly recognizes variables used in action parameters and doesn't report them as undefined.

Tests:
- `cargo run -- --analyze Nexus/test.wfl` now shows "No static analysis warnings found"
- `cargo run -- ./Nexus/test.wfl` no longer shows warnings for undefined variables that are action parameters

This fix improves the developer experience by reducing false positive warnings and making the type checker more accurate.