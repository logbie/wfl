# WFL Debugging Guide

## Debug Reports

When a runtime error occurs, WFL generates a debug report with detailed information about the error and the execution context. This helps diagnose and fix issues in your WFL scripts.

### Configuration Options

Debug reports can be configured in your WFL configuration file:

```
# Enable/disable debug reports (default: true)
debug_report_enabled = true

# Enable/disable full debug reports with complete local variable capture (default: false)
debug_full_report = false

# Maximum memory usage in MB before throwing OutOfMemory error (default: 512)
max_memory_mb = 512
```

### Debug Report Contents

A debug report includes:

- Error message and location
- Call stack with function names and line numbers
- Local variables at each call frame (limited by memory constraints)
- Source code snippets around error locations

### Memory Management

WFL implements memory tracking to prevent excessive memory usage:

- Memory usage is tracked for large data structures (Lists, Objects, large Text values)
- If memory usage exceeds `max_memory_mb`, an OutOfMemory error is thrown
- For OutOfMemory errors, debug reports are simplified to avoid recursive memory issues
- Each call frame has a limit on captured local variable size (default: 32 KiB)

### Optimizing Memory Usage

If your script encounters memory issues:

1. Reduce the size of large data structures
2. Avoid deeply nested function calls with large local variables
3. Use smaller text strings where possible
4. Increase `max_memory_mb` if your system has sufficient RAM
5. Set `debug_full_report = false` to reduce memory usage during error reporting
