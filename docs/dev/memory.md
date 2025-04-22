# Memory Management in WFL

This document describes the memory management strategies used in the WFL interpreter and parser.

## Reference Cycles and Weak References

The WFL interpreter uses a combination of `Rc<T>` (reference-counted pointers) and `Weak<T>` (weak references) to manage memory efficiently and prevent memory leaks.

### Environment ↔ FunctionValue Cycle

A key potential source of memory leaks is the relationship between `Environment` and `FunctionValue`:

- Functions capture their defining environment to implement closures
- Environments contain function values

This creates a reference cycle that would prevent memory from being freed if both used strong references (`Rc<T>`).

To break this cycle, `FunctionValue` uses a `Weak<RefCell<Environment>>` reference to its environment:

```rust
pub struct FunctionValue {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
    pub env: Weak<RefCell<Environment>>, // Weak reference breaks the cycle
    pub line: usize,
    pub column: usize,
}
```

When creating a function value, we use `Rc::downgrade(&env)` to create a weak reference:

```rust
let function = FunctionValue {
    name: Some(name.clone()),
    params: param_names,
    body: body.clone(),
    env: Rc::downgrade(&env), // Convert strong reference to weak
    line: *line,
    column: *column,
};
```

### Other Potential Cycles

Other areas where weak references are used to prevent cycles:

- Object methods that reference their parent object
- Callbacks that capture their environment
- Event handlers that reference their source

## Parser Memory Optimizations

The parser is optimized to minimize memory allocations:

### Token Handling

- Identifiers use string interning with `Arc<str>` to deduplicate strings
- Parser functions use references to tokens instead of cloning them
- Peek operations return references rather than cloned values

### Vector Growth

- Collections pre-allocate capacity using `Vec::with_capacity()` or `reserve()`
- Parameter lists, argument lists, and statement blocks use capacity hints
- Reuse vectors where possible to avoid repeated allocations

### String Interning

The lexer uses a global string intern pool to deduplicate identifiers and keywords:

```rust
static INTERN_POOL: Lazy<Mutex<HashSet<Arc<str>>>> = Lazy::new(|| {
    Mutex::new(HashSet::new())
});
```

Note: The intern pool intentionally "leaks" unique strings (they remain allocated for the program's lifetime). This is a standard trade-off for string interning that improves overall memory usage by eliminating duplicate strings.

## Debug Output Truncation

To prevent memory explosions when formatting large data structures, the `SafeDebug` trait implements truncation:

- Lists and objects truncate after 16 elements with a count of remaining items
- Long strings truncate after 128 characters with an ellipsis
- Truncation is UTF-8 safe to prevent invalid character sequences

## Call Stack Hygiene

The interpreter maintains a call stack for debugging and error reporting:

- Call frames are popped on function return
- After generating a debug report, the call stack is explicitly cleared
- Invariant checks verify call stack consistency

## Memory Leak Testing

Memory leak tests verify:

- Closure cycles are properly broken (reference counts return to 1)
- Parser memory usage remains stable across multiple parses
- Debug truncation correctly limits output size
