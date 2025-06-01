### Goal  
Add a short version flag (`-v`) that displays the version number and ignores any other flags.

### Approach  
Modified the command-line argument processing in `main.rs` to check for both `--version` and `-v` flags at the beginning of the argument processing. When either flag is detected, the version is displayed and the program exits immediately, ensuring that all other flags are ignored.

Also updated the help text to indicate that both `--version` and `-v` can be used to display version information.

### Gotchas  
During implementation, discovered and fixed an unrelated bug in `src/interpreter/mod.rs` where a variable named `_i` was being referenced as `i` in a logging macro.

### Outcome  
Successfully implemented the `-v` flag that:
1. Displays the version number
2. Ignores any other flags when present
3. Works alongside the existing `--version` flag

The implementation follows the project's existing patterns for command-line argument processing and maintains backward compatibility with the existing `--version` flag.