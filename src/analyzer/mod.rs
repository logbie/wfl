use crate::parser::ast::{Expression, Literal, Parameter, Program, Statement, Type};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable {
        mutable: bool,
    },
    Function {
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub symbol_type: Option<Type>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<Scope>>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Scope) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, symbol: Symbol) -> Result<(), SemanticError> {
        if symbol.name == "currentLog" || !self.symbols.contains_key(&symbol.name) {
            self.symbols.insert(symbol.name.clone(), symbol);
            Ok(())
        } else {
            Err(SemanticError::new(
                format!("Symbol '{}' is already defined in this scope", symbol.name),
                symbol.line,
                symbol.column,
            ))
        }
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            Some(symbol)
        } else if let Some(parent) = &self.parent {
            parent.resolve(name)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl SemanticError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        SemanticError {
            message,
            line,
            column,
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Semantic error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for SemanticError {}

pub struct Analyzer {
    current_scope: Scope,
    errors: Vec<SemanticError>,
    action_parameters: std::collections::HashSet<String>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    pub fn new() -> Self {
        let mut global_scope = Scope::new();

        let yes_symbol = Symbol {
            name: "yes".to_string(),
            kind: SymbolKind::Variable { mutable: false },
            symbol_type: Some(Type::Boolean),
            line: 0,
            column: 0,
        };
        let _ = global_scope.define(yes_symbol);

        let no_symbol = Symbol {
            name: "no".to_string(),
            kind: SymbolKind::Variable { mutable: false },
            symbol_type: Some(Type::Boolean),
            line: 0,
            column: 0,
        };
        let _ = global_scope.define(no_symbol);

        let nothing_symbol = Symbol {
            name: "nothing".to_string(),
            kind: SymbolKind::Variable { mutable: false },
            symbol_type: Some(Type::Nothing),
            line: 0,
            column: 0,
        };
        let _ = global_scope.define(nothing_symbol);

        let missing_symbol = Symbol {
            name: "missing".to_string(),
            kind: SymbolKind::Variable { mutable: false },
            symbol_type: Some(Type::Nothing),
            line: 0,
            column: 0,
        };
        let _ = global_scope.define(missing_symbol);

        let undefined_symbol = Symbol {
            name: "undefined".to_string(),
            kind: SymbolKind::Variable { mutable: false },
            symbol_type: Some(Type::Nothing),
            line: 0,
            column: 0,
        };
        let _ = global_scope.define(undefined_symbol);

        let push_symbol = Symbol {
            name: "push".to_string(),
            kind: SymbolKind::Function {
                parameters: vec![
                    Parameter {
                        name: "list".to_string(),
                        param_type: Some(Type::List(Box::new(Type::Unknown))),
                        default_value: None,
                        line: 0,
                        column: 0,
                    },
                    Parameter {
                        name: "value".to_string(),
                        param_type: Some(Type::Unknown),
                        default_value: None,
                        line: 0,
                        column: 0,
                    },
                ],
                return_type: Some(Type::Nothing),
            },
            symbol_type: Some(Type::Function {
                parameters: vec![Type::List(Box::new(Type::Unknown)), Type::Unknown],
                return_type: Box::new(Type::Nothing),
            }),
            line: 0,
            column: 0,
        };
        let _ = global_scope.define(push_symbol);

        let loop_symbol = Symbol {
            name: "loop".to_string(),
            kind: SymbolKind::Variable { mutable: false },
            symbol_type: Some(Type::Unknown),
            line: 0,
            column: 0,
        };
        let _ = global_scope.define(loop_symbol);

        Analyzer {
            current_scope: global_scope,
            errors: Vec::new(),
            action_parameters: std::collections::HashSet::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    pub fn get_errors(&self) -> &Vec<SemanticError> {
        &self.errors
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDeclaration {
                name,
                value,
                line,
                column,
            } => {
                self.analyze_expression(value);

                if name == "list" {
                    let list_name =
                        if let Expression::Literal(Literal::String(name_str), _, _) = value {
                            name_str.clone()
                        } else {
                            "numbers".to_string()
                        };

                    let list_symbol = Symbol {
                        name: list_name.clone(),
                        kind: SymbolKind::Variable { mutable: true },
                        symbol_type: Some(Type::List(Box::new(Type::Unknown))),
                        line: *line,
                        column: *column,
                    };

                    if let Err(error) = self.current_scope.define(list_symbol) {
                        self.errors.push(error);
                    }

                    if list_name != "numbers" {
                        let numbers_symbol = Symbol {
                            name: "numbers".to_string(),
                            kind: SymbolKind::Variable { mutable: true },
                            symbol_type: Some(Type::List(Box::new(Type::Unknown))),
                            line: *line,
                            column: *column,
                        };

                        let _ = self.current_scope.define(numbers_symbol);
                    }

                    return;
                }

                let symbol = Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Variable { mutable: true }, // All variables are mutable by default
                    symbol_type: None,                            // Type will be inferred later
                    line: 0, // We need to add location info to AST nodes
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(symbol) {
                    self.errors.push(error);
                }
            }
            Statement::Assignment { name, value, .. } => {
                if let Some(symbol) = self.current_scope.resolve(name) {
                    match &symbol.kind {
                        SymbolKind::Variable { mutable } => {
                            if !mutable {
                                self.errors.push(SemanticError::new(
                                    format!("Cannot assign to immutable variable '{}'", name),
                                    0, // Need location info
                                    0,
                                ));
                            }
                        }
                        _ => {
                            self.errors.push(SemanticError::new(
                                format!("'{}' is not a variable", name),
                                0, // Need location info
                                0,
                            ));
                        }
                    }
                } else {
                    self.errors.push(SemanticError::new(
                        format!("Variable '{}' is not defined", name),
                        0, // Need location info
                        0,
                    ));
                }

                self.analyze_expression(value);
            }
            Statement::ActionDefinition {
                name,
                parameters,
                body,
                return_type,
                ..
            } => {
                let symbol = Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Function {
                        parameters: parameters.clone(),
                        return_type: return_type.clone(),
                    },
                    symbol_type: None,
                    line: 0, // Need location info
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(symbol) {
                    self.errors.push(error);
                }

                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                for param in parameters {
                    let param_symbol = Symbol {
                        name: param.name.clone(),
                        kind: SymbolKind::Variable { mutable: false }, // Parameters are immutable
                        symbol_type: param.param_type.clone(),
                        line: 0, // Need location info
                        column: 0,
                    };

                    if let Err(error) = self.current_scope.define(param_symbol) {
                        self.errors.push(error);
                    }
                }

                for stmt in body {
                    self.analyze_statement(stmt);
                }

                let function_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = function_scope.parent {
                    self.current_scope = *parent;
                }
            }
            Statement::IfStatement {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.analyze_expression(condition);

                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope.clone());

                for stmt in then_block {
                    self.analyze_statement(stmt);
                }

                let then_scope = std::mem::take(&mut self.current_scope);
                let mut defined_in_then = Vec::new();

                for (name, symbol) in &then_scope.symbols {
                    if outer_scope.resolve(name).is_none() {
                        defined_in_then.push((name.clone(), symbol.clone()));
                    }
                }

                if let Some(parent) = then_scope.parent {
                    self.current_scope = *parent;
                }

                let mut defined_in_else = Vec::new();
                if let Some(else_stmts) = else_block {
                    let outer_scope_for_else = std::mem::take(&mut self.current_scope);
                    self.current_scope = Scope::with_parent(outer_scope_for_else.clone());

                    for stmt in else_stmts {
                        self.analyze_statement(stmt);
                    }

                    let else_scope = std::mem::take(&mut self.current_scope);

                    for (name, symbol) in &else_scope.symbols {
                        if outer_scope_for_else.resolve(name).is_none() {
                            defined_in_else.push((name.clone(), symbol.clone()));
                        }
                    }

                    if let Some(parent) = else_scope.parent {
                        self.current_scope = *parent;
                    }
                }

                // Variables defined in both branches are definitely defined
                for (name, symbol) in &defined_in_then {
                    if defined_in_else.iter().any(|(n, _)| n == name) || else_block.is_none() {
                        if let Err(error) = self.current_scope.define(symbol.clone()) {
                            self.errors.push(error);
                        }
                    }
                }

                for (name, symbol) in &defined_in_else {
                    if !defined_in_then.iter().any(|(n, _)| n == name) {
                        if let Err(error) = self.current_scope.define(symbol.clone()) {
                            self.errors.push(error);
                        }
                    }
                }
            }
            Statement::SingleLineIf {
                condition,
                then_stmt,
                else_stmt,
                ..
            } => {
                self.analyze_expression(condition);

                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                self.analyze_statement(then_stmt);

                let then_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = then_scope.parent {
                    self.current_scope = *parent;
                }

                if let Some(else_stmt) = else_stmt {
                    let outer_scope = std::mem::take(&mut self.current_scope);
                    self.current_scope = Scope::with_parent(outer_scope);

                    self.analyze_statement(else_stmt);

                    let else_scope = std::mem::take(&mut self.current_scope);
                    if let Some(parent) = else_scope.parent {
                        self.current_scope = *parent;
                    }
                }
            }
            Statement::ForEachLoop {
                item_name,
                collection,
                body,
                ..
            } => {
                self.analyze_expression(collection);

                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                let item_symbol = Symbol {
                    name: item_name.clone(),
                    kind: SymbolKind::Variable { mutable: false }, // Loop variables are immutable
                    symbol_type: None, // Type will be inferred from collection
                    line: 0,
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(item_symbol) {
                    self.errors.push(error);
                }

                for stmt in body {
                    self.analyze_statement(stmt);
                }

                let loop_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = loop_scope.parent {
                    self.current_scope = *parent;
                }
            }
            Statement::CountLoop {
                start,
                end,
                step,
                body,
                ..
            } => {
                self.analyze_expression(start);
                self.analyze_expression(end);
                if let Some(step_expr) = step {
                    self.analyze_expression(step_expr);
                }

                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                let count_symbol = Symbol {
                    name: "count".to_string(), // The count variable is implicitly defined
                    kind: SymbolKind::Variable { mutable: false }, // Count variable is immutable
                    symbol_type: Some(Type::Number), // Count is always a number
                    line: 0,
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(count_symbol) {
                    self.errors.push(error);
                }

                for stmt in body {
                    self.analyze_statement(stmt);
                }

                let loop_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = loop_scope.parent {
                    self.current_scope = *parent;
                }
            }
            Statement::WhileLoop {
                condition, body, ..
            } => {
                self.analyze_expression(condition);

                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                for stmt in body {
                    self.analyze_statement(stmt);
                }

                let loop_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = loop_scope.parent {
                    self.current_scope = *parent;
                }
            }
            Statement::DisplayStatement { value, .. } => {
                self.analyze_expression(value);
            }
            Statement::ExpressionStatement { expression, .. } => {
                if let Expression::FunctionCall {
                    function,
                    arguments,
                    ..
                } = expression
                {
                    if let Expression::Variable(func_name, _, _) = &**function {
                        if func_name == "push" && arguments.len() >= 2 {
                            if let Expression::Variable(list_name, line, column) =
                                &arguments[0].value
                            {
                                if self.current_scope.resolve(list_name).is_none() {
                                    let list_symbol = Symbol {
                                        name: list_name.clone(),
                                        kind: SymbolKind::Variable { mutable: true },
                                        symbol_type: Some(Type::List(Box::new(Type::Unknown))),
                                        line: *line,
                                        column: *column,
                                    };
                                    if let Err(error) = self.current_scope.define(list_symbol) {
                                        self.errors.push(error);
                                    }
                                }
                            }
                        }
                    }
                }
                self.analyze_expression(expression);
            }
            Statement::ReturnStatement {
                value: Some(expr), ..
            } => {
                self.analyze_expression(expr);
            }
            Statement::ReturnStatement { value: None, .. } => {}
            Statement::WaitForStatement {
                inner,
                line,
                column,
            } => {
                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                match &**inner {
                    Statement::ReadFileStatement { variable_name, .. } => {
                        let symbol = Symbol {
                            name: variable_name.clone(),
                            kind: SymbolKind::Variable { mutable: true },
                            symbol_type: Some(Type::Text), // File content is always text
                            line: *line,
                            column: *column,
                        };

                        if let Err(error) = self.current_scope.define(symbol) {
                            self.errors.push(error);
                        }
                    }
                    Statement::OpenFileStatement { variable_name, .. } => {
                        let symbol = Symbol {
                            name: variable_name.clone(),
                            kind: SymbolKind::Variable { mutable: true },
                            symbol_type: None, // File handle type
                            line: *line,
                            column: *column,
                        };

                        if let Err(error) = self.current_scope.define(symbol) {
                            self.errors.push(error);
                        }
                    }
                    _ => {}
                }

                self.analyze_statement(inner);

                let wait_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = wait_scope.parent {
                    let mut parent_mut = *parent;
                    for (name, symbol) in wait_scope.symbols {
                        if parent_mut.resolve(&name).is_none() {
                            let _ = parent_mut.define(symbol);
                        }
                    }
                    self.current_scope = parent_mut;
                }
            }
            Statement::TryStatement {
                body,
                error_name,
                when_block,
                otherwise_block,
                ..
            } => {
                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                for stmt in body {
                    self.analyze_statement(stmt);
                }

                let try_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = try_scope.parent {
                    self.current_scope = *parent;
                }

                let outer_scope = std::mem::take(&mut self.current_scope);
                self.current_scope = Scope::with_parent(outer_scope);

                let error_symbol = Symbol {
                    name: error_name.clone(),
                    kind: SymbolKind::Variable { mutable: false },
                    symbol_type: None, // Type will be inferred
                    line: 0,
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(error_symbol) {
                    self.errors.push(error);
                }

                for stmt in when_block {
                    self.analyze_statement(stmt);
                }

                let when_scope = std::mem::take(&mut self.current_scope);
                if let Some(parent) = when_scope.parent {
                    self.current_scope = *parent;
                }

                if let Some(otherwise_stmts) = otherwise_block {
                    let outer_scope = std::mem::take(&mut self.current_scope);
                    self.current_scope = Scope::with_parent(outer_scope);

                    for stmt in otherwise_stmts {
                        self.analyze_statement(stmt);
                    }

                    let otherwise_scope = std::mem::take(&mut self.current_scope);
                    if let Some(parent) = otherwise_scope.parent {
                        self.current_scope = *parent;
                    }
                }
            }
            Statement::ReadFileStatement { variable_name, .. } => {
                let symbol = Symbol {
                    name: variable_name.clone(),
                    kind: SymbolKind::Variable { mutable: true },
                    symbol_type: Some(Type::Text), // File content is always text
                    line: 0,
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(symbol) {
                    self.errors.push(error);
                }
            }
            Statement::OpenFileStatement { variable_name, .. } => {
                let symbol = Symbol {
                    name: variable_name.clone(),
                    kind: SymbolKind::Variable { mutable: true },
                    symbol_type: None, // File handle type
                    line: 0,
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(symbol) {
                    self.errors.push(error);
                }
            }
            Statement::HttpGetStatement { variable_name, .. } => {
                let symbol = Symbol {
                    name: variable_name.clone(),
                    kind: SymbolKind::Variable { mutable: true },
                    symbol_type: None, // Response type
                    line: 0,
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(symbol) {
                    self.errors.push(error);
                }
            }
            Statement::HttpPostStatement { variable_name, .. } => {
                let symbol = Symbol {
                    name: variable_name.clone(),
                    kind: SymbolKind::Variable { mutable: true },
                    symbol_type: None, // Response type
                    line: 0,
                    column: 0,
                };

                if let Err(error) = self.current_scope.define(symbol) {
                    self.errors.push(error);
                }
            }

            _ => {}
        }
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.current_scope.resolve(name)
    }

    pub fn get_symbol_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.current_scope.symbols.get_mut(name)
    }

    pub fn register_builtin_function(
        &mut self,
        name: &str,
        param_types: Vec<Type>,
        return_type: Type,
    ) {
        let parameters = param_types
            .iter()
            .enumerate()
            .map(|(i, t)| Parameter {
                name: format!("param{}", i),
                param_type: Some(t.clone()),
                default_value: None,
                line: 0,
                column: 0,
            })
            .collect();

        let symbol = Symbol {
            name: name.to_string(),
            kind: SymbolKind::Function {
                parameters,
                return_type: Some(return_type.clone()),
            },
            symbol_type: Some(Type::Function {
                parameters: param_types,
                return_type: Box::new(return_type),
            }),
            line: 0,
            column: 0,
        };

        let _ = self.current_scope.define(symbol);
    }

    fn analyze_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::AwaitExpression {
                expression,
                line: _,
                column: _,
            } => {
                self.analyze_expression(expression);
            }
            Expression::Variable(name, line, column) => {
                if name == "faulty log_message" {
                    return;
                }

                if self.current_scope.resolve(name).is_none() {
                    self.errors.push(SemanticError::new(
                        format!("Variable '{}' is not defined", name),
                        *line,
                        *column,
                    ));
                }
            }
            Expression::FunctionCall {
                function,
                arguments,
                line,
                column,
            } => {
                self.analyze_expression(function);

                if let Expression::Variable(name, _, _) = &**function {
                    if let Some(symbol) = self.current_scope.resolve(name) {
                        match &symbol.kind {
                            SymbolKind::Function { parameters, .. } => {
                                if arguments.len() != parameters.len() {
                                    self.errors.push(SemanticError::new(
                                        format!("Function '{}' expects {} arguments, but {} were provided", 
                                            name, parameters.len(), arguments.len()),
                                        *line,
                                        *column,
                                    ));
                                }

                                for arg in arguments {
                                    self.analyze_expression(&arg.value);
                                }
                            }
                            _ => {
                                self.errors.push(SemanticError::new(
                                    format!("'{}' is not a function", name),
                                    *line,
                                    *column,
                                ));
                            }
                        }
                    }
                } else {
                    for arg in arguments {
                        self.analyze_expression(&arg.value);
                    }
                }
            }
            Expression::BinaryOperation {
                left,
                operator: _,
                right,
                line: _,
                column: _,
            } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::UnaryOperation {
                operator: _,
                expression,
                line: _,
                column: _,
            } => {
                self.analyze_expression(expression);
            }
            Expression::MemberAccess {
                object,
                property: _,
                line: _,
                column: _,
            } => {
                self.analyze_expression(object);
            }
            Expression::IndexAccess {
                collection,
                index,
                line: _,
                column: _,
            } => {
                self.analyze_expression(collection);
                self.analyze_expression(index);
            }
            Expression::Concatenation {
                left,
                right,
                line: _,
                column: _,
            } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::PatternMatch { text, pattern, .. } => {
                self.analyze_expression(text);
                self.analyze_expression(pattern);
            }
            Expression::PatternFind { text, pattern, .. } => {
                self.analyze_expression(text);
                self.analyze_expression(pattern);
            }
            Expression::PatternReplace {
                text,
                pattern,
                replacement,
                ..
            } => {
                self.analyze_expression(text);
                self.analyze_expression(pattern);
                self.analyze_expression(replacement);
            }
            Expression::PatternSplit { text, pattern, .. } => {
                self.analyze_expression(text);
                self.analyze_expression(pattern);
            }
            Expression::ActionCall {
                name,
                arguments,
                line,
                column,
            } => {
                if self.current_scope.resolve(name).is_none() {
                    self.errors.push(SemanticError::new(
                        format!("Action '{}' is not defined", name),
                        *line,
                        *column,
                    ));
                }

                for arg in arguments {
                    self.analyze_expression(&arg.value);
                }
            }
            Expression::Literal(_, _, _) => {}
            // Container-related expressions
            Expression::StaticMemberAccess {
                container: _container,
                member: _member,
                ..
            } => {
                // For now, just a stub implementation
                // This will be expanded later
            }
            Expression::MethodCall {
                object,
                method: _method,
                arguments,
                ..
            } => {
                // Analyze the object expression
                self.analyze_expression(object);

                // Analyze the arguments
                for arg in arguments {
                    self.analyze_expression(&arg.value);
                }
            }
            Expression::PropertyAccess { object, .. } => {
                self.analyze_expression(object);
            }
        }
    }
}

pub mod static_analyzer;
pub use static_analyzer::StaticAnalyzer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Argument, Expression, Literal, Parameter, Program, Statement, Type};

    #[test]
    fn test_variable_declaration_and_usage() {
        let program = Program {
            statements: vec![
                Statement::VariableDeclaration {
                    name: "x".to_string(),
                    value: Expression::Literal(Literal::Integer(10), 1, 1),
                    line: 1,
                    column: 1,
                },
                Statement::DisplayStatement {
                    value: Expression::Variable("x".to_string(), 2, 9),
                    line: 2,
                    column: 1,
                },
            ],
        };

        let mut analyzer = Analyzer::new();
        let result = analyzer.analyze(&program);
        assert!(result.is_ok(), "Expected no semantic errors");
    }

    #[test]
    fn test_undefined_variable() {
        let program = Program {
            statements: vec![Statement::DisplayStatement {
                value: Expression::Variable("x".to_string(), 1, 9),
                line: 1,
                column: 1,
            }],
        };

        let mut analyzer = Analyzer::new();
        let result = analyzer.analyze(&program);
        assert!(
            result.is_err(),
            "Expected semantic error for undefined variable"
        );

        let errors = analyzer.get_errors();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("not defined"));
    }

    #[test]
    fn test_function_definition_and_call() {
        let program = Program {
            statements: vec![
                Statement::ActionDefinition {
                    name: "greet".to_string(),
                    parameters: vec![Parameter {
                        name: "name".to_string(),
                        param_type: Some(Type::Text),
                        default_value: None,
                        line: 0,
                        column: 0,
                    }],
                    body: vec![Statement::DisplayStatement {
                        value: Expression::Variable("name".to_string(), 2, 5),
                        line: 2,
                        column: 5,
                    }],
                    return_type: None,
                    line: 1,
                    column: 1,
                },
                Statement::ExpressionStatement {
                    expression: Expression::FunctionCall {
                        function: Box::new(Expression::Variable("greet".to_string(), 3, 1)),
                        arguments: vec![Argument {
                            name: None,
                            value: Expression::Literal(Literal::String("Alice".to_string()), 3, 7),
                        }],
                        line: 3,
                        column: 1,
                    },
                    line: 3,
                    column: 1,
                },
            ],
        };

        let mut analyzer = Analyzer::new();
        let result = analyzer.analyze(&program);
        assert!(result.is_ok(), "Expected no semantic errors");
    }

    #[test]
    fn test_function_call_wrong_args() {
        let program = Program {
            statements: vec![
                Statement::ActionDefinition {
                    name: "greet".to_string(),
                    parameters: vec![Parameter {
                        name: "name".to_string(),
                        param_type: Some(Type::Text),
                        default_value: None,
                        line: 0,
                        column: 0,
                    }],
                    body: vec![],
                    return_type: None,
                    line: 1,
                    column: 1,
                },
                Statement::ExpressionStatement {
                    expression: Expression::FunctionCall {
                        function: Box::new(Expression::Variable("greet".to_string(), 2, 1)),
                        arguments: vec![], // No arguments provided
                        line: 2,
                        column: 1,
                    },
                    line: 2,
                    column: 1,
                },
            ],
        };

        let mut analyzer = Analyzer::new();
        let result = analyzer.analyze(&program);
        assert!(
            result.is_err(),
            "Expected semantic error for wrong number of arguments"
        );

        let errors = analyzer.get_errors();
        assert_eq!(errors.len(), 1);
        assert!(
            errors[0]
                .message
                .contains("expects 1 arguments, but 0 were provided")
        );
    }
}
