### Goal
Fix a false positive in the static analyzer that was incorrectly flagging loop variables in RepeatWhile loops as "unused".

### Approach
Analyzed the static analyzer implementation and found that it had proper handling for traditional `WhileLoop` statements but was missing handlers for the variants `RepeatWhileLoop` and `RepeatUntilLoop`. These missing handlers were causing variables used in the conditions of these loop types to be incorrectly reported as unused.

The solution was to add explicit handlers for both `RepeatWhileLoop` and `RepeatUntilLoop` in the `mark_used_variables` method, ensuring that variables used in their conditions are properly marked as used.

### Gotchas
This issue highlights the importance of systematically testing all statement variants when implementing static analysis features. When new language constructs are added, we need to make sure they're properly integrated into all components of the static analyzer.

The bug was particularly subtle because:
1. It only appeared with `repeat while` syntax but not with the standard `while` syntax
2. The variable was actually used in the condition and body, making it even more confusing for users
3. This highlights our commitment to backward compatibility - users shouldn't have to change their code to work with our tools

### Outcome
Successfully fixed the false positive. The static analyzer now correctly recognizes variables used in all loop type conditions.