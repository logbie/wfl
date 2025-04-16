use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use thiserror::Error;

use crate::parser::*;

mod op;
pub use op::*;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl CompileError {
    pub fn new(message: &str) -> Self {
        CompileError::InvalidOperation(message.to_string())
    }
}

/// A Value represents runtime values in WFL
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
}

/// A constant value in the bytecode
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Number(f64),
    String(String),
    Boolean(bool),
    Nothing,
    Function(Box<Function>),
    Container(String, HashMap<String, usize>),
}

impl From<Value> for Constant {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(n) => Constant::Number(n),
            Value::String(s) => Constant::String(s),
            Value::Boolean(b) => Constant::Boolean(b),
            Value::Null => Constant::Nothing,
            Value::Function(f) => Constant::Function(Box::new((*f).clone())),
            _ => panic!("Cannot convert to constant: {:?}", value),
        }
    }
}

/// Chunk represents a compiled block of bytecode
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Constant>,
    // Line numbers for debugging (maps instruction index to source line)
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// Add an instruction to the chunk
    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }

    /// Add a constant to the chunk and return its index
    pub fn add_constant(&mut self, value: Constant) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

/// Container represents a class-like structure
#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub name: String,
    pub fields: HashMap<String, Value>,
    pub methods: HashMap<String, Rc<Function>>,
}

impl Container {
    pub fn new(name: &str) -> Self {
        Container {
            name: name.to_string(),
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }
}

/// Function represents a compiled function with its bytecode
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub chunk: Chunk,
    pub arity: usize,
    pub is_async: bool,
}

impl Function {
    pub fn new(name: &str, is_async: bool) -> Self {
        Self {
            name: name.to_string(),
            chunk: Chunk::new(),
            arity: 0,
            is_async,
        }
    }
}

/// Local represents a local variable
#[derive(Debug, Clone)]
pub struct Local {
    pub name: String,
    pub depth: usize,
    pub is_captured: bool,
}

/// BytecodeCompiler compiles functions
#[derive(Debug)]
pub struct BytecodeCompiler {
    pub function: Function,
    pub locals: Vec<Local>,
    pub scope_depth: usize,
    pub current_line: usize,
    pub enclosing: Option<Box<BytecodeCompiler>>,
    pub globals: HashMap<String, usize>,
    pub functions: Vec<Function>,
    pub is_container_method: bool, // Flag to track if we're compiling a container method
}

impl BytecodeCompiler {
    pub fn new(name: &str) -> Self {
        Self {
            function: Function::new(name, false),
            locals: Vec::new(),
            scope_depth: 0,
            current_line: 0,
            enclosing: None,
            globals: HashMap::new(),
            functions: Vec::new(),
            is_container_method: false,
        }
    }
    
    pub fn chunk(&self) -> &Chunk {
        &self.function.chunk
    }
    
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.function.chunk
    }
    
    /// Add a parameter to the function
    pub fn add_parameter(&mut self, param: String) -> Result<(), CompileError> {
        self.add_local(param)
    }
    
    /// Begin a new scope
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    
    /// End the current scope
    pub fn end_scope(&mut self) {
        self.scope_depth -= 1;
        
        // Pop locals that are out of scope
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            self.emit(OpCode::Pop);
            self.locals.pop();
        }
    }
    
    /// Add a local variable
    pub fn add_local(&mut self, name: String) -> Result<(), CompileError> {
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
        
        let local = Local {
            name,
            depth: self.scope_depth,
            is_captured: false,
        };
        
        self.locals.push(local);
        Ok(())
    }
    
    /// Declare a local variable (add to locals but don't mark as initialized yet)
    pub fn declare_local(&mut self, name: String) -> Result<usize, CompileError> {
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
        
        let local = Local {
            name,
            depth: usize::MAX, // Mark as uninitialized
            is_captured: false,
        };
        
        self.locals.push(local);
        Ok(self.locals.len() - 1)
    }
    
    /// Define a local variable (mark as initialized)
    pub fn define_local(&mut self, index: usize) {
        if self.locals.len() > index {
            self.locals[index].depth = self.scope_depth;
        }
    }
    
    /// Define a variable (local or global)
    pub fn define_variable(&mut self, name: &str) -> Result<(), CompileError> {
        if self.scope_depth == 0 {
            // Global variable
            let global_idx = self.add_or_get_global(name.to_string());
            self.emit(OpCode::DefineGlobal(global_idx));
        } else {
            // Local variable - already on the stack
        }
        
        Ok(())
    }
    
    /// Add or get a global variable index
    pub fn add_or_get_global(&mut self, name: String) -> usize {
        if let Some(idx) = self.globals.get(&name) {
            return *idx;
        }
        
        let idx = self.globals.len();
        self.globals.insert(name, idx);
        idx
    }
    
    /// Find a variable (local or global)
    pub fn resolve_variable(&mut self, name: &str) -> Result<usize, CompileError> {
        // Look for local variables first (in reverse order to find the most recent)
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                if local.depth == usize::MAX {
                    return Err(CompileError::InvalidOperation(
                        format!("Cannot read local variable '{}' in its own initializer", name)
                    ));
                }
                return Ok(i);
            }
        }
        
        // If not found in locals, try globals
        if let Some(idx) = self.globals.get(name) {
            Ok(*idx)
        } else {
            Err(CompileError::UndefinedVariable(name.to_string()))
        }
    }
    
    /// Add a function to the functions table
    pub fn add_function(&mut self, function: Function) -> usize {
        self.functions.push(function);
        self.functions.len() - 1
    }
    
    /// Emit a bytecode instruction
    pub fn emit(&mut self, op: OpCode) {
        self.function.chunk.write(op, self.current_line);
    }
    
    /// Emit a constant
    pub fn emit_constant(&mut self, value: Value) -> Result<(), CompileError> {
        let constant = Constant::from(value);
        let idx = self.function.chunk.add_constant(constant);
        self.emit(OpCode::Constant(idx));
        Ok(())
    }
    
    /// Add a constant to the chunk
    pub fn add_constant(&mut self, value: Constant) -> usize {
        self.function.chunk.add_constant(value)
    }
    
    /// Check if the last instruction is a return
    pub fn last_instruction_is_return(&self) -> bool {
        if let Some(last) = self.function.chunk.code.last() {
            match last {
                OpCode::Return => true,
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// End function compilation and return the function
    pub fn end(mut self) -> Function {
        // Add a return if the function doesn't end with one
        if !self.last_instruction_is_return() {
            self.emit(OpCode::Null);
            self.emit(OpCode::Return);
        }
        
        self.function
    }
    
    /// Compile a statement
    pub fn compile_statement(&mut self, stmt: Statement) -> Result<(), CompileError> {
        match stmt {
            Statement::ExpressionStatement(expr) => {
                self.compile_expression(expr)?;
                // Discard the value
                self.emit(OpCode::Pop);
                Ok(())
            },
            Statement::VariableDeclaration { name, value_type: _, initializer } => {
                // Compile initializer if present
                if let Some(init_expr) = initializer {
                    self.compile_expression(init_expr)?;
                } else {
                    // Default to null if no initializer
                    self.emit(OpCode::Null);
                }
                
                // Define the variable
                if self.scope_depth > 0 {
                    // Local variable
                    let local_idx = self.declare_local(name.clone())?;
                    self.define_local(local_idx);
                } else {
                    // Global variable
                    let global_idx = self.add_or_get_global(name);
                    self.emit(OpCode::DefineGlobal(global_idx));
                }
                
                Ok(())
            },
            Statement::ReturnStatement(expr) => {
                if let Some(return_expr) = expr {
                    self.compile_expression(return_expr)?;
                } else {
                    self.emit(OpCode::Null);
                }
                
                self.emit(OpCode::Return);
                Ok(())
            },
            Statement::CheckStatement { condition: _condition, then_branch: _then_branch, else_branch: _else_branch } => {
                Err(CompileError::NotImplemented("Check statements not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::ForEachLoop { item_name: _, index_name: _, collection: _, body: _ } => {
                Err(CompileError::NotImplemented("ForEach loops not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::RepeatLoop { is_while: _, condition: _, body: _ } => {
                Err(CompileError::NotImplemented("Repeat loops not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::CountLoop { counter_name: _, start: _, end: _, step: _, body: _ } => {
                Err(CompileError::NotImplemented("Count loops not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::TryCatch { try_block: _, catch_variable: _, catch_block: _, finally_block: _ } => {
                Err(CompileError::NotImplemented("Try-catch not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::ActionDefinition { name: _, parameters: _, return_type: _, body: _, is_async: _, is_private: _ } => {
                Err(CompileError::NotImplemented("Action definitions not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::ContainerDefinition { name: _, fields: _, methods: _, constructor: _ } => {
                Err(CompileError::NotImplemented("Container definitions not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::BreakStatement(_) => {
                Err(CompileError::NotImplemented("Break statements not fully implemented in BytecodeCompiler".to_string()))
            },
            Statement::ContinueStatement(_) => {
                Err(CompileError::NotImplemented("Continue statements not fully implemented in BytecodeCompiler".to_string()))
            }
        }
    }
    
    /// Compile an expression
    pub fn compile_expression(&mut self, expr: Expression) -> Result<(), CompileError> {
        match expr {
            Expression::StringLiteral(value) => {
                self.emit_constant(Value::String(value))?;
                return Ok(());
            },
            Expression::NumberLiteral(value) => {
                self.emit_constant(Value::Number(value))?;
                return Ok(());
            },
            Expression::BooleanLiteral(value) => {
                self.emit_constant(Value::Boolean(value))?;
                return Ok(());
            },
            Expression::NothingLiteral => {
                self.emit(OpCode::Null);
                return Ok(());
            },
            Expression::Variable(name) => {
                // Check if this is a local variable
                if let Some(idx) = self.resolve_local(&name) {
                    self.emit(OpCode::GetLocal(idx));
                    return Ok(());
                }

                // If we're in a container method, check if this is a field access
                if self.is_container_method {
                    // Check if it's the "self" parameter (which should be the first parameter)
                    if self.locals.len() > 0 {
                        // Load "self"
                        self.emit(OpCode::GetLocal(0));
                        
                        // Push field name
                        self.emit_constant(Value::String(name.clone()))?;
                        
                        // Get property
                        self.emit(OpCode::GetProperty);
                        
                        return Ok(());
                    }
                }

                // Check if it's a global variable
                if let Some(idx) = self.globals.get(&name) {
                    self.emit(OpCode::GetGlobal(*idx));
                    return Ok(());
                }

                // Not found
                return Err(CompileError::UndefinedVariable(name));
            },
            Expression::Binary { left, operator, right } => {
                match operator {
                    BinaryOperator::Assign => {
                        // We handle assignment differently - right side first, then left side target
                        // Compile the right side of the assignment
                        self.compile_expression(*right)?;
                        
                        match *left {
                            Expression::Variable(name) => {
                                // Check if this is a local variable
                                if let Some(idx) = self.resolve_local(&name) {
                                    self.emit(OpCode::SetLocal(idx));
                                    return Ok(());
                                }
                                
                                // Assign to global variable
                                let global_idx = self.add_or_get_global(name);
                                self.emit(OpCode::DefineGlobal(global_idx));
                                return Ok(());
                            },
                            Expression::MemberAccess { object, name } => {
                                // Compile the object
                                self.compile_expression(*object)?;
                                
                                // Push property name
                                self.emit_constant(Value::String(name))?;
                                
                                // Set property
                                self.emit(OpCode::SetProperty);
                                return Ok(());
                            },
                            Expression::Index { collection, index } => {
                                // Compile collection
                                self.compile_expression(*collection)?;
                                
                                // Compile index
                                self.compile_expression(*index)?;
                                
                                // Set index
                                self.emit(OpCode::SetIndex);
                                return Ok(());
                            },
                            _ => return Err(CompileError::InvalidOperation("Invalid assignment target".to_string())),
                        }
                    },
                    // For normal binary operations, compile both sides first
                    _ => {
                        self.compile_expression(*left)?;
                        self.compile_expression(*right)?;
                        
                        match operator {
                            BinaryOperator::Add => self.emit(OpCode::Add),
                            BinaryOperator::Subtract => self.emit(OpCode::Subtract),
                            BinaryOperator::Multiply => self.emit(OpCode::Multiply),
                            BinaryOperator::Divide => self.emit(OpCode::Divide),
                            BinaryOperator::Modulo => self.emit(OpCode::Modulo),
                            BinaryOperator::Equal => self.emit(OpCode::Equal),
                            BinaryOperator::NotEqual => {
                                self.emit(OpCode::Equal);
                                self.emit(OpCode::Not);
                            },
                            BinaryOperator::Greater => self.emit(OpCode::Greater),
                            BinaryOperator::Less => self.emit(OpCode::Less),
                            BinaryOperator::GreaterEqual => self.emit(OpCode::GreaterEqual),
                            BinaryOperator::LessEqual => self.emit(OpCode::LessEqual),
                            BinaryOperator::And => self.emit(OpCode::And),
                            BinaryOperator::Or => self.emit(OpCode::Or),
                            BinaryOperator::Join => self.emit(OpCode::Join),
                            BinaryOperator::Assign => unreachable!(), // Handled above
                        }
                        
                        return Ok(());
                    }
                }
            },
            Expression::Unary { operator: _operator, right: _right } => {
                Err(CompileError::NotImplemented("Unary expressions not fully implemented in BytecodeCompiler".to_string()))
            },
            Expression::Call { callee, arguments } => {
                // Compile the function
                self.compile_expression(*callee)?;
                
                // Split positional and named arguments
                let mut positional_args = Vec::new();
                let mut named_args = Vec::new();
                
                for arg in arguments {
                    if arg.name.is_none() {
                        positional_args.push(arg.value);
                    } else {
                        named_args.push(arg);
                    }
                }
                
                // If we have named arguments, create a map for them
                if !named_args.is_empty() {
                    self.emit(OpCode::NewMap(named_args.len()));
                    
                    for arg in named_args {
                        if let Some(name) = arg.name {
                            // Push the name
                            self.emit_constant(Value::String(name))?;
                            
                            // Push the value
                            self.compile_expression(arg.value)?;
                            
                            // Set in map
                            self.emit(OpCode::SetProperty);
                        }
                    }
                    
                    // Compile positional arguments
                    for arg in &positional_args {
                        self.compile_expression(arg.clone())?;
                    }
                    
                    // Call with positional args + 1 for named args map
                    self.emit(OpCode::Call(positional_args.len() + 1));
                } else {
                    // Compile all arguments
                    for arg in &positional_args {
                        self.compile_expression(arg.clone())?;
                    }
                    
                    // Call function
                    self.emit(OpCode::Call(positional_args.len()));
                }
                
                return Ok(());
            },
            Expression::MemberAccess { object, name } => {
                self.compile_expression(*object)?;
                self.emit_constant(Value::String(name))?;
                self.emit(OpCode::GetProperty);
                return Ok(());
            },
            Expression::Index { collection: _collection, index: _index } => {
                Err(CompileError::NotImplemented("Index access not fully implemented in BytecodeCompiler".to_string()))
            },
            Expression::ListExpression(_) => {
                Err(CompileError::NotImplemented("List expressions not fully implemented in BytecodeCompiler".to_string()))
            },
            Expression::MapExpression(_) => {
                Err(CompileError::NotImplemented("Map expressions not fully implemented in BytecodeCompiler".to_string()))
            },
            Expression::SetExpression(_) => {
                Err(CompileError::NotImplemented("Set expressions not fully implemented in BytecodeCompiler".to_string()))
            },
            Expression::RecordExpression(_) => {
                Err(CompileError::NotImplemented("Record expressions not fully implemented in BytecodeCompiler".to_string()))
            },
            Expression::Await(_) => {
                Err(CompileError::NotImplemented("Await expressions not fully implemented in BytecodeCompiler".to_string()))
            },
            Expression::Try(_) => {
                Err(CompileError::NotImplemented("Try expressions not fully implemented in BytecodeCompiler".to_string()))
            }
        }
    }

    /// Find a local variable by name
    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate() {
            if local.name == name {
                if local.depth == usize::MAX {
                    // Variable is being used in its own initializer
                    return None;
                }
                return Some(i);
            }
        }
        None
    }

    // Compile container methods in the BytecodeCompiler implementation
    fn compile_container_methods(&mut self, methods: &[Statement]) -> Result<(), CompileError> {
        for method in methods {
            if let Statement::ActionDefinition { name, parameters, return_type: _, body, is_async, is_private: _ } = method {
                // Duplicate container reference
                self.emit(OpCode::Duplicate);
                
                // Push method name
                self.emit_constant(Value::String(name.clone()))?;
                
                // Create a new compiler for this method
                let mut method_compiler = BytecodeCompiler::new(&name);
                method_compiler.current_line = self.current_line;
                method_compiler.is_container_method = true;
                
                // Begin scope for parameters
                method_compiler.begin_scope();
                
                // Add 'self' as the first parameter
                method_compiler.add_parameter("self".to_string())?;
                
                // Add method parameters
                for param in parameters {
                    method_compiler.add_parameter(param.name.clone())?;
                }
                
                // Compile method body
                for stmt in body {
                    method_compiler.compile_statement(stmt.clone())?;
                }
                
                // Add implicit return if needed
                if !method_compiler.last_instruction_is_return() {
                    method_compiler.emit(OpCode::Null);
                    method_compiler.emit(OpCode::Return);
                }
                
                // End scope
                method_compiler.end_scope();
                
                // Get the compiled method
                let function = method_compiler.end();
                
                // Add to constants
                let function_idx = self.add_constant(Constant::Function(Box::new(function)));
                
                // Create closure
                self.emit(OpCode::Closure(function_idx));
                
                // Define the method on the container
                self.emit(OpCode::DefineMethod);
            }
        }
        
        Ok(())
    }
}

/// The bytecode compiler
pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
    #[allow(dead_code)]
    label_counter: usize,
    #[allow(dead_code)]
    enclosing: Option<Box<Compiler>>,
    current_line: usize,
    globals: HashMap<String, usize>,
    #[allow(dead_code)]
    functions: Vec<Function>,
    is_container_method: bool, // Flag to track if we're compiling a container method
}

impl Compiler {
    /// Create a new compiler
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
            label_counter: 0,
            enclosing: None,
            current_line: 0,
            globals: HashMap::new(),
            functions: Vec::new(),
            is_container_method: false,
        }
    }

    /// Compile an AST into bytecode
    pub fn compile(&mut self, program: Program) -> Result<Function, Vec<CompileError>> {
        let mut errors = Vec::new();

        // Compile each statement in the program
        for statement in program.statements {
            match self.compile_statement(statement) {
                Ok(_) => (),
                Err(err) => errors.push(err),
            }
        }

        // Add a return instruction at the end if there isn't one
        if !self.last_instruction_is_return() {
            self.emit(OpCode::Null);
            self.emit(OpCode::Return);
        }

        if errors.is_empty() {
            // Create a main function with the compiled bytecode
            let main_function = Function {
                name: "main".to_string(),
                chunk: self.chunk.clone(),
                arity: 0,
                is_async: false,
            };
            
            Ok(main_function)
        } else {
            Err(errors)
        }
    }

    /// Compile a statement to bytecode
    fn compile_statement(&mut self, statement: Statement) -> Result<(), CompileError> {
        match statement {
            Statement::ExpressionStatement(expr) => {
                self.compile_expression(expr)?;
                // Discard the value
                self.emit(OpCode::Pop);
                Ok(())
            },
            Statement::VariableDeclaration { name, value_type: _, initializer } => {
                // Compile initializer if present
                if let Some(init_expr) = initializer {
                    self.compile_expression(init_expr)?;
                } else {
                    // Default to null if no initializer
                    self.emit(OpCode::Null);
                }
                
                // Define the variable
                if self.scope_depth > 0 {
                    // Local variable
                    let local_idx = self.declare_local(name.clone())?;
                    self.define_local(local_idx);
                } else {
                    // Global variable
                    let global_idx = self.add_or_get_global(name);
                    self.emit(OpCode::DefineGlobal(global_idx));
                }
                
                Ok(())
            },
            Statement::CheckStatement { condition, then_branch, else_branch } => {
                // Similar to if-statement
                self.compile_expression(condition)?;
                
                // Jump to else branch if condition is false
                let else_jump = self.emit_jump(OpCode::JumpIfFalse(0));
                
                // Pop condition value
                self.emit(OpCode::Pop);
                
                // Compile then branch
                self.begin_scope();
                for stmt in then_branch {
                    self.compile_statement(stmt)?;
                }
                self.end_scope();
                
                // Jump over else branch
                let end_jump = self.emit_jump(OpCode::Jump(0));
                
                // Patch the else jump
                self.patch_jump(else_jump);
                
                // Pop condition value for else branch
                self.emit(OpCode::Pop);
                
                // Compile else branch if present
                if let Some(else_stmts) = else_branch {
                    self.begin_scope();
                    for stmt in else_stmts {
                        self.compile_statement(stmt)?;
                    }
                    self.end_scope();
                }
                
                // Patch the end jump
                self.patch_jump(end_jump);
                
                Ok(())
            },
            Statement::ForEachLoop { item_name, index_name, collection, body } => {
                // Compile the collection expression
                self.compile_expression(collection)?;
                
                // Create a new iterator (needs VM support)
                self.emit(OpCode::Call(0)); // Call iterator constructor
                
                // Store iterator in a temporary variable
                let iterator_local = self.declare_local("__iterator".to_string())?;
                self.define_local(iterator_local);
                
                // Begin loop
                let loop_start = self.current_chunk().code.len();
                
                // Call iterator.next()
                self.emit(OpCode::GetLocal(iterator_local));
                self.emit(OpCode::Call(0)); // next() with 0 args
                
                // Check if iterator is done
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
                
                // Pop done flag
                self.emit(OpCode::Pop);
                
                // Start new scope for loop variables
                self.begin_scope();
                
                // Define item variable
                let item_local = self.declare_local(item_name)?;
                self.define_local(item_local);
                
                // Define index variable if needed
                if let Some(idx_name) = index_name {
                    let idx_local = self.declare_local(idx_name)?;
                    self.define_local(idx_local);
                }
                
                // Compile loop body
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // End loop variables scope
                self.end_scope();
                
                // Jump back to loop start
                self.emit_loop(loop_start);
                
                // Patch exit jump
                self.patch_jump(exit_jump);
                
                // Pop iterator.done value
                self.emit(OpCode::Pop);
                
                // Pop iterator
                self.emit(OpCode::Pop);
                
                Ok(())
            },
            Statement::RepeatLoop { is_while, condition, body } => {
                let loop_start = self.current_chunk().code.len();
                
                if is_while {
                    // While loop: check condition first
                    self.compile_expression(condition)?;
                    
                    // Jump out of loop if condition is false
                    let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
                    
                    // Pop condition value
                    self.emit(OpCode::Pop);
                    
                    // Compile loop body
                    self.begin_scope();
                    for stmt in body {
                        self.compile_statement(stmt)?;
                    }
                    self.end_scope();
                    
                    // Jump back to start
                    self.emit_loop(loop_start);
                    
                    // Patch exit jump
                    self.patch_jump(exit_jump);
                    
                    // Pop condition value when exiting loop
                    self.emit(OpCode::Pop);
                } else {
                    // Until loop: execute body first, then check condition
                    
                    // Compile loop body
                    let body_start = self.current_chunk().code.len();
                    self.begin_scope();
                    for stmt in body {
                        self.compile_statement(stmt)?;
                    }
                    self.end_scope();
                    
                    // Compile condition
                    self.compile_expression(condition)?;
                    
                    // If condition is true, exit loop
                    let exit_jump = self.emit_jump(OpCode::JumpIfTrue(0));
                    
                    // Pop condition value
                    self.emit(OpCode::Pop);
                    
                    // Jump back to body start
                    self.emit_loop(body_start);
                    
                    // Patch exit jump
                    self.patch_jump(exit_jump);
                    
                    // Pop condition value when exiting loop
                    self.emit(OpCode::Pop);
                }
                
                Ok(())
            },
            Statement::CountLoop { counter_name, start, end, step, body } => {
                // Compile start expression
                self.compile_expression(start)?;
                
                // Define counter variable
                let counter_local = self.declare_local(counter_name.clone())?;
                self.define_local(counter_local);
                
                // Compile end expression
                self.compile_expression(end)?;
                
                // Store end value in a temporary variable
                let end_local = self.declare_local("__end".to_string())?;
                self.define_local(end_local);
                
                // Compile step expression if present
                let step_local = if let Some(step_expr) = step {
                    self.compile_expression(step_expr)?;
                    let step_local = self.declare_local("__step".to_string())?;
                    self.define_local(step_local);
                    Some(step_local)
                } else {
                    // Use default step of 1
                    self.emit_constant(Value::Number(1.0))?;
                    let step_local = self.declare_local("__step".to_string())?;
                    self.define_local(step_local);
                    Some(step_local)
                };
                
                // Begin loop
                let loop_start = self.current_chunk().code.len();
                
                // Check counter < end
                self.emit(OpCode::GetLocal(counter_local));
                self.emit(OpCode::GetLocal(end_local));
                self.emit(OpCode::Less);
                
                // Jump out if counter >= end
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
                
                // Pop comparison result
                self.emit(OpCode::Pop);
                
                // Compile loop body
                self.begin_scope();
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                self.end_scope();
                
                // Increment counter by step
                self.emit(OpCode::GetLocal(counter_local));
                self.emit(OpCode::GetLocal(step_local.unwrap()));
                self.emit(OpCode::Add);
                self.emit(OpCode::SetLocal(counter_local));
                
                // Jump back to start
                self.emit_loop(loop_start);
                
                // Patch exit jump
                self.patch_jump(exit_jump);
                
                // Pop comparison result
                self.emit(OpCode::Pop);
                
                Ok(())
            },
            Statement::TryCatch { try_block, catch_variable, catch_block, finally_block } => {
                // Not fully implemented yet - this is a sketch
                self.begin_scope();
                
                // Try block
                for stmt in try_block {
                    self.compile_statement(stmt)?;
                }
                
                self.end_scope();
                
                // Catch block
                if let Some(var_name) = catch_variable {
                    self.begin_scope();
                    
                    // Define catch variable
                    let error_local = self.declare_local(var_name)?;
                    self.define_local(error_local);
                    
                    for stmt in catch_block {
                        self.compile_statement(stmt)?;
                    }
                    
                    self.end_scope();
                } else {
                    self.begin_scope();
                    
                    for stmt in catch_block {
                        self.compile_statement(stmt)?;
                    }
                    
                    self.end_scope();
                }
                
                // Finally block
                if let Some(finally) = finally_block {
                    self.begin_scope();
                    
                    for stmt in finally {
                        self.compile_statement(stmt)?;
                    }
                    
                    self.end_scope();
                }
                
                Ok(())
            },
            Statement::ActionDefinition { name, parameters, return_type, body, is_async, is_private } => {
                // Create a new compiler for the function
                let mut function_compiler = BytecodeCompiler::new(&name);
                function_compiler.current_line = self.current_line;
                
                // Set arity
                function_compiler.function.arity = parameters.len();
                function_compiler.function.is_async = is_async;
                
                // Add function parameters
                function_compiler.begin_scope();
                for param in parameters {
                    function_compiler.add_parameter(param.name)?;
                }
                
                // Compile function body
                for stmt in body {
                    function_compiler.compile_statement(stmt)?;
                }
                
                // Get the compiled function
                let function = function_compiler.end();
                
                // Add function to constants
                let function_idx = self.add_constant(Constant::Function(Box::new(function)));
                
                // Create closure
                self.emit(OpCode::Closure(function_idx));
                
                // Define function name
                self.define_variable(&name)?;
                
                Ok(())
            },
            Statement::ContainerDefinition { name, fields, methods, constructor } => {
                // Create a new container
                self.emit(OpCode::NewContainer);
                
                // Define container variable
                let global_idx = self.add_or_get_global(name.clone());
                self.emit(OpCode::DefineGlobal(global_idx));
                
                // Get the container back
                self.emit(OpCode::GetGlobal(global_idx));
                
                // Add fields
                for field in fields {
                    // Duplicate container reference
                    self.emit(OpCode::Duplicate);
                    
                    // Push field name
                    self.emit_constant(Value::String(field.name.clone()))?;
                    
                    // Add initializer if present
                    if let Some(expr) = field.initializer {
                        self.compile_expression(expr)?;
                    } else {
                        self.emit(OpCode::Null);
                    }
                    
                    // Define the field
                    self.emit(OpCode::DefineField);
                }
                
                // Compile container methods
                self.compile_container_methods(&methods)?;
                
                // Compile constructor if present
                if let Some(ctor_body) = constructor {
                    // Duplicate container reference
                    self.emit(OpCode::Duplicate);
                    
                    // Push constructor name
                    self.emit_constant(Value::String("constructor".to_string()))?;
                    
                    // Create a compiler for the constructor
                    let mut ctor_compiler = BytecodeCompiler::new("constructor");
                    ctor_compiler.current_line = self.current_line;
                    
                    // Add 'self' as parameter
                    ctor_compiler.begin_scope();
                    ctor_compiler.add_parameter("self".to_string())?;
                    
                    // Compile constructor body
                    for stmt in ctor_body {
                        ctor_compiler.compile_statement(stmt)?;
                    }
                    
                    // Get the compiled constructor
                    let ctor_function = ctor_compiler.end();
                    
                    // Add constructor to constants
                    let function_idx = self.add_constant(Constant::Function(Box::new(ctor_function)));
                    
                    // Create closure
                    self.emit(OpCode::Closure(function_idx));
                    
                    // Define the constructor
                    self.emit(OpCode::DefineMethod);
                }
                
                // Pop container reference
                self.emit(OpCode::Pop);
                
                Ok(())
            },
            Statement::ReturnStatement(expr) => {
                if let Some(return_expr) = expr {
                    self.compile_expression(return_expr)?;
                } else {
                    self.emit(OpCode::Null);
                }
                
                self.emit(OpCode::Return);
                Ok(())
            },
            Statement::BreakStatement(_label) => {
                // Break statement implementation
                // Note: Labels not fully implemented
                Err(CompileError::NotImplemented("Break statements not fully implemented".to_string()))
            },
            Statement::ContinueStatement(_label) => {
                // Continue statement implementation
                // Note: Labels not fully implemented
                Err(CompileError::NotImplemented("Continue statements not fully implemented".to_string()))
            },
        }
    }

    /// Compile an expression to bytecode
    fn compile_expression(&mut self, expr: Expression) -> Result<(), CompileError> {
        match expr {
            Expression::StringLiteral(value) => {
                self.emit_constant(Value::String(value))?;
                return Ok(());
            },
            Expression::NumberLiteral(value) => {
                self.emit_constant(Value::Number(value))?;
                return Ok(());
            },
            Expression::BooleanLiteral(value) => {
                self.emit_constant(Value::Boolean(value))?;
                return Ok(());
            },
            Expression::NothingLiteral => {
                self.emit(OpCode::Null);
                return Ok(());
            },
            Expression::Variable(name) => {
                // Check if this is a local variable
                if let Some(idx) = self.resolve_local(&name) {
                    self.emit(OpCode::GetLocal(idx));
                    return Ok(());
                }

                // If we're in a container method, check if this is a field access
                if self.is_container_method {
                    // Check if it's the "self" parameter (which should be the first parameter)
                    if self.locals.len() > 0 {
                        // Load "self"
                        self.emit(OpCode::GetLocal(0));
                        
                        // Push field name
                        self.emit_constant(Value::String(name.clone()))?;
                        
                        // Get property
                        self.emit(OpCode::GetProperty);
                        
                        return Ok(());
                    }
                }

                // Check if it's a global variable
                if let Some(idx) = self.globals.get(&name) {
                    self.emit(OpCode::GetGlobal(*idx));
                    return Ok(());
                }

                // Not found
                return Err(CompileError::UndefinedVariable(name));
            },
            Expression::Binary { left, operator, right } => {
                match operator {
                    BinaryOperator::Assign => {
                        // We handle assignment differently - right side first, then left side target
                        // Compile the right side of the assignment
                        self.compile_expression(*right)?;
                        
                        match *left {
                            Expression::Variable(name) => {
                                // Check if this is a local variable
                                if let Some(idx) = self.resolve_local(&name) {
                                    self.emit(OpCode::SetLocal(idx));
                                    return Ok(());
                                }
                                
                                // Assign to global variable
                                let global_idx = self.add_or_get_global(name);
                                self.emit(OpCode::DefineGlobal(global_idx));
                                return Ok(());
                            },
                            Expression::MemberAccess { object, name } => {
                                // Compile the object
                                self.compile_expression(*object)?;
                                
                                // Push property name
                                self.emit_constant(Value::String(name))?;
                                
                                // Set property
                                self.emit(OpCode::SetProperty);
                                return Ok(());
                            },
                            Expression::Index { collection, index } => {
                                // Compile collection
                                self.compile_expression(*collection)?;
                                
                                // Compile index
                                self.compile_expression(*index)?;
                                
                                // Set index
                                self.emit(OpCode::SetIndex);
                                return Ok(());
                            },
                            _ => return Err(CompileError::InvalidOperation("Invalid assignment target".to_string())),
                        }
                    },
                    // For normal binary operations, compile both sides first
                    _ => {
                        self.compile_expression(*left)?;
                        self.compile_expression(*right)?;
                        
                        match operator {
                            BinaryOperator::Add => self.emit(OpCode::Add),
                            BinaryOperator::Subtract => self.emit(OpCode::Subtract),
                            BinaryOperator::Multiply => self.emit(OpCode::Multiply),
                            BinaryOperator::Divide => self.emit(OpCode::Divide),
                            BinaryOperator::Modulo => self.emit(OpCode::Modulo),
                            BinaryOperator::Equal => self.emit(OpCode::Equal),
                            BinaryOperator::NotEqual => {
                                self.emit(OpCode::Equal);
                                self.emit(OpCode::Not);
                            },
                            BinaryOperator::Greater => self.emit(OpCode::Greater),
                            BinaryOperator::Less => self.emit(OpCode::Less),
                            BinaryOperator::GreaterEqual => self.emit(OpCode::GreaterEqual),
                            BinaryOperator::LessEqual => self.emit(OpCode::LessEqual),
                            BinaryOperator::And => self.emit(OpCode::And),
                            BinaryOperator::Or => self.emit(OpCode::Or),
                            BinaryOperator::Join => self.emit(OpCode::Join),
                            BinaryOperator::Assign => unreachable!(), // Handled above
                        }
                        
                        return Ok(());
                    }
                }
            },
            Expression::Unary { operator, right } => {
                self.compile_expression(*right)?;
                
                match operator {
                    UnaryOperator::Negate => self.emit(OpCode::Negate),
                    UnaryOperator::Not => self.emit(OpCode::Not),
                }
            },
            Expression::Call { callee, arguments } => {
                // Compile the function
                self.compile_expression(*callee)?;
                
                // Split positional and named arguments
                let mut positional_args = Vec::new();
                let mut named_args = Vec::new();
                
                for arg in arguments {
                    if arg.name.is_none() {
                        positional_args.push(arg.value);
                    } else {
                        named_args.push(arg);
                    }
                }
                
                // If we have named arguments, create a map for them
                if !named_args.is_empty() {
                    self.emit(OpCode::NewMap(named_args.len()));
                    
                    for arg in named_args {
                        if let Some(name) = arg.name {
                            // Push the name
                            self.emit_constant(Value::String(name))?;
                            
                            // Push the value
                            self.compile_expression(arg.value)?;
                            
                            // Set in map
                            self.emit(OpCode::SetProperty);
                        }
                    }
                    
                    // Compile positional arguments
                    for arg in &positional_args {
                        self.compile_expression(arg.clone())?;
                    }
                    
                    // Call with positional args + 1 for named args map
                    self.emit(OpCode::Call(positional_args.len() + 1));
                } else {
                    // Compile all arguments
                    for arg in &positional_args {
                        self.compile_expression(arg.clone())?;
                    }
                    
                    // Call function
                    self.emit(OpCode::Call(positional_args.len()));
                }
                
                return Ok(());
            },
            Expression::MemberAccess { object, name } => {
                self.compile_expression(*object)?;
                self.emit_constant(Value::String(name))?;
                self.emit(OpCode::GetProperty);
                return Ok(());
            },
            Expression::Index { collection, index } => {
                self.compile_expression(*collection)?;
                self.compile_expression(*index)?;
                self.emit(OpCode::GetIndex);
                return Ok(());
            },
            Expression::ListExpression(items) => {
                // Create a new list
                self.emit(OpCode::NewList(items.len()));
                
                // Add each item
                for (i, item) in items.iter().enumerate() {
                    // Duplicate list reference
                    self.emit(OpCode::Duplicate);
                    
                    // Push index
                    self.emit_constant(Value::Number(i as f64))?;
                    
                    // Compile item
                    self.compile_expression(item.clone())?;
                    
                    // Set item
                    self.emit(OpCode::SetIndex);
                }
                return Ok(());
            },
            Expression::MapExpression(entries) => {
                // Create a new map
                self.emit(OpCode::NewMap(entries.len()));
                
                // Add each entry
                for (key, value) in entries {
                    // Duplicate map reference
                    self.emit(OpCode::Duplicate);
                    
                    // Push key as string
                    self.emit_constant(Value::String(key))?;
                    
                    // Compile value
                    self.compile_expression(value)?;
                    
                    // Set property
                    self.emit(OpCode::SetProperty);
                }
                return Ok(());
            },
            Expression::Await(expr) => {
                // Compile the expression
                self.compile_expression(*expr)?;
                
                // Mark as awaiting
                self.emit(OpCode::Call(0)); // Call await function with 0 args
                
                // Note: Actual await handling requires VM support
                return Err(CompileError::NotImplemented("Await expressions not fully implemented".to_string()));
            },
            Expression::Try(expr) => {
                // Compile the expression
                self.compile_expression(*expr)?;
                
                // Mark as try
                self.emit(OpCode::Call(0)); // Call try function with 0 args
                
                // Note: Actual try handling requires VM support
                return Err(CompileError::NotImplemented("Try expressions not fully implemented".to_string()));
            },
            Expression::SetExpression(items) => {
                // Create a set (implemented as a Map in the VM)
                self.emit(OpCode::NewMap(items.len()));
                
                // Add each item as a key with a true value
                for item in items {
                    // Duplicate map reference
                    self.emit(OpCode::Duplicate);
                    
                    // Compile item as key
                    self.compile_expression(item)?;
                    
                    // Push true as value
                    self.emit_constant(Value::Boolean(true))?;
                    
                    // Set in map
                    self.emit(OpCode::SetProperty);
                }
                return Ok(());
            },
            Expression::RecordExpression(fields) => {
                // Record is just a map with string keys
                self.emit(OpCode::NewMap(fields.len()));
                
                // Add each field
                for (key, value) in fields {
                    // Duplicate map reference
                    self.emit(OpCode::Duplicate);
                    
                    // Push key
                    self.emit_constant(Value::String(key))?;
                    
                    // Compile value
                    self.compile_expression(value)?;
                    
                    // Set in map
                    self.emit(OpCode::SetProperty);
                }
                return Ok(());
            },
        }
        
        Ok(())
    }
    
    /// Emit a jump instruction and return its position
    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit(instruction);
        self.current_chunk().code.len() - 1
    }
    
    /// Patch a jump instruction with the current position
    fn patch_jump(&mut self, offset: usize) {
        // Get the amount to jump
        let jump = self.current_chunk().code.len() - offset - 1;
        
        // Update the jump instruction
        match &mut self.chunk.code[offset] {
            OpCode::JumpIfFalse(ref mut dest) => *dest = jump,
            OpCode::JumpIfTrue(ref mut dest) => *dest = jump,
            OpCode::Jump(ref mut dest) => *dest = jump,
            _ => panic!("Not a jump instruction"),
        }
    }
    
    /// Emit a loop instruction (jump back to a previous position)
    fn emit_loop(&mut self, start: usize) {
        let offset = self.current_chunk().code.len() - start + 1;
        self.emit(OpCode::Jump(offset));
    }
    
    /// Get the current chunk
    fn current_chunk(&self) -> &Chunk {
        &self.chunk
    }
    
    /// Get a mutable reference to the current chunk
    #[allow(dead_code)]
    fn current_chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
    
    /// Begin a scope
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    
    /// End a scope
    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        
        // Pop locals that are now out of scope
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            self.emit(OpCode::Pop);
            self.locals.pop();
        }
    }
    
    /// Declare a local variable
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
        
        let local = Local {
            name,
            depth: usize::MAX, // Mark as uninitialized
            is_captured: false,
        };
        
        self.locals.push(local);
        Ok(self.locals.len() - 1)
    }
    
    /// Define a local variable (mark as initialized)
    fn define_local(&mut self, index: usize) {
        if self.locals.len() > index {
            self.locals[index].depth = self.scope_depth;
        }
    }
    
    /// Add or get a global variable
    fn add_or_get_global(&mut self, name: String) -> usize {
        if let Some(idx) = self.globals.get(&name) {
            return *idx;
        }
        
        let idx = self.globals.len();
        self.globals.insert(name, idx);
        idx
    }
    
    /// Define a variable
    fn define_variable(&mut self, name: &str) -> Result<(), CompileError> {
        if self.scope_depth == 0 {
            // Global variable
            let global_idx = self.add_or_get_global(name.to_string());
            self.emit(OpCode::DefineGlobal(global_idx));
        }
        // For local variables, we've already defined them
        
        Ok(())
    }
    
    /// Emit a bytecode instruction
    fn emit(&mut self, op: OpCode) {
        self.chunk.write(op, self.current_line);
    }
    
    /// Add a constant to the chunk
    fn add_constant(&mut self, value: Constant) -> usize {
        self.chunk.add_constant(value)
    }
    
    /// Emit a constant instruction
    fn emit_constant(&mut self, value: Value) -> Result<(), CompileError> {
        let constant = Constant::from(value);
        let idx = self.chunk.add_constant(constant);
        self.emit(OpCode::Constant(idx));
        Ok(())
    }
    
    /// Check if the last instruction is a return
    fn last_instruction_is_return(&self) -> bool {
        if let Some(last) = self.chunk.code.last() {
            match last {
                OpCode::Return => true,
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// Resolve a variable name to its index
    fn resolve_variable(&mut self, name: &str) -> Result<usize, CompileError> {
        // Look for local variables first (in reverse order to find the most recent)
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                if local.depth == usize::MAX {
                    return Err(CompileError::InvalidOperation(
                        format!("Cannot read local variable '{}' in its own initializer", name)
                    ));
                }
                return Ok(i);
            }
        }
        
        // If not found in locals, try globals
        if let Some(idx) = self.globals.get(name) {
            Ok(*idx)
        } else {
            Err(CompileError::UndefinedVariable(name.to_string()))
        }
    }

    /// Find a local variable by name
    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate() {
            if local.name == name {
                if local.depth == usize::MAX {
                    // Variable is being used in its own initializer
                    return None;
                }
                return Some(i);
            }
        }
        None
    }

    // Compile container methods
    fn compile_container_methods(&mut self, methods: &[Statement]) -> Result<(), CompileError> {
        for method in methods {
            if let Statement::ActionDefinition { name, parameters, return_type: _, body, is_async, is_private } = method {
                // Duplicate container reference
                self.emit(OpCode::Duplicate);
                
                // Push method name
                self.emit_constant(Value::String(name.clone()))?;
                
                // Create a new compiler for this method
                let mut method_compiler = BytecodeCompiler::new(&name);
                method_compiler.current_line = self.current_line;
                method_compiler.is_container_method = true;
                
                // Begin scope for parameters
                method_compiler.begin_scope();
                
                // Add 'self' as the first parameter
                method_compiler.add_parameter("self".to_string())?;
                
                // Add method parameters
                for param in parameters {
                    method_compiler.add_parameter(param.name.clone())?;
                }
                
                // Compile method body
                for stmt in body {
                    method_compiler.compile_statement(stmt.clone())?;
                }
                
                // End scope
                method_compiler.end_scope();
                
                // Get the compiled method
                let function = method_compiler.end();
                
                // Add to constants
                let function_idx = self.add_constant(Constant::Function(Box::new(function)));
                
                // Create closure
                self.emit(OpCode::Closure(function_idx));
                
                // Define the method on the container
                self.emit(OpCode::DefineMethod);
            }
        }
        
        Ok(())
    }

    // Compile action (function) definition
    fn compile_action_definition(&mut self, action: Statement) -> Result<(), CompileError> {
        if let Statement::ActionDefinition { name, parameters, return_type: _, body, is_async, is_private: _ } = action {
            // Create a compiler for the function
            let mut function_compiler = BytecodeCompiler::new(&name);
            function_compiler.current_line = self.current_line;
            
            // Set arity and async flag
            function_compiler.function.arity = parameters.len();
            function_compiler.function.is_async = is_async;
            
            // Add parameters
            function_compiler.begin_scope();
            for param in parameters {
                function_compiler.add_parameter(param.name)?;
            }
            
            // Compile body
            for stmt in body {
                function_compiler.compile_statement(stmt)?;
            }
            
            // Get the compiled function
            let function = function_compiler.end();
            
            // Add to constants
            let function_idx = self.add_constant(Constant::Function(Box::new(function)));
            
            // Create closure
            self.emit(OpCode::Closure(function_idx));
            
            // Define function name
            self.define_variable(&name)?;
            
            Ok(())
        } else {
            unreachable!("compile_action_definition called with non-action statement");
        }
    }
} 