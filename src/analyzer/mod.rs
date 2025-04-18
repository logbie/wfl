use crate::parser::ast::{Expression, Parameter, Program, Statement, Type};
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
        if self.symbols.contains_key(&symbol.name) {
            return Err(SemanticError::new(
                format!("Symbol '{}' is already defined in this scope", symbol.name),
                symbol.line,
                symbol.column,
            ));
        }
        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
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

        Analyzer {
            current_scope: global_scope,
            errors: Vec::new(),
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
            Statement::VariableDeclaration { name, value, .. } => {
                self.analyze_expression(value);

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

                let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
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

                let function_scope = std::mem::replace(&mut self.current_scope, Scope::new());
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

                let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                self.current_scope = Scope::with_parent(outer_scope);

                for stmt in then_block {
                    self.analyze_statement(stmt);
                }

                let then_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                if let Some(parent) = then_scope.parent {
                    self.current_scope = *parent;
                }

                if let Some(else_stmts) = else_block {
                    let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                    self.current_scope = Scope::with_parent(outer_scope);

                    for stmt in else_stmts {
                        self.analyze_statement(stmt);
                    }

                    let else_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                    if let Some(parent) = else_scope.parent {
                        self.current_scope = *parent;
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

                let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                self.current_scope = Scope::with_parent(outer_scope);

                self.analyze_statement(then_stmt);

                let then_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                if let Some(parent) = then_scope.parent {
                    self.current_scope = *parent;
                }

                if let Some(else_stmt) = else_stmt {
                    let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                    self.current_scope = Scope::with_parent(outer_scope);

                    self.analyze_statement(else_stmt);

                    let else_scope = std::mem::replace(&mut self.current_scope, Scope::new());
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

                let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
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

                let loop_scope = std::mem::replace(&mut self.current_scope, Scope::new());
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

                let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
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

                let loop_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                if let Some(parent) = loop_scope.parent {
                    self.current_scope = *parent;
                }
            }
            Statement::WhileLoop {
                condition, body, ..
            } => {
                self.analyze_expression(condition);

                let outer_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                self.current_scope = Scope::with_parent(outer_scope);

                for stmt in body {
                    self.analyze_statement(stmt);
                }

                let loop_scope = std::mem::replace(&mut self.current_scope, Scope::new());
                if let Some(parent) = loop_scope.parent {
                    self.current_scope = *parent;
                }
            }
            Statement::DisplayStatement { value, .. } => {
                self.analyze_expression(value);
            }
            Statement::ExpressionStatement { expression, .. } => {
                self.analyze_expression(expression);
            }
            Statement::ReturnStatement {
                value: Some(expr), ..
            } => {
                self.analyze_expression(expr);
            }
            Statement::ReturnStatement { value: None, .. } => {}
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
            Expression::Literal(_, _, _) => {}
        }
    }
}

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
