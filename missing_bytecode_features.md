# Missing Bytecode Features

## Overview

The WFL bytecode compiler is partially implemented but lacks support for several language features. This document outlines the missing components and suggests implementation approaches.

## Missing OpCodes

The following opcodes need to be added to the `OpCode` enum:

| OpCode | Purpose | Implementation Priority |
|--------|---------|-------------------------|
| `Duplicate` | Duplicates the top value on the stack | High |
| `GetProperty` | Accesses properties of objects | High |
| `SetProperty` | Sets properties on objects | High |
| `GetIndex` | Accesses elements in collections | High |
| `NewContainer` | Creates new containers | High |
| `DefineField` | Defines fields in containers | High |
| `DefineMethod` | Defines methods in containers | High |
| `Closure` | Creates closures | Medium |
| `Null` | Represents null values | High |
| `AddList` | Adds elements to lists | High |
| `ListAppend` | Appends to lists | High |
| `DefineGlobal` | Defines global variables | Medium |
| `GetGlobal` | Gets global variables | Medium |
| `NewMap` | Creates a new map | High |
| `NewList` | Creates a new list | High |

## Missing Types

The following types need to be implemented:

### Value Type

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function(Rc<Function>),
    Container(Rc<Container>),
    List(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<String, Value>>>),
    // Add other value types as needed
}
```

### Local Variable Type

```rust
#[derive(Debug, Clone)]
pub struct Local {
    pub name: String,
    pub depth: usize,
    pub is_captured: bool,
}
```

### Bytecode Compiler Type

The bytecode compiler needs to be a separate type from the main `Compiler` to support nested function compilation:

```rust
#[derive(Debug)]
pub struct BytecodeCompiler {
    pub function: Function,
    pub locals: Vec<Local>,
    pub scope_depth: usize,
    pub current_line: usize,
    pub enclosing: Option<Box<BytecodeCompiler>>,
    pub functions: Vec<Function>,
}
```

## Missing Methods

The following methods need to be implemented in the `Compiler` struct:

### Constant Emission

```rust
impl Compiler {
    fn emit_constant(&mut self, value: Value) -> Result<(), CompileError> {
        let constant_idx = self.add_constant(Constant::from(value));
        self.emit_byte(OpCode::Constant, constant_idx);
        Ok(())
    }
    
    fn emit_with_operand(&mut self, op: OpCode, operand: u8) {
        self.emit(op);
        self.emit_byte(operand);
    }
    
    fn emit_byte(&mut self, value: u8) {
        self.function.chunk.code.push(value);
        self.function.chunk.lines.push(self.current_line);
    }
}
```

### Scope Management

```rust
impl Compiler {
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    
    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        
        // Pop locals that are out of scope
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            self.emit(OpCode::Pop);
            self.locals.pop();
        }
    }
}
```

### Variable Management

```rust
impl Compiler {
    fn add_local(&mut self, name: String) -> Result<usize, CompileError> {
        let local = Local {
            name: name.clone(),
            depth: self.scope_depth,
            is_captured: false,
        };
        self.locals.push(local);
        Ok(self.locals.len() - 1)
    }
    
    fn declare_local(&mut self, name: String) -> Result<usize, CompileError> {
        // Check for duplicate local in the current scope
        for i in (0..self.locals.len()).rev() {
            let local = &self.locals[i];
            if local.depth < self.scope_depth {
                break;
            }
            if local.name == name {
                return Err(CompileError::InvalidOperation(
                    format!("Variable '{}' already declared in this scope", name)
                ));
            }
        }
        
        self.add_local(name)
    }
    
    fn define_local(&mut self, index: usize) {
        if index < self.locals.len() {
            self.locals[index].depth = self.scope_depth;
        }
    }
    
    fn define_variable(&mut self, name: &str) -> Result<(), CompileError> {
        if self.scope_depth == 0 {
            // Global variable
            let global_idx = self.add_or_get_global(name.to_string());
            self.emit_byte(OpCode::DefineGlobal, global_idx);
        } else {
            // Local variable - already on the stack
            // No need to emit code - the local is defined by its position on the stack
        }
        
        Ok(())
    }
    
    fn add_or_get_global(&mut self, name: String) -> usize {
        // This would maintain a map of global variables for the compiler
        // Since we don't have that structure in the current snippets, this is a placeholder
        0
    }
    
    fn add_identifier(&mut self, name: String) -> usize {
        // This would look up a variable in scope and return its index
        // Since we don't have that structure in the current snippets, this is a placeholder
        0
    }
}
```

## Statement Compilation

The following statement compilation functions need to be implemented or fixed:

1. **Check/If Statement**
```rust
fn compile_if_statement(&mut self, condition: Expression, then_branch: Vec<Statement>, 
                      else_branch: Option<Vec<Statement>>) -> Result<(), CompileError> {
    // Compile condition
    self.compile_expression(condition)?;
    
    // Emit jump instruction for the false case
    let else_jump = self.emit_jump(OpCode::JumpIfFalse);
    
    // Compile then branch
    self.emit(OpCode::Pop); // Pop condition value
    for stmt in then_branch {
        self.compile_statement(stmt)?;
    }
    
    // Emit jump over the else branch
    let end_jump = self.emit_jump(OpCode::Jump);
    
    // Patch the else jump
    self.patch_jump(else_jump);
    self.emit(OpCode::Pop); // Pop condition value
    
    // Compile else branch if present
    if let Some(else_statements) = else_branch {
        for stmt in else_statements {
            self.compile_statement(stmt)?;
        }
    }
    
    // Patch the end jump
    self.patch_jump(end_jump);
    
    Ok(())
}
```

2. **Loop Statements**
```rust
fn compile_while_statement(&mut self, condition: Expression, body: Vec<Statement>) -> Result<(), CompileError> {
    // Store the position of the loop start
    let loop_start = self.current_chunk().code.len();
    
    // Compile condition
    self.compile_expression(condition)?;
    
    // Emit jump instruction for the exit case
    let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
    
    // Compile loop body
    self.emit(OpCode::Pop); // Pop condition value
    for stmt in body {
        self.compile_statement(stmt)?;
    }
    
    // Jump back to the start
    self.emit_loop(loop_start);
    
    // Patch the exit jump
    self.patch_jump(exit_jump);
    self.emit(OpCode::Pop); // Pop condition value
    
    Ok(())
}
```

3. **Action Definitions (Functions)**
```rust
fn compile_action_definition(&mut self, action: ActionDefinition) -> Result<(), CompileError> {
    // Extract details from ActionDefinition
    let name = action.name;
    let parameters = action.parameters;
    let body = action.body;
    
    // Begin function compilation in a new scope
    let function_compiler = BytecodeCompiler::new(name.clone());
    
    // Add parameters
    for param in parameters {
        function_compiler.add_parameter(param)?;
    }
    
    // Compile function body
    for stmt in body {
        function_compiler.compile_statement(stmt)?;
    }
    
    // End function compilation
    let function = function_compiler.end();
    
    // Add function to constants
    let function_idx = self.add_constant(Constant::Function(function));
    
    // Create closure
    self.emit(OpCode::Closure);
    self.emit_byte(function_idx);
    
    // Define the function variable
    self.define_variable(&name)?;
    
    Ok(())
}
```

4. **Container Definitions (Classes)**
```rust
fn compile_container_definition(&mut self, name: String, fields: Vec<VariableField>, 
                             methods: Vec<Statement>) -> Result<(), CompileError> {
    // Create a new container prototype
    self.emit(OpCode::NewContainer);
    
    // Store in global variable
    let global_idx = self.add_or_get_global(name.clone());
    self.emit_byte(OpCode::DefineGlobal, global_idx);
    
    // Get the container
    self.emit_byte(OpCode::GetGlobal, global_idx);
    
    // Add fields
    for field in fields {
        // Duplicate container reference
        self.emit(OpCode::Duplicate);
        
        // Add field name
        let name_idx = self.add_constant(Constant::String(field.name.clone()));
        self.emit_byte(OpCode::Constant, name_idx);
        
        // Add default value if present
        if let Some(expr) = field.initializer {
            self.compile_expression(expr)?;
        } else {
            self.emit(OpCode::Null);
        }
        
        // Define the field
        self.emit(OpCode::DefineField);
    }
    
    // Add methods
    for method_stmt in methods {
        if let Statement::ActionDefinition { name, parameters, body, .. } = method_stmt {
            // Duplicate container reference
            self.emit(OpCode::Duplicate);
            
            // Add method name
            let name_idx = self.add_constant(Constant::String(name.clone()));
            self.emit_byte(OpCode::Constant, name_idx);
            
            // Compile method
            self.compile_action_definition(method_stmt)?;
            
            // Define the method
            self.emit(OpCode::DefineMethod);
        }
    }
    
    // Pop the container reference
    self.emit(OpCode::Pop);
    
    Ok(())
}
```

## Expression Compilation

The following expression compilation functions need to be implemented or fixed:

1. **Collection Operations**
```rust
fn compile_list(&mut self, items: Vec<Expression>) -> Result<(), CompileError> {
    // Create a new list
    self.emit(OpCode::NewList);
    
    // Add each item
    for item in items {
        // Duplicate list reference
        self.emit(OpCode::Duplicate);
        
        // Compile item
        self.compile_expression(item)?;
        
        // Add to list
        self.emit(OpCode::AddList);
    }
    
    Ok(())
}

fn compile_map(&mut self, entries: Vec<(Expression, Expression)>) -> Result<(), CompileError> {
    // Create a new map
    self.emit(OpCode::NewMap);
    
    // Add each entry
    for (key, value) in entries {
        // Duplicate map reference
        self.emit(OpCode::Duplicate);
        
        // Compile key
        self.compile_expression(key)?;
        
        // Compile value
        self.compile_expression(value)?;
        
        // Set in map
        self.emit(OpCode::SetProperty);
    }
    
    Ok(())
}
```

2. **Function Calls With Named Parameters**
```rust
fn compile_call(&mut self, callee: Expression, args: Vec<NamedArgument>) -> Result<(), CompileError> {
    // Compile callee
    self.compile_expression(callee)?;
    
    // Count positional args vs named args
    let (positional, named): (Vec<_>, Vec<_>) = args.into_iter()
        .partition(|arg| arg.name.is_none());
    
    // If we have named args, create a map for them
    if !named.is_empty() {
        self.emit(OpCode::NewMap);
        
        for arg in named {
            // Duplicate map reference
            self.emit(OpCode::Duplicate);
            
            // Add name
            if let Some(name) = arg.name {
                let name_idx = self.add_constant(Constant::String(name));
                self.emit_byte(OpCode::Constant, name_idx);
            }
            
            // Add value
            self.compile_expression(arg.value)?;
            
            // Set in map
            self.emit(OpCode::SetProperty);
        }
        
        // Compile positional args
        for arg in positional {
            self.compile_expression(arg.value)?;
        }
        
        // Call with positional args + 1 for named args map
        self.emit_byte(OpCode::Call, (positional.len() + 1) as u8);
    } else {
        // Compile all args (they're all positional)
        for arg in positional {
            self.compile_expression(arg.value)?;
        }
        
        // Call with arg count
        self.emit_byte(OpCode::Call, positional.len() as u8);
    }
    
    Ok(())
}
```

## Implementation Plan

1. **High Priority Tasks**
   - Implement missing types (`Value`, `Local`, `BytecodeCompiler`)
   - Add missing OpCodes to the enum
   - Implement scope and variable management functions
   - Fix the compilation of basic statements (if/check, loops)

2. **Medium Priority Tasks**
   - Implement container and action definition compilation
   - Add support for collection operations
   - Implement proper error handling with line information

3. **Low Priority Tasks**
   - Optimize bytecode generation
   - Add debug information
   - Implement more specialized opcodes for performance

## Testing Strategy

For each implemented feature:
1. Create unit tests with simple test cases
2. Create more complex integration tests
3. Test error handling
4. Verify bytecode output against expected patterns

## Conclusion

The bytecode compiler has a good foundation but needs significant work to fully support the WFL language features. By implementing the missing components outlined in this document, we can create a robust bytecode compiler that can handle all valid WFL programs. 