# Memory Profiling with dhat-rs in WFL

This document explains how to use dhat-rs memory profiling in the WebFirst Language (WFL) project.

## Overview

WFL integrates with [dhat-rs](https://docs.rs/dhat/), a Rust heap profiling library, to provide:

1. **Heap profiling**: Track all allocations with the `dhat-heap` feature
2. **Ad-hoc profiling**: Instrument specific hotspots with the `dhat-ad-hoc` feature
3. **Heap usage tests**: Verify memory usage in CI

## Running Profiling

### Heap Profiling

```bash
cargo run --features dhat-heap --release
```

This will generate a `dhat-heap.json` file in the project root that contains detailed information about all heap allocations.

### Ad-hoc Profiling 

```bash
cargo run --features dhat-ad-hoc --release
```

This will generate a `dhat-ad-hoc.json` file in the project root that contains information about the instrumented hotspots.

## Viewing Results

The JSON files can be viewed using the [online dhat viewer](https://nnethercote.github.io/dh_view.html):

1. Open the viewer in your web browser
2. Load the generated JSON file
3. Explore the allocation tree and hotspots

## Running Memory Tests

```bash
cargo test --features dhat-heap --release -- --test-threads 1 memory_usage
```

These tests verify that memory usage remains within expected limits. The single-threaded execution ensures consistent memory measurements.

## Implementation Details

- **Global allocator** is defined in `src/lib.rs`
- **Profiler initialization** happens in `src/main.rs` and `src/repl.rs`
- **Ad-hoc instrumentation** points are placed in key memory hotspots:
  - `Interpreter::call_function` - Function call operations
  - `Environment::new_global` - Global environment creation
  - `Environment::new` - Environment creation with parent
  - `Environment::new_child_env` - Child environment creation
- **Memory usage tests** are in `tests/memory_usage.rs`

## Notes for Windows Users

- Windows debugging information may cause slower performance
- Set `debug = 1` in the release profile (already configured in Cargo.toml)
- Always run with `--release` flag for reasonable performance
- CI includes Windows-specific memory tests to ensure cross-platform compatibility

## Troubleshooting

If the profiler seems to be running too slowly:
- Always use `--release` mode
- Ensure you're not capturing too many ad-hoc events
- For large programs, consider using targeted profiling of specific sections

If the profiler isn't generating output:
- Ensure the `_profiler` binding is kept alive for the entire program
- Check that you're using the correct feature flag
- Verify file permissions in the output directory

If you encounter unexpected memory usage:
- Check for reference cycles between Environment and FunctionValue
- Verify that weak references are used appropriately
- Run memory tests with `RUST_BACKTRACE=1` for more detailed allocation information
