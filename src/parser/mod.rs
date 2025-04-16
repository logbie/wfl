use thiserror::Error;

use std::collections::HashMap;
use crate::lexer::{Token, TokenType};

mod ast;
pub use ast::*;

// Collection type enum for determining parsing strategy
#[derive(Debug, Clone, Copy, PartialEq)]
enum CollectionType {
    List,
    Map,
    // We can add more collection types in the future (Set, Record, etc.)
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Expected {expected}, found {found:?}")]
    Expected { expected: String, found: Option<Token> },
    #[error("Custom error: {0}")]
    Custom(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        self.parse_program()
    }

    // Parse a complete program
    fn parse_program(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    errors.push(err);
                    self.synchronize(); // Skip to the next statement
                }
            }
        }

        if errors.is_empty() {
            Ok(Program { statements })
        } else {
            Err(errors)
        }
    }

    // Skip tokens until we find a statement boundary
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Newline || 
               self.previous().token_type == TokenType::Comment {
                return;
            }

            match self.peek().token_type {
                TokenType::Define | TokenType::Create | TokenType::Store | 
                TokenType::Check | TokenType::For | TokenType::Repeat => return,
                _ => {}
            }

            self.advance();
        }
    }

    // Helper methods for token navigation
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1] // Return EOF token
        } else {
            &self.tokens[self.current]
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        if self.current == 0 {
            &self.tokens[0]
        } else {
            &self.tokens[self.current - 1]
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::Expected {
                expected: message.to_string(),
                found: Some(self.peek().clone()),
            })
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    // Parsing methods for different constructs
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // Skip any newlines, comments, indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Comment, TokenType::Indent, TokenType::Dedent]) {}
        
        // Check for EOF
        if self.is_at_end() {
            return Err(ParseError::UnexpectedEOF);
        }
        
        match self.peek().token_type {
            TokenType::Define => self.parse_action_definition(),
            TokenType::Create => self.parse_creation(),
            TokenType::Store => self.parse_variable_declaration(),
            TokenType::Check => self.parse_check(),
            TokenType::For => self.parse_for_loop(),
            TokenType::Repeat => self.parse_repeat_loop(),
            TokenType::Try => self.parse_try_catch(),
            TokenType::Give => self.parse_return_statement(),
            TokenType::Perform => self.parse_perform_statement(),
            TokenType::Set => self.parse_set_statement(),
            TokenType::Identifier => {
                let token = self.peek().clone();
                self.advance();
                self.parse_expression_statement(token)
            }
            // Skip comment tokens
            TokenType::Comment => {
                self.advance();
                self.parse_statement()
            },
            _ => Err(ParseError::UnexpectedToken(self.peek().clone())),
        }
    }
    
    // Parse a set statement (collection initialization)
    fn parse_set_statement(&mut self) -> Result<Statement, ParseError> {
        // Parse collection initialization expression
        let expr = self.parse_collection_initialization()?;
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::ExpressionStatement(expr))
    }
    
    // Parse a variable declaration statement: "store name as value"
    fn parse_variable_declaration(&mut self) -> Result<Statement, ParseError> {
        // Consume 'store' keyword
        self.consume(TokenType::Store, "Expected 'store' keyword")?;
        
        // Parse the variable name - first part is an identifier
        let name_token = self.consume(TokenType::Identifier, "Expected variable name")?;
        let mut name = name_token.value.clone();
        
        // Handle multi-word identifiers (like "current language")
        // Keep adding identifiers as long as they aren't keywords
        while self.check(TokenType::Identifier) {
            // Check if the next token is a known type token or keyword
            if self.peek().token_type == TokenType::As || 
               self.peek().token_type == TokenType::In || 
               self.peek().token_type == TokenType::Number || 
               self.peek().token_type == TokenType::Text || 
               self.peek().token_type == TokenType::Truth || 
               self.peek().token_type == TokenType::List || 
               self.peek().token_type == TokenType::Map || 
               self.peek().token_type == TokenType::Record {
                break;
            }
            
            // Add the next word to the name with a space
            name.push(' ');
            name.push_str(&self.advance().value);
        }
        
        // Check for "store X in Y at Z" syntax (storing in collections)
        if self.match_token(&[TokenType::In]) {
            // This is a collection storage operation
            let collection_name = self.consume(TokenType::Identifier, "Expected collection name after 'in'")?
                .value.clone();
                
            // Check for "at" keyword
            self.consume(TokenType::At, "Expected 'at' after collection name")?;
            
            // Parse the index/key expression
            let index_expr = self.parse_expression()?;
            
            // Parse the value expression
            let value_expr = self.parse_expression()?;
            
            // Create assignment expression with index access
            let index_access = Expression::Index {
                collection: Box::new(Expression::Variable(collection_name)),
                index: Box::new(index_expr),
            };
            
            let assignment = Expression::Binary {
                left: Box::new(index_access),
                operator: BinaryOperator::Assign,
                right: Box::new(value_expr),
            };
            
            // Consume optional newline
            self.match_token(&[TokenType::Newline]);
            
            return Ok(Statement::ExpressionStatement(assignment));
        }
        
        // Normal variable declaration: "store X as Y"
        // Consume 'as' keyword
        self.consume(TokenType::As, "Expected 'as' keyword after variable name")?;
        
        // Special case for "store X as set to:" - direct collection initialization
        if self.check(TokenType::Set) {
            self.advance(); // Consume 'set'
            
            // Check for 'to' and ':' keywords
            if self.match_token(&[TokenType::To]) {
                self.consume(TokenType::Colon, "Expected ':' after 'to'")?;
                
                // Consume any newlines or indentation
                while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                
                // Parse collection items
                let mut items = Vec::new();
                
                while !self.is_at_end() && !self.check(TokenType::End) {
                    // Consume any newlines or indentation
                    while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                    
                    // Check if we've reached the end
                    if self.check(TokenType::End) {
                        break;
                    }
                    
                    // Parse collection item
                    let item = self.parse_expression()?;
                    items.push(item);
                    
                    // Consume any newlines or indentation
                    while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                }
                
                // Consume 'end' token
                self.consume(TokenType::End, "Expected 'end' to close collection declaration")?;
                
                // Consume 'set' token
                self.match_token(&[TokenType::Set]);
                
                // Create a list expression and use it as initializer
                let initializer = Expression::ListExpression(items);
                
                // Consume optional newline
                self.match_token(&[TokenType::Newline]);
                
                return Ok(Statement::VariableDeclaration {
                    name,
                    value_type: Some("list".to_string()),
                    initializer: Some(initializer),
                });
            }
        }
        
        // Normal variable declaration
        // Parse the variable type (optional)
        let value_type = if self.match_token(&[TokenType::Number, TokenType::Text, TokenType::Truth, TokenType::List, TokenType::Map, TokenType::Record]) {
            Some(self.previous().value.clone())
        } else {
            None
        };
        
        // Parse the initializer expression (optional)
        let initializer = if !self.is_at_end() && self.peek().token_type != TokenType::Newline {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::VariableDeclaration {
            name,
            value_type,
            initializer,
        })
    }

    // Parse a return statement: "give back expression"
    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        // Consume 'give' keyword
        self.consume(TokenType::Give, "Expected 'give' keyword")?;
        
        // Consume 'back' keyword
        self.consume(TokenType::Back, "Expected 'back' keyword after 'give'")?;
        
        // Parse the return expression (optional)
        let value = if !self.is_at_end() && self.peek().token_type != TokenType::Newline {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::ReturnStatement(value))
    }

    // Parse an expression and convert it to an expression statement
    fn parse_expression_statement(&mut self, _: Token) -> Result<Statement, ParseError> {
        // Put the token back
        self.current -= 1;
        
        // Parse the expression
        let expression = self.parse_expression()?;
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::ExpressionStatement(expression))
    }

    // Parse an expression
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_equality()
    }

    // Parse equality expressions (==, !=)
    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        
        while self.match_token(&[TokenType::Equals, TokenType::NotEquals]) {
            let operator = match self.previous().token_type {
                TokenType::Equals => BinaryOperator::Equal,
                TokenType::NotEquals => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    // Parse comparison expressions (>, <, >=, <=)
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;
        
        while self.match_token(&[TokenType::Greater, TokenType::Less, TokenType::GreaterEquals, TokenType::LessEquals]) {
            let operator = match self.previous().token_type {
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::Less => BinaryOperator::Less,
                TokenType::GreaterEquals => BinaryOperator::GreaterEqual,
                TokenType::LessEquals => BinaryOperator::LessEqual,
                _ => unreachable!(),
            };
            
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    // Parse terms (+ and -)
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_factor()?;
        
        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    // Parse factors (* and /)
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_unary()?;
        
        while self.match_token(&[TokenType::Multiply, TokenType::Divide, TokenType::Modulo]) {
            let operator = match self.previous().token_type {
                TokenType::Multiply => BinaryOperator::Multiply,
                TokenType::Divide => BinaryOperator::Divide,
                TokenType::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    // Parse unary expressions (! and -)
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&[TokenType::Not, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Not => UnaryOperator::Not,
                TokenType::Minus => UnaryOperator::Negate,
                _ => unreachable!(),
            };
            
            let right = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }
        
        self.parse_primary()
    }

    // Parse primary expressions (literals, identifiers, etc.)
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        if self.is_at_end() {
            return Err(ParseError::UnexpectedEOF);
        }

        // Parse the initial expression
        let mut expr = self.parse_atom()?;
        
        // Handle chained access operations (collection access, method calls, etc.)
        loop {
            if self.check(TokenType::At) || self.check(TokenType::LeftBracket) {
                // Collection access
                expr = self.parse_collection_access(expr)?;
            } else if self.check(TokenType::With) || self.check(TokenType::LeftParen) {
                // Function call
                if let Expression::Variable(name) = expr {
                    expr = self.parse_function_call(name)?;
                } else {
                    return Err(ParseError::Custom("Function call target must be a variable".to_string()));
                }
            } else if self.check(TokenType::Dot) {
                // Member access (dot notation)
                self.advance(); // Consume the dot
                
                // Member name should be an identifier
                if self.check(TokenType::Identifier) {
                    let member_name = self.advance().value.clone();
                    expr = Expression::MemberAccess {
                        object: Box::new(expr),
                        name: member_name,
                    };
                } else {
                    return Err(ParseError::Expected {
                        expected: "member name (identifier)".to_string(),
                        found: Some(self.peek().clone()),
                    });
                }
            } else {
                // No more chained operations
                break;
            }
        }
        
        Ok(expr)
    }
    
    // Parse a single atom (the smallest unit of an expression)
    fn parse_atom(&mut self) -> Result<Expression, ParseError> {
        if self.is_at_end() {
            return Err(ParseError::UnexpectedEOF);
        }

        // Match literals
        if self.match_token(&[TokenType::StringLiteral]) {
            return Ok(Expression::StringLiteral(self.previous().value.clone()));
        }
        
        if self.match_token(&[TokenType::NumberLiteral]) {
            let num_str = self.previous().value.clone();
            match num_str.parse::<f64>() {
                Ok(value) => return Ok(Expression::NumberLiteral(value)),
                Err(_) => return Err(ParseError::Custom(format!("Invalid number literal: {}", num_str))),
            }
        }
        
        if self.match_token(&[TokenType::TruthLiteral]) {
            let value = self.previous().value.clone();
            let is_true = value == "yes" || value == "true";
            return Ok(Expression::BooleanLiteral(is_true));
        }
        
        if self.match_token(&[TokenType::Nothing, TokenType::Missing, TokenType::Undefined, TokenType::Empty]) {
            return Ok(Expression::NothingLiteral);
        }
        
        // Match 'perform' keyword for function calls
        if self.match_token(&[TokenType::Perform]) {
            // This is a function call with the 'perform' keyword
            
            // Parse the function name, which can be an identifier or a string
            if self.check(TokenType::Identifier) || self.check(TokenType::StringLiteral) {
                let name = self.peek().value.clone();
                self.advance();
                
                return self.parse_function_call(name);
            } else {
                return Err(ParseError::Expected { 
                    expected: "function name (identifier or string)".to_string(), 
                    found: Some(self.peek().clone())
                });
            }
        }
        
        // Match list literals using square bracket syntax
        if self.check(TokenType::LeftBracket) {
            return self.parse_list_literal();
        }
        
        // Match collection initialization using 'set' keyword
        if self.check(TokenType::Set) {
            return self.parse_collection_initialization();
        }
        
        // Match identifiers (variables)
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expression::Variable(self.previous().value.clone()));
        }
        
        // Handle parenthesized expressions
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(expr);
        }
        
        Err(ParseError::UnexpectedToken(self.peek().clone()))
    }

    // Parse a function call
    fn parse_function_call(&mut self, callee_name: String) -> Result<Expression, ParseError> {
        let mut arguments = Vec::new();
        
        // Check if this is a block-style function call with "with:" syntax
        if self.check(TokenType::With) {
            self.advance(); // Consume 'with'
            
            // Check for colon after 'with'
            if self.match_token(&[TokenType::Colon]) {
                // Block-style function call with named parameters
                
                // Consume all indentation tokens
                while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                
                // Parse named parameters until we hit 'end'
                while !self.is_at_end() && !self.check(TokenType::End) {
                    // Consume all indentation tokens
                    while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                    
                    // Break if we've reached the end token
                    if self.check(TokenType::End) {
                        break;
                    }
                    
                    // Parse the parameter name
                    let name = if self.check(TokenType::Identifier) {
                        let name = self.peek().value.clone();
                        self.advance();
                        
                        // Expect 'as' after parameter name
                        self.consume(TokenType::As, "Expected 'as' after parameter name")?;
                        
                        Some(name)
                    } else {
                        return Err(ParseError::Expected {
                            expected: "parameter name".to_string(),
                            found: Some(self.peek().clone()),
                        });
                    };
                    
                    // Parse the parameter value
                    let value = self.parse_expression()?;
                    
                    // Add the argument
                    arguments.push(NamedArgument { name, value });
                    
                    // Consume all indentation tokens
                    while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                }
                
                // Consume 'end' token
                self.consume(TokenType::End, "Expected 'end' to close function call arguments")?;
                
                // Consume 'with' token
                self.match_token(&[TokenType::With]);
                
                // Consume all indentation tokens
                while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                
            } else {
                // Inline-style function call with simple arguments
                // Similar to the existing implementation but already consumed 'with'
                
                while !self.is_at_end() && 
                      self.peek().token_type != TokenType::Newline && 
                      self.peek().token_type != TokenType::End {
                    
                    // Parse the argument name if present
                    let name = if self.check(TokenType::Identifier) {
                        let name = self.peek().value.clone();
                        self.advance();
                        
                        // Check for 'as' keyword
                        if self.match_token(&[TokenType::As]) {
                            Some(name)
                        } else {
                            self.current -= 1; // Put the token back
                            None
                        }
                    } else {
                        None
                    };
                    
                    // Parse the argument value
                    let value = self.parse_expression()?;
                    
                    // Add the argument
                    arguments.push(NamedArgument { name, value });
                    
                    // Check for comma
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
        } else {
            // Direct function call without 'with' keyword
            // For example: perform function_name(arg1, arg2)
            
            // Check for opening parenthesis
            if self.match_token(&[TokenType::LeftParen]) {
                // Parse arguments until we hit closing parenthesis
                if !self.check(TokenType::RightParen) {
                    loop {
                        // Parse the argument value (positional arguments without names)
                        let value = self.parse_expression()?;
                        
                        // Add the argument (no name for positional arguments)
                        arguments.push(NamedArgument { name: None, value });
                        
                        // Check for comma
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                
                // Consume closing parenthesis
                self.consume(TokenType::RightParen, "Expected ')' after function arguments")?;
            }
            // If no parenthesis, it's a function call without arguments
        }
        
        Ok(Expression::Call {
            callee: Box::new(Expression::Variable(callee_name)),
            arguments,
        })
    }

    // Parse an action (function) definition
    fn parse_action_definition(&mut self) -> Result<Statement, ParseError> {
        // Consume 'define' keyword
        self.consume(TokenType::Define, "Expected 'define' keyword")?;
        
        // Check for private keyword (optional)
        let is_private = self.match_token(&[TokenType::Private]);
        
        // Consume 'action' keyword
        self.consume(TokenType::Action, "Expected 'action' keyword")?;
        
        // Consume 'called' keyword
        self.consume(TokenType::Called, "Expected 'called' keyword")?;
        
        // Parse the action name
        let name = if self.match_token(&[TokenType::StringLiteral]) {
            self.previous().value.clone()
        } else {
            return Err(ParseError::Expected { 
                expected: "action name as string literal".to_string(), 
                found: Some(self.peek().clone()) 
            });
        };
        
        // Consume the colon
        self.consume(TokenType::Colon, "Expected ':' after action name")?;
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        // Parse parameters (needs: section)
        let parameters = if self.check(TokenType::Needs) {
            self.advance(); // Consume 'needs'
            
            // Consume the colon
            self.consume(TokenType::Colon, "Expected ':' after 'needs'")?;
            
            // Consume all indentation tokens
            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
            
            // Parse parameters
            self.parse_function_parameters()?
        } else {
            Vec::new()
        };
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        // Parse return type (gives back: section)
        let return_type = if self.check(TokenType::Give) {
            self.advance(); // Consume 'give'
            self.consume(TokenType::Back, "Expected 'back' after 'give'")?;
            
            // Consume the colon
            self.consume(TokenType::Colon, "Expected ':' after 'gives back'")?;
            
            // Consume all indentation tokens
            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
            
            // Parse return type
            Some(self.parse_function_return_type()?)
        } else {
            None
        };
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        // Parse function body (does: section)
        self.consume(TokenType::Does, "Expected 'does' section in action definition")?;
        
        // Consume the colon
        self.consume(TokenType::Colon, "Expected ':' after 'does'")?;
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        // Parse the function body
        let mut body = Vec::new();
        
        while !self.is_at_end() && !self.check(TokenType::End) {
            let statement = self.parse_statement()?;
            body.push(statement);
            
            // Consume all indentation tokens
            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        }
        
        // Consume 'end' keyword
        self.consume(TokenType::End, "Expected 'end' to close action definition")?;
        
        // Consume 'action' keyword
        self.match_token(&[TokenType::Action]);
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        Ok(Statement::ActionDefinition {
            name,
            parameters,
            return_type,
            body,
            is_async: false, // For now, we don't support async actions
            is_private,
        })
    }
    
    // Parse the parameters section of a function definition
    fn parse_function_parameters(&mut self) -> Result<Vec<Parameter>, ParseError> {
        let mut parameters = Vec::new();
        
        while !self.is_at_end() && 
              !self.check(TokenType::Give) && 
              !self.check(TokenType::Does) && 
              !self.check(TokenType::End) && 
              !self.check(TokenType::Dedent) {
            
            // Skip any indent/dedent tokens
            while self.match_token(&[TokenType::Indent, TokenType::Dedent, TokenType::Newline]) {}
            
            // Check if we've reached the end of parameters
            if self.check(TokenType::Give) || self.check(TokenType::Does) || self.check(TokenType::End) {
                break;
            }
            
            // Parse parameter name
            let name = self.consume(TokenType::Identifier, "Expected parameter name")?
                .value.clone();
            
            // Consume 'as' keyword
            self.consume(TokenType::As, "Expected 'as' after parameter name")?;
            
            // Parse parameter type
            let param_type = if self.check(TokenType::Number) || 
                               self.check(TokenType::Text) || 
                               self.check(TokenType::Truth) || 
                               self.check(TokenType::List) || 
                               self.check(TokenType::Map) || 
                               self.check(TokenType::Record) {
                self.advance().value.clone()
            } else {
                return Err(ParseError::Expected { 
                    expected: "parameter type".to_string(), 
                    found: Some(self.peek().clone()) 
                });
            };
            
            // Parse optional default value
            let default_value = if self.match_token(&[TokenType::With]) {
                self.consume(TokenType::Default, "Expected 'default' after 'with'")?;
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            parameters.push(Parameter {
                name,
                param_type,
                default_value,
            });
            
            // Consume optional newline
            self.match_token(&[TokenType::Newline]);
        }
        
        Ok(parameters)
    }
    
    // Parse the return type section of a function definition
    fn parse_function_return_type(&mut self) -> Result<String, ParseError> {
        // Skip any indent/dedent tokens
        while self.match_token(&[TokenType::Indent, TokenType::Dedent, TokenType::Newline]) {}
        
        // Parse return value name (identifier)
        let _name = self.consume(TokenType::Identifier, "Expected return value name")?
            .value.clone();
        
        // Consume 'as' keyword
        self.consume(TokenType::As, "Expected 'as' after return value name")?;
        
        // Parse return type
        if self.check(TokenType::Number) || 
           self.check(TokenType::Text) || 
           self.check(TokenType::Truth) || 
           self.check(TokenType::List) || 
           self.check(TokenType::Map) || 
           self.check(TokenType::Record) {
            let return_type = self.advance().value.clone();
            
            // Consume optional newline
            self.match_token(&[TokenType::Newline]);
            
            Ok(return_type)
        } else {
            Err(ParseError::Expected { 
                expected: "return type".to_string(), 
                found: Some(self.peek().clone()) 
            })
        }
    }

    fn parse_creation(&mut self) -> Result<Statement, ParseError> {
        // Consume 'create' keyword
        self.consume(TokenType::Create, "Expected 'create' keyword")?;
        
        // Check if this is a container creation
        if self.match_token(&[TokenType::Container]) {
            return self.parse_container_creation();
        }
        
        // Otherwise, it's an object creation/instantiation
        return self.parse_object_creation();
    }
    
    // Parse container (class) creation
    fn parse_container_creation(&mut self) -> Result<Statement, ParseError> {
        // Consume 'called' keyword
        self.consume(TokenType::Called, "Expected 'called' keyword after 'container'")?;
        
        // Parse container name
        let name = if self.match_token(&[TokenType::StringLiteral]) {
            self.previous().value.clone()
        } else {
            return Err(ParseError::Expected { 
                expected: "container name as string literal".to_string(), 
                found: Some(self.peek().clone()) 
            });
        };
        
        // Consume the colon
        self.consume(TokenType::Colon, "Expected ':' after container name")?;
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        // Parse container body
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut constructor = None;
        
        // Current visibility mode for parsing sections
        let mut current_visibility_is_private = false;
        
        while !self.is_at_end() && !self.check(TokenType::End) {
            // Consume all indentation tokens
            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
            
            // Check if we've reached the end
            if self.check(TokenType::End) {
                break;
            }
            
            // Check for visibility modifiers
            if self.match_token(&[TokenType::Private, TokenType::Public]) {
                current_visibility_is_private = self.previous().token_type == TokenType::Private;
                
                // Consume the colon
                self.consume(TokenType::Colon, "Expected ':' after visibility modifier")?;
                
                // Consume all indentation tokens
                while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                
                // Parse fields and methods for this visibility section
                while !self.is_at_end() && 
                      !self.check(TokenType::Private) && 
                      !self.check(TokenType::Public) && 
                      !self.check(TokenType::End) && 
                      !self.check(TokenType::When) {
                    
                    // Consume all indentation tokens
                    while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                    
                    // Check if we've reached the end of this section
                    if self.check(TokenType::Private) || 
                       self.check(TokenType::Public) || 
                       self.check(TokenType::End) || 
                       self.check(TokenType::When) {
                        break;
                    }
                    
                    if self.check(TokenType::Store) {
                        // Parse a field declaration
                        let field = self.parse_field_declaration(current_visibility_is_private)?;
                        fields.push(field);
                    } else if self.check(TokenType::Define) {
                        // Parse a method definition
                        let method = self.parse_action_definition()?;
                        
                        // If method is an ActionDefinition statement, extract its fields
                        if let Statement::ActionDefinition { name, parameters, return_type, body, is_async, is_private } = method {
                            // Update privacy based on current section
                            let updated_method = Statement::ActionDefinition {
                                name,
                                parameters,
                                return_type,
                                body,
                                is_async,
                                is_private: current_visibility_is_private, // Override with section privacy
                            };
                            methods.push(updated_method);
                        } else {
                            // This should never happen if parse_action_definition returns the right type
                            return Err(ParseError::Custom("Expected action definition".to_string()));
                        }
                    } else {
                        return Err(ParseError::UnexpectedToken(self.peek().clone()));
                    }
                    
                    // Consume all indentation tokens
                    while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                }
            } else if self.match_token(&[TokenType::When]) {
                // Parse constructor
                if self.match_token(&[TokenType::Created]) {
                    // Consume the colon
                    self.consume(TokenType::Colon, "Expected ':' after 'when created'")?;
                    
                    // Consume all indentation tokens
                    while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                    
                    // Check if there's a 'does' section
                    if self.match_token(&[TokenType::Does]) {
                        // Consume the colon
                        self.consume(TokenType::Colon, "Expected ':' after 'does'")?;
                        
                        // Consume all indentation tokens
                        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                        
                        // Parse constructor body
                        let mut body = Vec::new();
                        
                        while !self.is_at_end() && !self.check(TokenType::End) {
                            let statement = self.parse_statement()?;
                            body.push(statement);
                            
                            // Consume all indentation tokens
                            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                        }
                        
                        constructor = Some(body);
                        
                        // Consume 'end' keyword
                        self.consume(TokenType::End, "Expected 'end' to close constructor")?;
                        
                        // Consume 'when' keyword
                        self.match_token(&[TokenType::When]);
                        
                        // Consume all indentation tokens
                        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                    }
                } else {
                    return Err(ParseError::Expected { 
                        expected: "'created' after 'when'".to_string(), 
                        found: Some(self.peek().clone()) 
                    });
                }
            } else {
                return Err(ParseError::UnexpectedToken(self.peek().clone()));
            }
            
            // Consume all indentation tokens
            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        }
        
        // Consume 'end' keyword
        self.consume(TokenType::End, "Expected 'end' to close container definition")?;
        
        // Consume 'container' keyword
        self.match_token(&[TokenType::Container]);
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        Ok(Statement::ContainerDefinition {
            name,
            fields,
            methods,
            constructor,
        })
    }
    
    // Parse a field declaration within a container
    fn parse_field_declaration(&mut self, is_private: bool) -> Result<VariableField, ParseError> {
        // Consume 'store' keyword
        self.consume(TokenType::Store, "Expected 'store' keyword")?;
        
        // Parse field name - first part is an identifier
        let mut name = self.consume(TokenType::Identifier, "Expected field name")?
            .value.clone();
        
        // Handle multi-word identifiers (like "current language")
        // Keep adding identifiers as long as they aren't keywords
        while self.check(TokenType::Identifier) {
            // Check if the next token is a known type token or keyword
            if self.peek().token_type == TokenType::As || 
               self.peek().token_type == TokenType::In || 
               self.peek().token_type == TokenType::Number || 
               self.peek().token_type == TokenType::Text || 
               self.peek().token_type == TokenType::Truth || 
               self.peek().token_type == TokenType::List || 
               self.peek().token_type == TokenType::Map || 
               self.peek().token_type == TokenType::Record {
                break;
            }
            
            // Add the next word to the name with a space
            name.push(' ');
            name.push_str(&self.advance().value);
        }
        
        // Consume 'as' keyword
        self.consume(TokenType::As, "Expected 'as' after field name")?;
        
        // Parse field type
        let field_type = if self.check(TokenType::Number) || 
                           self.check(TokenType::Text) || 
                           self.check(TokenType::Truth) || 
                           self.check(TokenType::List) || 
                           self.check(TokenType::Map) || 
                           self.check(TokenType::Record) {
            self.advance().value.clone()
        } else {
            return Err(ParseError::Expected { 
                expected: "field type".to_string(), 
                found: Some(self.peek().clone()) 
            });
        };
        
        // Parse optional initializer
        let initializer = if !self.is_at_end() && 
                            self.peek().token_type != TokenType::Newline &&
                            self.peek().token_type != TokenType::Dedent {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Consume all indentation tokens
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        Ok(VariableField {
            name,
            field_type,
            is_private,
            initializer,
        })
    }
    
    // Parse object creation (instantiation)
    fn parse_object_creation(&mut self) -> Result<Statement, ParseError> {
        // Get variable name where we'll store the object
        let var_name = self.consume(TokenType::Identifier, "Expected variable name")?
            .value.clone();
            
        // Consume 'as' keyword
        self.consume(TokenType::As, "Expected 'as' after variable name")?;
        
        // Consume 'new' keyword
        self.consume(TokenType::New, "Expected 'new' keyword")?;
        
        // Parse container type (string literal)
        let container_name = if self.match_token(&[TokenType::StringLiteral]) {
            self.previous().value.clone()
        } else {
            return Err(ParseError::Expected { 
                expected: "container name as string literal".to_string(), 
                found: Some(self.peek().clone()) 
            });
        };
        
        // Create a call to the constructor
        let constructor_call = Expression::Call {
            callee: Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Variable(container_name.clone())),
                name: "constructor".to_string(),
            }),
            arguments: Vec::new(),  // For now, no constructor arguments
        };
        
        // Create a variable declaration with the constructor call as initializer
        Ok(Statement::VariableDeclaration {
            name: var_name,
            value_type: Some(container_name),  // Container name is the type
            initializer: Some(constructor_call),
        })
    }

    fn parse_check(&mut self) -> Result<Statement, ParseError> {
        // Consume 'check' keyword
        self.consume(TokenType::Check, "Expected 'check' keyword")?;
        
        // Parse the condition
        let condition = self.parse_expression()?;
        
        // Consume the colon
        self.consume(TokenType::Colon, "Expected ':' after check condition")?;
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        // Parse the then branch
        let mut then_statements = Vec::new();
        
        while !self.is_at_end() && 
              !self.check(TokenType::Otherwise) && 
              !self.check(TokenType::End) {
            
            // Parse a statement
            let statement = self.parse_statement()?;
            then_statements.push(statement);
        }
        
        // Parse the optional else branch
        let else_statements = if self.match_token(&[TokenType::Otherwise]) {
            // Consume the colon
            self.consume(TokenType::Colon, "Expected ':' after 'otherwise'")?;
            
            // Consume optional newline
            self.match_token(&[TokenType::Newline]);
            
            let mut else_statements = Vec::new();
            
            while !self.is_at_end() && !self.check(TokenType::End) {
                // Parse a statement
                let statement = self.parse_statement()?;
                else_statements.push(statement);
            }
            
            Some(else_statements)
        } else {
            None
        };
        
        // Consume 'end' keyword
        self.consume(TokenType::End, "Expected 'end' after check block")?;
        
        // Check if the next token is 'check', but don't consume it
        if self.check(TokenType::Check) {
            self.advance();
        }
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::CheckStatement {
            condition,
            then_branch: then_statements,
            else_branch: else_statements,
        })
    }

    fn parse_for_loop(&mut self) -> Result<Statement, ParseError> {
        // Consume 'for' keyword
        self.consume(TokenType::For, "Expected 'for' keyword")?;
        
        // Consume 'each' keyword
        self.consume(TokenType::Each, "Expected 'each' after 'for'")?;
        
        // Parse item variable name
        let item_name = self.consume(TokenType::Identifier, "Expected item variable name")?.value.clone();
        
        // Parse index name (optional)
        let index_name = if self.match_token(&[TokenType::And]) {
            let name = self.consume(TokenType::Identifier, "Expected index name after 'and'")?.value.clone();
            Some(name)
        } else {
            None
        };
        
        // Consume 'in' keyword
        self.consume(TokenType::In, "Expected 'in' after item name")?;
        
        // Parse collection expression
        let collection = self.parse_expression()?;
        
        // Consume colon
        self.consume(TokenType::Colon, "Expected ':' after collection expression")?;
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        // Parse loop body
        let mut body = Vec::new();
        
        while !self.is_at_end() && !self.check(TokenType::End) {
            let statement = self.parse_statement()?;
            body.push(statement);
        }
        
        // Consume 'end' keyword
        self.consume(TokenType::End, "Expected 'end' to close for-each loop")?;
        
        // Consume 'for' or 'for each' keywords
        if self.match_token(&[TokenType::For]) {
            self.match_token(&[TokenType::Each]);
        }
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::ForEachLoop {
            item_name,
            index_name,
            collection,
            body,
        })
    }

    fn parse_repeat_loop(&mut self) -> Result<Statement, ParseError> {
        // Consume 'repeat' keyword
        self.consume(TokenType::Repeat, "Expected 'repeat' keyword")?;
        
        // Check if this is a 'while' or 'until' loop
        let is_while = if self.match_token(&[TokenType::While]) {
            true
        } else if self.match_token(&[TokenType::Until]) {
            false
        } else {
            return Err(ParseError::Expected {
                expected: "'while' or 'until' after 'repeat'".to_string(),
                found: Some(self.peek().clone()),
            });
        };
        
        // Parse condition
        let condition = self.parse_expression()?;
        
        // Consume colon
        self.consume(TokenType::Colon, "Expected ':' after loop condition")?;
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        // Parse loop body
        let mut body = Vec::new();
        
        while !self.is_at_end() && !self.check(TokenType::End) {
            let statement = self.parse_statement()?;
            body.push(statement);
        }
        
        // Consume 'end' keyword
        self.consume(TokenType::End, "Expected 'end' to close repeat loop")?;
        
        // Consume 'repeat' keyword
        self.match_token(&[TokenType::Repeat]);
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::RepeatLoop {
            is_while,
            condition,
            body,
        })
    }

    fn parse_try_catch(&mut self) -> Result<Statement, ParseError> {
        // Not fully implemented yet
        Err(ParseError::Custom("Try-catch parsing not implemented yet".to_string()))
    }

    // Parse a perform statement: "perform function_name with arguments"
    fn parse_perform_statement(&mut self) -> Result<Statement, ParseError> {
        // Consume 'perform' keyword
        self.consume(TokenType::Perform, "Expected 'perform' keyword")?;
        
        // Parse the function call expression
        let expr = if self.check(TokenType::Identifier) || self.check(TokenType::StringLiteral) {
            let name = self.peek().value.clone();
            self.advance();
            
            self.parse_function_call(name)?
        } else {
            return Err(ParseError::Expected { 
                expected: "function name (identifier or string)".to_string(), 
                found: Some(self.peek().clone())
            });
        };
        
        // Consume optional newline
        self.match_token(&[TokenType::Newline]);
        
        Ok(Statement::ExpressionStatement(expr))
    }

    // Parse collection initialization: "set x to: ... end set"
    fn parse_collection_initialization(&mut self) -> Result<Expression, ParseError> {
        // Consume 'set' keyword
        self.consume(TokenType::Set, "Expected 'set' keyword")?;
        
        // Parse target variable (optional)
        let target_var = if self.check(TokenType::Identifier) {
            let name = self.peek().value.clone();
            self.advance();
            
            // Consume 'to' keyword
            self.consume(TokenType::To, "Expected 'to' keyword after variable name")?;
            
            Some(name)
        } else {
            None
        };
        
        // Consume ':' after to
        self.consume(TokenType::Colon, "Expected ':' after 'to'")?;
        
        // Consume any newlines or indentation
        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        
        // Initialize collections
        let mut items = Vec::new();
        let mut key_value_pairs = HashMap::new();
        let mut collection_type = CollectionType::List; // Default to list
        
        while !self.is_at_end() && !self.check(TokenType::End) {
            // Consume any newlines or indentation
            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
            
            // Check if we've reached the end
            if self.check(TokenType::End) {
                break;
            }
            
            // Try to parse collection entry
            // Check if this is a key-value pair (map entry) or just a value (list entry)
            if self.check(TokenType::StringLiteral) || self.check(TokenType::Identifier) || 
               self.check(TokenType::NumberLiteral) {
                
                // Parse the key or value
                let first_expr = self.parse_expression()?;
                
                // Check if this is a key-value pair
                if self.match_token(&[TokenType::Is]) {
                    // This is a map entry ("key is value")
                    collection_type = CollectionType::Map;
                    
                    // Check for nested collection with "set to:" syntax
                    let value = if self.check(TokenType::Set) {
                        // Handle nested collection with the special "set to:" syntax
                        
                        // Consume 'set' keyword
                        self.advance();
                        
                        // Consume 'to' keyword
                        self.consume(TokenType::To, "Expected 'to' keyword after 'set'")?;
                        
                        // Consume ':' after 'to'
                        self.consume(TokenType::Colon, "Expected ':' after 'to'")?;
                        
                        // Consume any newlines or indentation
                        while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                        
                        // Initialize collections for the nested set
                        let mut nested_items = Vec::new();
                        let mut nested_key_value_pairs = HashMap::new();
                        let mut nested_collection_type = CollectionType::List; // Default to list
                        
                        while !self.is_at_end() && !self.check(TokenType::End) {
                            // Consume any newlines or indentation
                            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                            
                            // Check if we've reached the end
                            if self.check(TokenType::End) {
                                break;
                            }
                            
                            // Parse collection entry
                            if self.check(TokenType::StringLiteral) || self.check(TokenType::Identifier) || 
                               self.check(TokenType::NumberLiteral) {
                                
                                // Parse the key or value
                                let nested_expr = self.parse_expression()?;
                                
                                // Check if this is a key-value pair
                                if self.match_token(&[TokenType::Is]) {
                                    // This is a map entry
                                    nested_collection_type = CollectionType::Map;
                                    
                                    // Parse the value
                                    let nested_value = self.parse_expression()?;
                                    
                                    // For map entries, the key must be convertible to a string
                                    let nested_key = match &nested_expr {
                                        Expression::StringLiteral(s) => s.clone(),
                                        Expression::NumberLiteral(n) => n.to_string(),
                                        Expression::Variable(v) => v.clone(),
                                        _ => return Err(ParseError::Custom("Map keys must be string literals, numbers, or identifiers".to_string())),
                                    };
                                    
                                    // Add to the map
                                    nested_key_value_pairs.insert(nested_key, nested_value);
                                } else {
                                    // This is a list entry
                                    nested_collection_type = CollectionType::List;
                                    nested_items.push(nested_expr);
                                }
                            } else {
                                return Err(ParseError::UnexpectedToken(self.peek().clone()));
                            }
                            
                            // Consume any newlines or indentation
                            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
                        }
                        
                        // Consume 'end' token
                        self.consume(TokenType::End, "Expected 'end' to close nested collection declaration")?;
                        
                        // Consume 'set' token after end
                        self.match_token(&[TokenType::Set]);
                        
                        // Create appropriate collection expression
                        match nested_collection_type {
                            CollectionType::List => Expression::ListExpression(nested_items),
                            CollectionType::Map => Expression::MapExpression(nested_key_value_pairs),
                        }
                    } else {
                        // Normal value
                        self.parse_expression()?
                    };
                    
                    // For map entries, the key must be convertible to a string
                    let key = match &first_expr {
                        Expression::StringLiteral(s) => s.clone(),
                        Expression::NumberLiteral(n) => n.to_string(),
                        Expression::Variable(v) => v.clone(),
                        _ => return Err(ParseError::Custom("Map keys must be string literals, numbers, or identifiers".to_string())),
                    };
                    
                    // Add to the map
                    key_value_pairs.insert(key, value);
                } else {
                    // This is a list entry (just a value)
                    collection_type = CollectionType::List;
                    items.push(first_expr);
                }
            } else {
                return Err(ParseError::UnexpectedToken(self.peek().clone()));
            }
            
            // Consume any newlines or indentation
            while self.match_token(&[TokenType::Newline, TokenType::Indent, TokenType::Dedent]) {}
        }
        
        // Consume 'end' token
        self.consume(TokenType::End, "Expected 'end' to close collection declaration")?;
        
        // Consume 'set' token
        self.match_token(&[TokenType::Set]);
        
        // Create the appropriate collection expression
        let collection_expr = match collection_type {
            CollectionType::List => Expression::ListExpression(items),
            CollectionType::Map => Expression::MapExpression(key_value_pairs),
            // We can add support for other collection types here in the future
        };
        
        // If there was a target variable, this is an assignment
        if let Some(target) = target_var {
            Ok(Expression::Binary {
                left: Box::new(Expression::Variable(target)),
                operator: BinaryOperator::Assign,
                right: Box::new(collection_expr),
            })
        } else {
            // Just return the collection expression
            Ok(collection_expr)
        }
    }
    
    // Parse collection access
    fn parse_collection_access(&mut self, collection: Expression) -> Result<Expression, ParseError> {
        // There are two syntaxes for collection access:
        // 1. collection at key - For maps and list indices
        // 2. collection[key] - Square bracket notation (more traditional)
        
        if self.match_token(&[TokenType::At]) {
            // 'at' keyword syntax
            // Parse the index/key expression
            let index = self.parse_expression()?;
            
            Ok(Expression::Index {
                collection: Box::new(collection),
                index: Box::new(index),
            })
        } else if self.match_token(&[TokenType::LeftBracket]) {
            // Square bracket syntax
            // Parse the index/key expression
            let index = self.parse_expression()?;
            
            // Consume closing bracket
            self.consume(TokenType::RightBracket, "Expected ']' after index expression")?;
            
            Ok(Expression::Index {
                collection: Box::new(collection),
                index: Box::new(index),
            })
        } else {
            Err(ParseError::Expected {
                expected: "'at' keyword or '[' for collection access".to_string(),
                found: Some(self.peek().clone()),
            })
        }
    }
    
    // Parse a list literal using square bracket syntax: [1, 2, 3]
    fn parse_list_literal(&mut self) -> Result<Expression, ParseError> {
        // Consume opening bracket
        self.consume(TokenType::LeftBracket, "Expected '[' to start list literal")?;
        
        let mut items = Vec::new();
        
        // Check if this is an empty list
        if !self.check(TokenType::RightBracket) {
            // Parse list items
            loop {
                // Parse item expression
                let item = self.parse_expression()?;
                items.push(item);
                
                // Check for comma
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
                
                // Handle trailing comma
                if self.check(TokenType::RightBracket) {
                    break;
                }
            }
        }
        
        // Consume closing bracket
        self.consume(TokenType::RightBracket, "Expected ']' to end list literal")?;
        
        Ok(Expression::ListExpression(items))
    }
} 