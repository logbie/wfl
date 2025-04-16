# WFL Parser and Bytecode Compiler Compatibility Issues

## Overview

The WFL implementation consists of several components that need to work together:
- Lexer (tokenizes source code)
- Parser (transforms tokens into an AST)
- Bytecode Compiler (transforms AST into bytecode)
- Virtual Machine (executes bytecode - not yet implemented)

Currently, there are compatibility issues between the parser and bytecode compiler that prevent the system from working correctly. This document outlines these issues and suggests solutions.

## Major Compatibility Issues

### 1. Type Mismatches

#### AST Representation vs Bytecode Compiler Expectations

The parser produces AST nodes that the bytecode compiler cannot properly handle:

- **Action Definitions**: The parser's `ActionDefinition` has a `parameters` field of type `Vec<Parameter>`, but the bytecode compiler expects a different structure.
- **Container Definitions**: The parser's `ContainerDefinition` has fields for `name`, `fields`, `methods`, and `constructor`, but the compiler has a mismatch in handling these.
- **Function Calls**: The parser creates calls with `NamedArgument` struct, but the compiler expects tuples of `(String, Expression)`.

#### Missing Type Definitions

The bytecode compiler refers to several types that don't appear to be defined:
- `Value` (used in `emit_constant`)
- `BytecodeCompiler` (used in `add_function`)
- `Literal` (used in `compile_expression`)
- `Local` (used in `locals` field)
- `ActionDef` (used in `compile_action_definition`)

### 2. Missing Functions and Fields

The bytecode compiler references functions and fields that are not implemented:
- `emit_constant`
- `emit_with_operand`
- `emit_byte`
- `add_identifier`
- `begin_scope`
- `add_local`
- `define_local`
- `declare_local`
- `define_variable`
- `add_or_get_global`
- `function` (field)
- `current_line` (field)
- `functions` (field)

### 3. OpCode Issues

The `OpCode` enum needs several additions:
- `Duplicate`
- `GetProperty`
- `SetProperty`
- `GetIndex`
- `NewContainer`
- `DefineField`
- `DefineMethod`
- `Closure`
- `Null`
- `AddList`
- `ListAppend`
- `DefineGlobal`
- `GetGlobal`

## Implementation Plan

### Step 1: Fix Type Definitions

1. Create a `Value` enum to represent runtime values:
```rust
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    // Add other value types as needed
}
```

2. Define the `Local` struct for tracking local variables:
```rust
#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
    is_captured: bool,
}
```

3. Make `Compiler` struct compatible with the current parser AST by updating `compile_action_definition` and `compile_container_definition` to handle the correct types.

### Step 2: Implement Missing Functions

```rust
impl Compiler {
    // Add a missing field for the current function being compiled
    function: Function,
    // Add a field for the current line
    current_line: usize,
    // Add a field for compiled functions
    functions: Vec<Function>,

    // Emit a constant value
    fn emit_constant(&mut self, value: Value) -> Result<(), CompileError> {
        let constant_idx = self.add_constant(value);
        self.emit_byte(OpCode::Constant, constant_idx);
        Ok(())
    }

    // Emit an opcode with an operand
    fn emit_with_operand(&mut self, op: OpCode, operand: u8) {
        self.emit(op);
        self.emit_byte(operand);
    }

    // Emit a byte
    fn emit_byte(&mut self, value: u8) {
        self.function.chunk.code.push(value);
        self.function.chunk.lines.push(self.current_line);
    }

    // Begin a new scope
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    // Add a local variable
    fn add_local(&mut self, name: String) -> Result<usize, CompileError> {
        let local = Local {
            name: name.clone(),
            depth: self.scope_depth,
            is_captured: false,
        };
        self.locals.push(local);
        Ok(self.locals.len() - 1)
    }

    // Define a local variable
    fn define_local(&mut self, index: usize) {
        // Implementation depends on how locals are used
    }

    // Declare a local variable
    fn declare_local(&mut self, name: String) -> Result<usize, CompileError> {
        // Implementation depends on how locals are used
        self.add_local(name)
    }

    // Define a variable
    fn define_variable(&mut self, name: &str) -> Result<(), CompileError> {
        // Implementation depends on variable scope handling
        Ok(())
    }

    // Add or get a global variable index
    fn add_or_get_global(&mut self, name: String) -> usize {
        // Implementation depends on how globals are tracked
        0
    }
}
```

### Step 3: Update OpCode Enum

Update the `op.rs` file to include all required opcodes:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Existing opcodes
    Constant(usize),
    Pop,
    // ... other existing opcodes ...
    
    // Add missing opcodes
    Duplicate,           // Duplicate the top value on the stack
    GetProperty,         // Get a property from an object
    SetProperty,         // Set a property on an object
    GetIndex,            // Get an item from a collection by index
    NewContainer,        // Create a new container
    DefineField,         // Define a field on a container
    DefineMethod,        // Define a method on a container
    Closure,             // Create a closure
    Null,                // Push a null value onto the stack
    AddList,             // Add an item to a list
    ListAppend,          // Append an item to a list
    DefineGlobal,        // Define a global variable
    GetGlobal,           // Get a global variable
    // ... add other missing opcodes ...
}
```

### Step 4: Fix Action and Container Definitions

Update the `compile_action_definition` method to handle the `ActionDefinition` AST node:

```rust
fn compile_action_definition(&mut self, action: Statement) -> Result<(), CompileError> {
    if let Statement::ActionDefinition { name, parameters, return_type, body, is_async, is_private } = action {
        // Convert parameters from Parameter to the expected format
        let params = parameters.iter().map(|p| p.name.clone()).collect::<Vec<_>>();
        
        // Create a new function object
        let function = self.begin_function(name.clone())?;
        
        // Add parameters to the local scope
        self.begin_scope();
        for param in &parameters {
            self.add_local(param.name.clone())?;
        }
        
        // Compile function body
        for stmt in body {
            self.compile_statement(stmt)?;
        }
        
        // Rest of the implementation...
        // ...
    } else {
        return Err(CompileError::TypeError("Expected ActionDefinition".to_string()));
    }
    
    Ok(())
}
```

Similarly, update `compile_container_definition` to handle the correct AST structure.

## Conclusion

The main issues between the parser and bytecode compiler involve type mismatches, missing functions, and OpCode definitions. By implementing the suggestions in this document, the compatibility issues can be resolved, allowing the WFL system to properly compile programs to bytecode.

After these fixes, the next major step would be implementing the virtual machine to execute the generated bytecode. 