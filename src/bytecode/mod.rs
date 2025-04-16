use std::collections::HashMap;
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

/// A constant value in the bytecode
#[derive(Debug, Clone)]
pub enum Constant {
    Number(f64),
    String(String),
    Boolean(bool),
    Nothing,
    Function(Box<Function>),
    Container(String, HashMap<String, usize>),
}

/// Chunk represents a compiled block of bytecode
#[derive(Debug, Clone)]
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

/// Function represents a compiled function with its bytecode
#[derive(Debug, Clone)]
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

/// The bytecode compiler
pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
    label_counter: usize,
    enclosing: Option<Box<Compiler>>,
}

impl Compiler {
    /// Create a new compiler
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
            label_counter: 0,
            enclosing: None,
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
        self.emit(OpCode::Return);

        if errors.is_empty() {
            Ok(self.function.clone())
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
            }
            Statement::VarDeclaration(name, initializer) => {
                self.compile_var_declaration(name, initializer)
            }
            Statement::Block(statements) => self.compile_block(statements),
            Statement::IfStatement(condition, then_branch, else_branch) => {
                self.compile_if_statement(condition, then_branch, else_branch)
            }
            Statement::WhileStatement(condition, body) => {
                self.compile_while_statement(condition, body)
            }
            Statement::PrintStatement(expr) => {
                self.compile_expression(expr)?;
                self.emit(OpCode::Print);
                Ok(())
            }
            Statement::ReturnStatement(expr) => {
                match expr {
                    Some(expr) => self.compile_expression(expr)?,
                    None => self.emit(OpCode::Null)
                }
                self.emit(OpCode::Return);
                Ok(())
            }
            Statement::ActionDefinition(action) => {
                self.compile_action_definition(action)
            }
            Statement::ContainerDefinition(name, properties, methods) => {
                self.compile_container_definition(name, properties, methods)
            }
            _ => Err(CompileError::NotImplemented(format!("Unsupported statement type")))
        }
    }

    /// Compile an expression to bytecode
    fn compile_expression(&mut self, expr: Expression) -> Result<(), CompileError> {
        match expr {
            Expression::Literal(literal) => {
                match literal {
                    Literal::Number(n) => self.emit_constant(Value::Number(n)),
                    Literal::String(s) => self.emit_constant(Value::String(s)),
                    Literal::Boolean(b) => self.emit_constant(Value::Boolean(b)),
                    Literal::Null => self.emit_constant(Value::Null),
                }
            }
            Expression::Variable(name) => {
                let var_idx = self.add_identifier(name.clone());
                self.emit(OpCode::GetVariable);
                self.emit_byte(var_idx);
            }
            Expression::Assignment(name, value) => {
                self.compile_expression(*value)?;
                let var_idx = self.add_identifier(name.clone());
                self.emit(OpCode::SetVariable);
                self.emit_byte(var_idx);
            }
            Expression::Binary(left, operator, right) => {
                self.compile_expression(*left)?;
                self.compile_expression(*right)?;
                
                match operator {
                    BinaryOperator::Plus => self.emit(OpCode::Add),
                    BinaryOperator::Minus => self.emit(OpCode::Subtract),
                    BinaryOperator::Multiply => self.emit(OpCode::Multiply),
                    BinaryOperator::Divide => self.emit(OpCode::Divide),
                    BinaryOperator::Equal => self.emit(OpCode::Equal),
                    BinaryOperator::NotEqual => {
                        self.emit(OpCode::Equal);
                        self.emit(OpCode::Not);
                    }
                    BinaryOperator::Greater => self.emit(OpCode::Greater),
                    BinaryOperator::GreaterEqual => self.emit(OpCode::GreaterEqual),
                    BinaryOperator::Less => self.emit(OpCode::Less),
                    BinaryOperator::LessEqual => self.emit(OpCode::LessEqual),
                    BinaryOperator::And => self.emit(OpCode::And),
                    BinaryOperator::Or => self.emit(OpCode::Or),
                }
            }
            Expression::Unary(operator, expr) => {
                self.compile_expression(*expr)?;
                
                match operator {
                    UnaryOperator::Minus => self.emit(OpCode::Negate),
                    UnaryOperator::Not => self.emit(OpCode::Not),
                }
            }
            Expression::Call(callee, args, named_args) => {
                self.compile_call(*callee, args, named_args)?;
            }
            Expression::Get(object, property) => {
                self.compile_expression(*object)?;
                
                // For property access, the property name is a string
                if let Expression::Variable(name) = *property {
                    self.emit_constant(Value::String(name));
                    self.emit(OpCode::GetProperty);
                } else {
                    return Err(CompileError::new("Expected property name"));
                }
            }
            Expression::Index(object, index) => {
                self.compile_expression(*object)?;
                self.compile_expression(*index)?;
                self.emit(OpCode::GetIndex);
            }
            Expression::List(items) => {
                self.compile_list(items)?;
            }
            Expression::Map(entries) => {
                self.compile_map(entries)?;
            }
            _ => return Err(CompileError::new(&format!("Unsupported expression: {:?}", expr))),
        }
        
        Ok(())
    }
    
    /// Compile a function call expression
    fn compile_call(&mut self, callee: Expression, args: Vec<Expression>, named_args: Option<Vec<(String, Expression)>>) -> Result<(), CompileError> {
        // Compile the callee expression to get a function reference
        self.compile_expression(callee)?;
        
        let arg_count = args.len();
        
        // Handle named arguments if present
        if let Some(named) = named_args {
            // Create a new object for the named arguments
            self.emit(OpCode::NewMap);
            
            // Add each named argument as a property
            for (name, value) in named {
                // Duplicate the map
                self.emit(OpCode::Duplicate);
                
                // Push the property name
                self.emit_constant(Value::String(name));
                
                // Compile and push the value
                self.compile_expression(value)?;
                
                // Set the property
                self.emit(OpCode::SetProperty);
            }
            
            // Compile positional arguments
            for arg in args {
                self.compile_expression(arg)?;
            }
            
            // Call with positional args + 1 for named args object
            self.emit_with_operand(OpCode::Call, (arg_count + 1) as u8);
        } else {
            // Compile each argument
            for arg in args {
                self.compile_expression(arg)?;
            }
            
            // Call with the number of arguments
            self.emit_with_operand(OpCode::Call, arg_count as u8);
        }
        
        Ok(())
    }
    
    /// Compile a container definition
    fn compile_container(&mut self, name: String, fields: Vec<(String, Option<Expression>)>, 
                         methods: Vec<(String, Vec<String>, Vec<Statement>)>) -> Result<(), CompileError> {
        // Create a new container prototype
        self.emit(OpCode::NewContainer);

        // Store the container prototype in a local variable
        let container_local = self.declare_local(name.clone())?;
        self.define_local(container_local);

        // Add each field with its default value if provided
        for (field_name, default_value) in fields {
            // Duplicate the container reference
            self.emit(OpCode::Duplicate);
            
            // Push the field name
            self.emit_constant(Value::String(field_name));
            
            // Compile default value or use nil
            if let Some(expr) = default_value {
                self.compile_expression(expr)?;
            } else {
                self.emit_constant(Value::Nil);
            }
            
            // Define the field
            self.emit(OpCode::DefineField);
        }

        // Add each method
        for (method_name, params, body) in methods {
            // Duplicate container reference
            self.emit(OpCode::Duplicate);
            
            // Push method name
            self.emit_constant(Value::String(method_name));
            
            // Create a new function for the method
            let function_index = self.add_function(method_name.clone(), params, body)?;
            
            // Create the function object
            self.emit_with_operand(OpCode::Closure, function_index as u8);
            
            // Define the method on the container
            self.emit(OpCode::DefineMethod);
        }

        Ok(())
    }
    
    /// Compile an action (function) definition
    fn compile_action_definition(&mut self, action: ActionDef) -> Result<(), CompileError> {
        // Create a new function object
        let function_name = action.name.clone();
        let function = self.begin_function(function_name)?;
        
        // Add parameters to the local scope
        self.begin_scope();
        for param in action.params.iter() {
            self.add_local(param.name.clone())?;
        }
        
        // Compile function body
        for stmt in action.body {
            self.compile_statement(stmt)?;
        }
        
        // Handle implicit return if there isn't one
        if !self.chunk.code.ends_with(&[OpCode::Return as u8]) {
            self.emit(OpCode::Null);
            self.emit(OpCode::Return);
        }
        
        // End function and create a closure from it
        let function = self.end_function()?;
        
        // Get the function index (to be filled in when the function ends)
        let function_idx = self.add_constant(Value::Function(function))?;
        
        // Create the closure
        self.emit(OpCode::Closure);
        self.emit(function_idx as u8);
        
        // Define the variable
        self.define_variable(&action.name)?;
        
        Ok(())
    }

    /// Add a function to the function table and return its index
    fn add_function(&mut self, name: String, params: Vec<String>, body: Vec<Statement>) -> Result<usize, CompileError> {
        // Create a new compiler for the function scope
        let mut function_compiler = BytecodeCompiler::new();
        
        // Add parameters as locals
        for param in &params {
            let param_local = function_compiler.declare_local(param.clone())?;
            function_compiler.define_local(param_local);
        }
        
        // Compile function body
        for statement in body {
            function_compiler.compile_statement(statement)?;
        }
        
        // Add implicit return nil if needed
        if !function_compiler.last_instruction_is_return() {
            function_compiler.emit_constant(Value::Nil);
            function_compiler.emit(OpCode::Return);
        }
        
        // Get the compiled function chunk
        let function = Function::new(
            name,
            params.len(),
            function_compiler.chunk(),
            params,
        );
        
        // Add to function table
        let function_index = self.functions.len();
        self.functions.push(function);
        
        Ok(function_index)
    }
    
    /// Check if the last instruction is a return
    fn last_instruction_is_return(&self) -> bool {
        if self.chunk.code.is_empty() {
            return false;
        }
        
        *self.chunk.code.last().unwrap() == OpCode::Return as u8
    }

    /// Compile a list expression
    fn compile_list(&mut self, items: Vec<Expression>) -> Result<(), CompileError> {
        // Create a new list
        self.emit(OpCode::NewList);
        
        // For each item, duplicate the list reference, compile the item, and add it
        for item in items {
            // Duplicate the list reference for the add operation
            self.emit(OpCode::Duplicate);
            
            // Compile the item
            self.compile_expression(item)?;
            
            // Add to list
            self.emit(OpCode::AddList);
        }
        
        Ok(())
    }
    
    /// Compile a map expression
    fn compile_map(&mut self, entries: Vec<(Expression, Expression)>) -> Result<(), CompileError> {
        // Create a new map
        self.emit(OpCode::NewMap);
        
        // For each entry, duplicate the map reference, compile the key and value, and set them
        for (key, value) in entries {
            // Duplicate the map reference for the set operation
            self.emit(OpCode::Duplicate);
            
            // Compile the key (convert to string if it's a variable)
            match key {
                Expression::Variable(name) => {
                    self.emit_constant(Value::String(name));
                },
                _ => {
                    // For other expressions, compile them directly
                    self.compile_expression(key)?;
                }
            }
            
            // Compile the value
            self.compile_expression(value)?;
            
            // Set in map
            self.emit(OpCode::SetProperty);
        }
        
        Ok(())
    }

    /// Compile a list initialization expression
    fn compile_list_init(&mut self, elements: Vec<Expression>) -> Result<(), CompileError> {
        // Create a new list
        self.emit(OpCode::NewList);
        
        // Add each element to the list
        for element in elements {
            // Duplicate the list reference
            self.emit(OpCode::Duplicate);
            
            // Compile the element
            self.compile_expression(element)?;
            
            // Add the element to the list
            self.emit(OpCode::ListAppend);
        }
        
        Ok(())
    }
    
    /// Compile a map initialization expression
    fn compile_map_init(&mut self, entries: Vec<(Expression, Expression)>) -> Result<(), CompileError> {
        // Create a new map
        self.emit(OpCode::NewMap);
        
        // Add each key-value pair to the map
        for (key, value) in entries {
            // Duplicate the map reference
            self.emit(OpCode::Duplicate);
            
            // Compile the key
            self.compile_expression(key)?;
            
            // Compile the value
            self.compile_expression(value)?;
            
            // Set the property in the map
            self.emit(OpCode::SetProperty);
        }
        
        Ok(())
    }

    /// Compile an index expression (object[index])
    fn compile_index(&mut self, object: Expression, index: Expression) -> Result<(), CompileError> {
        // Compile the object expression
        self.compile_expression(object)?;
        
        // Compile the index expression
        self.compile_expression(index)?;
        
        // Get the property
        self.emit(OpCode::GetProperty);
        
        Ok(())
    }

    /// Emit an instruction with the current line number
    fn emit(&mut self, op: OpCode) {
        self.function.chunk.write(op, self.current_line);
    }

    /// Add a constant and return its index
    fn add_constant(&mut self, value: Constant) -> usize {
        self.function.chunk.add_constant(value)
    }

    /// Compile a container definition
    fn compile_container_definition(&mut self, name: String, properties: Vec<(String, Expression)>, methods: Vec<ActionDefinition>) -> Result<(), CompileError> {
        // Create a new container prototype (like a class)
        self.emit(OpCode::NewContainer);
        
        // Store the container in a global variable
        let global_idx = self.add_or_get_global(name);
        self.emit_byte(OpCode::DefineGlobal, global_idx);
        
        // Retrieve the container for adding properties and methods
        self.emit_byte(OpCode::GetGlobal, global_idx);
        
        // Add default properties
        for (prop_name, default_value) in properties {
            // Duplicate the container reference
            self.emit(OpCode::Duplicate);
            
            // Push the property name
            let constant_idx = self.add_constant(Value::String(prop_name.clone()));
            self.emit_byte(OpCode::Constant, constant_idx);
            
            // Compile the default value
            self.compile_expression(default_value)?;
            
            // Set the property
            self.emit(OpCode::SetProperty);
        }
        
        // Add methods to the container
        for method in methods {
            // Duplicate the container reference
            self.emit(OpCode::Duplicate);
            
            // Push the method name
            let constant_idx = self.add_constant(Value::String(method.name.clone()));
            self.emit_byte(OpCode::Constant, constant_idx);
            
            // Compile the method
            self.compile_action_definition(method)?;
            
            // Set the method as a property
            self.emit(OpCode::SetProperty);
        }
        
        // Pop the container reference
        self.emit(OpCode::Pop);
        
        Ok(())
    }

    /// Begin compiling a new function
    fn begin_function(&mut self, name: String) -> Result<Function, CompileError> {
        // Save the current compiler state
        let enclosing = std::mem::replace(
            &mut self.function,
            Function::new(name, false)
        );
        self.enclosing = Some(Box::new(enclosing));
        
        // Get the function index in the constants table
        0 // This will be filled in later when we end the function
    }
    
    /// End function compilation and return to the enclosing function
    fn end_function(&mut self) -> Result<Function, CompileError> {
        // Add an implicit return
        self.emit(OpCode::Null);
        self.emit(OpCode::Return);
        
        // Swap back to the enclosing function
        let function = std::mem::replace(
            &mut self.function,
            *self.enclosing.take().expect("Enclosing function not found")
        );
        
        Ok(function)
    }
} 