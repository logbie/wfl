pub mod ast;
#[cfg(test)]
mod tests;

use crate::lexer::token::{Token, TokenWithPosition};
use ast::*;
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, TokenWithPosition>>,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [TokenWithPosition]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut program = Program::new();

        while let Some(_) = self.tokens.peek() {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(error) => {
                    self.errors.push(error);
                    self.synchronize(); // Skip to next statement on error
                }
            }
        }

        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors.clone())
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.tokens.peek() {
            match &token.token {
                Token::KeywordStore | Token::KeywordCreate | Token::KeywordDisplay |
                Token::KeywordCheck | Token::KeywordCount | Token::KeywordFor |
                Token::KeywordDefine | Token::KeywordIf | Token::KeywordEnd => {
                    break;
                }
                _ => {
                    self.tokens.next(); // Skip the token
                }
            }
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        if let Some(token) = self.tokens.peek() {
            match &token.token {
                Token::KeywordStore | Token::KeywordCreate => {
                    self.parse_variable_declaration()
                }
                Token::KeywordDisplay => {
                    self.parse_display_statement()
                }
                Token::KeywordCheck => {
                    self.parse_if_statement()
                }
                Token::KeywordIf => {
                    self.parse_single_line_if()
                }
                Token::KeywordCount => {
                    self.parse_count_loop()
                }
                Token::KeywordFor => {
                    self.parse_for_each_loop()
                }
                Token::KeywordDefine => {
                    self.parse_action_definition()
                }
                Token::KeywordChange => {
                    self.parse_assignment()
                }
                Token::KeywordBreak => {
                    self.tokens.next();
                    Ok(Statement::BreakStatement)
                }
                Token::KeywordContinue | Token::KeywordSkip => {
                    self.tokens.next();
                    Ok(Statement::ContinueStatement)
                }
                Token::KeywordOpen => {
                    self.parse_open_file_statement()
                }
                Token::KeywordGive | Token::KeywordReturn => {
                    self.parse_return_statement()
                }
                _ => {
                    self.parse_expression_statement()
                }
            }
        } else {
            Err(ParseError::new(
                "Unexpected end of input".to_string(),
                0,
                0,
            ))
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParseError> {
        let token_pos = self.tokens.next().unwrap();
        let _is_store = matches!(token_pos.token, Token::KeywordStore);
        
        let mut name = String::new();
        
        while let Some(token) = self.tokens.peek() {
            if let Token::Identifier(id) = &token.token {
                if !name.is_empty() {
                    name.push(' ');
                }
                name.push_str(id);
                self.tokens.next();
            } else if let Token::KeywordAs = &token.token {
                break;
            } else {
                return Err(ParseError::new(
                    format!("Expected identifier or 'as', found {:?}", token.token),
                    token.line,
                    token.column,
                ));
            }
        }
        
        self.expect_token(Token::KeywordAs, "Expected 'as' after variable name")?;
        
        let value = self.parse_expression()?;
        
        Ok(Statement::VariableDeclaration { name, value })
    }

    fn expect_token(&mut self, expected: Token, error_message: &str) -> Result<(), ParseError> {
        if let Some(token) = self.tokens.peek() {
            if &token.token == &expected {
                self.tokens.next();
                Ok(())
            } else {
                Err(ParseError::new(
                    format!("{}: expected {:?}, found {:?}", error_message, expected, token.token),
                    token.line,
                    token.column,
                ))
            }
        } else {
            Err(ParseError::new(
                format!("{}: unexpected end of input", error_message),
                0,
                0,
            ))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_binary_expression(0) // Start with lowest precedence
    }
    
    fn parse_binary_expression(&mut self, precedence: u8) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary_expression()?;
        
        while let Some(token_pos) = self.tokens.peek().cloned() {
            let token = token_pos.token.clone();
            let line = token_pos.line;
            let column = token_pos.column;
            
            let op = match token {
                Token::KeywordPlus => {
                    self.tokens.next(); // Consume "plus"
                    Some((Operator::Plus, 1))
                },
                Token::KeywordMinus => {
                    self.tokens.next(); // Consume "minus"
                    Some((Operator::Minus, 1))
                },
                Token::KeywordTimes => {
                    self.tokens.next(); // Consume "times"
                    Some((Operator::Multiply, 2))
                },
                Token::KeywordDivided => {
                    self.tokens.next();
                    if let Some(by_token) = self.tokens.peek().cloned() {
                        if matches!(by_token.token, Token::KeywordBy) {
                            self.tokens.next(); // Consume "by"
                            Some((Operator::Divide, 2))
                        } else {
                            return Err(ParseError::new(
                                format!("Expected 'by' after 'divided', found {:?}", by_token.token),
                                by_token.line,
                                by_token.column,
                            ));
                        }
                    } else {
                        return Err(ParseError::new(
                            "Unexpected end of input after 'divided'".to_string(),
                            line,
                            column,
                        ));
                    }
                },
                Token::KeywordIs => {
                    self.tokens.next(); // Consume "is"
                    
                    if let Some(next_token) = self.tokens.peek().cloned() {
                        match next_token.token {
                            Token::KeywordEqual => {
                                self.tokens.next(); // Consume "equal"
                                
                                if let Some(to_token) = self.tokens.peek().cloned() {
                                    if matches!(to_token.token, Token::KeywordTo) {
                                        self.tokens.next(); // Consume "to"
                                        Some((Operator::Equals, 0))
                                    } else {
                                        Some((Operator::Equals, 0)) // "is equal" without "to" is valid too
                                    }
                                } else {
                                    return Err(ParseError::new(
                                        "Unexpected end of input after 'is equal'".to_string(),
                                        line,
                                        column,
                                    ));
                                }
                            },
                            Token::KeywordNot => {
                                self.tokens.next(); // Consume "not"
                                Some((Operator::NotEquals, 0))
                            },
                            Token::KeywordGreater => {
                                self.tokens.next(); // Consume "greater"
                                
                                if let Some(than_token) = self.tokens.peek().cloned() {
                                    if let Token::Identifier(id) = &than_token.token {
                                        if id == "than" {
                                            self.tokens.next(); // Consume "than"
                                            Some((Operator::GreaterThan, 0))
                                        } else {
                                            Some((Operator::GreaterThan, 0)) // "is greater" without "than" is valid too
                                        }
                                    } else {
                                        Some((Operator::GreaterThan, 0)) // "is greater" without "than" is valid too
                                    }
                                } else {
                                    return Err(ParseError::new(
                                        "Unexpected end of input after 'is greater'".to_string(),
                                        line,
                                        column,
                                    ));
                                }
                            },
                            Token::KeywordLess => {
                                self.tokens.next(); // Consume "less"
                                
                                if let Some(than_token) = self.tokens.peek().cloned() {
                                    if let Token::Identifier(id) = &than_token.token {
                                        if id == "than" {
                                            self.tokens.next(); // Consume "than"
                                            Some((Operator::LessThan, 0))
                                        } else {
                                            Some((Operator::LessThan, 0)) // "is less" without "than" is valid too
                                        }
                                    } else {
                                        Some((Operator::LessThan, 0)) // "is less" without "than" is valid too
                                    }
                                } else {
                                    return Err(ParseError::new(
                                        "Unexpected end of input after 'is less'".to_string(),
                                        line,
                                        column,
                                    ));
                                }
                            },
                            _ => Some((Operator::Equals, 0)), // Simple "is" means equals
                        }
                    } else {
                        return Err(ParseError::new(
                            "Unexpected end of input after 'is'".to_string(),
                            line,
                            column,
                        ));
                    }
                },
                Token::KeywordWith => {
                    self.tokens.next(); // Consume "with"
                    let right = self.parse_expression()?;
                    left = Expression::Concatenation {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    continue; // Skip the rest of the loop since we've already updated left
                },
                Token::KeywordAnd => {
                    self.tokens.next(); // Consume "and"
                    Some((Operator::And, 0))
                },
                Token::KeywordOr => {
                    self.tokens.next(); // Consume "or"
                    Some((Operator::Or, 0))
                },
                Token::KeywordContains => {
                    self.tokens.next(); // Consume "contains"
                    Some((Operator::Contains, 0))
                },
                _ => None,
            };
            
            if let Some((operator, op_precedence)) = op {
                if op_precedence < precedence {
                    break;
                }
                
                let right = self.parse_binary_expression(op_precedence + 1)?;
                
                left = Expression::BinaryOperation {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_primary_expression(&mut self) -> Result<Expression, ParseError> {
        if let Some(token) = self.tokens.peek().cloned() {
            let result = match &token.token {
                Token::StringLiteral(s) => {
                    self.tokens.next();
                    Ok(Expression::Literal(Literal::String(s.clone())))
                },
                Token::IntLiteral(n) => {
                    self.tokens.next();
                    Ok(Expression::Literal(Literal::Integer(*n)))
                },
                Token::FloatLiteral(f) => {
                    self.tokens.next();
                    Ok(Expression::Literal(Literal::Float(*f)))
                },
                Token::BooleanLiteral(b) => {
                    self.tokens.next();
                    Ok(Expression::Literal(Literal::Boolean(*b)))
                },
                Token::NothingLiteral => {
                    self.tokens.next();
                    Ok(Expression::Literal(Literal::Nothing))
                },
                Token::Identifier(name) => {
                    self.tokens.next();
                    
                    if let Some(next_token) = self.tokens.peek().cloned() {
                        if let Token::Identifier(id) = &next_token.token {
                            if id.to_lowercase() == "with" {
                                self.tokens.next(); // Consume "with"
                                
                                let mut arguments = Vec::new();
                                
                                loop {
                                    let arg_name = if let Some(name_token) = self.tokens.peek().cloned() {
                                        if let Token::Identifier(id) = &name_token.token {
                                            if let Some(next) = self.tokens.clone().nth(1) {
                                                if matches!(next.token, Token::Colon) {
                                                    self.tokens.next(); // Consume name
                                                    self.tokens.next(); // Consume ":"
                                                    Some(id.clone())
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    };
                                    
                                    let arg_value = self.parse_expression()?;
                                    
                                    arguments.push(Argument {
                                        name: arg_name,
                                        value: arg_value,
                                    });
                                    
                                    if let Some(token) = self.tokens.peek().cloned() {
                                        if let Token::Identifier(id) = &token.token {
                                            if id.to_lowercase() == "and" {
                                                self.tokens.next(); // Consume "and"
                                            } else {
                                                break;
                                            }
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                
                                return Ok(Expression::FunctionCall {
                                    function: Box::new(Expression::Variable(name.clone())),
                                    arguments,
                                });
                            }
                        }
                    }
                    
                    Ok(Expression::Variable(name.clone()))
                },
                Token::KeywordNot => {
                    self.tokens.next(); // Consume "not"
                    let expr = self.parse_primary_expression()?;
                    Ok(Expression::UnaryOperation {
                        operator: UnaryOperator::Not,
                        expression: Box::new(expr),
                    })
                },
                Token::KeywordWith => {
                    self.tokens.next(); // Consume "with"
                    let expr = self.parse_expression()?;
                    Ok(expr)
                },
                _ => Err(ParseError::new(
                    format!("Unexpected token in expression: {:?}", token.token),
                    token.line,
                    token.column,
                )),
            };
            
            if let Ok(mut expr) = result {
                loop {
                    if let Some(token) = self.tokens.peek().cloned() {
                        match &token.token {
                            Token::Identifier(id) if id == "of" => {
                                self.tokens.next(); // Consume "of"
                                
                                if let Some(prop_token) = self.tokens.peek().cloned() {
                                    if let Token::Identifier(prop) = &prop_token.token {
                                        self.tokens.next(); // Consume property name
                                        expr = Expression::MemberAccess {
                                            object: Box::new(expr),
                                            property: prop.clone(),
                                        };
                                    } else {
                                        return Err(ParseError::new(
                                            format!("Expected identifier after 'of', found {:?}", prop_token.token),
                                            prop_token.line,
                                            prop_token.column,
                                        ));
                                    }
                                } else {
                                    return Err(ParseError::new(
                                        "Unexpected end of input after 'of'".to_string(),
                                        token.line,
                                        token.column,
                                    ));
                                }
                            },
                            Token::KeywordAt => {
                                self.tokens.next(); // Consume "at"
                                
                                let index = self.parse_expression()?;
                                
                                expr = Expression::IndexAccess {
                                    collection: Box::new(expr),
                                    index: Box::new(index),
                                };
                            },
                            _ => break,
                        }
                    } else {
                        break;
                    }
                }
                
                Ok(expr)
            } else {
                result
            }
        } else {
            Err(ParseError::new(
                "Unexpected end of input while parsing expression".to_string(),
                0,
                0,
            ))
        }
    }
    
    fn parse_display_statement(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "display"
        
        let expr = self.parse_expression()?;
        
        Ok(Statement::DisplayStatement { value: expr })
    }
    
    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "check"
        
        self.expect_token(Token::KeywordIf, "Expected 'if' after 'check'")?;
        
        let condition = self.parse_expression()?;
        
        self.expect_token(Token::Colon, "Expected ':' after if condition")?;
        
        let mut then_block = Vec::new();
        
        while let Some(token) = self.tokens.peek() {
            match &token.token {
                Token::KeywordOtherwise | Token::KeywordEnd => {
                    break;
                }
                _ => match self.parse_statement() {
                    Ok(stmt) => then_block.push(stmt),
                    Err(e) => return Err(e),
                },
            }
        }
        
        let else_block = if let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordOtherwise) {
                self.tokens.next(); // Consume "otherwise"
                
                self.expect_token(Token::Colon, "Expected ':' after 'otherwise'")?;
                
                let mut else_stmts = Vec::new();
                
                while let Some(token) = self.tokens.peek() {
                    if matches!(token.token, Token::KeywordEnd) {
                        break;
                    }
                    
                    match self.parse_statement() {
                        Ok(stmt) => else_stmts.push(stmt),
                        Err(e) => return Err(e),
                    }
                }
                
                Some(else_stmts)
            } else {
                None
            }
        } else {
            None
        };
        
        self.expect_token(Token::KeywordEnd, "Expected 'end' after if block")?;
        self.expect_token(Token::KeywordCheck, "Expected 'check' after 'end'")?;
        
        Ok(Statement::IfStatement {
            condition,
            then_block,
            else_block,
        })
    }
    
    fn parse_single_line_if(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "if"
        
        let condition = self.parse_expression()?;
        
        self.expect_token(Token::KeywordThen, "Expected 'then' after if condition")?;
        
        let then_stmt = Box::new(self.parse_statement()?);
        
        let else_stmt = if let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordOtherwise) {
                self.tokens.next(); // Consume "otherwise"
                Some(Box::new(self.parse_statement()?))
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(Statement::SingleLineIf {
            condition,
            then_stmt,
            else_stmt,
        })
    }
    
    fn parse_for_each_loop(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "for"
        
        self.expect_token(Token::KeywordEach, "Expected 'each' after 'for'")?;
        
        let item_name = if let Some(token) = self.tokens.peek() {
            if let Token::Identifier(id) = &token.token {
                self.tokens.next();
                id.clone()
            } else {
                return Err(ParseError::new(
                    format!("Expected identifier after 'each', found {:?}", token.token),
                    token.line,
                    token.column,
                ));
            }
        } else {
            return Err(ParseError::new(
                "Unexpected end of input after 'each'".to_string(),
                0,
                0,
            ));
        };
        
        self.expect_token(Token::KeywordIn, "Expected 'in' after item name")?;
        
        let reversed = if let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordReversed) {
                self.tokens.next(); // Consume "reversed"
                true
            } else {
                false
            }
        } else {
            false
        };
        
        let collection = self.parse_expression()?;
        
        self.expect_token(Token::Colon, "Expected ':' after for-each loop collection")?;
        
        let mut body = Vec::new();
        
        while let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordEnd) {
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(e),
            }
        }
        
        self.expect_token(Token::KeywordEnd, "Expected 'end' after for-each loop body")?;
        self.expect_token(Token::KeywordFor, "Expected 'for' after 'end'")?;
        
        Ok(Statement::ForEachLoop {
            item_name,
            collection,
            reversed,
            body,
        })
    }
    
    fn parse_count_loop(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "count"
        
        self.expect_token(Token::KeywordFrom, "Expected 'from' after 'count'")?;
        
        let start = self.parse_expression()?;
        
        let downward = if let Some(token) = self.tokens.peek() {
            if let Token::Identifier(id) = &token.token {
                if id.to_lowercase() == "down" {
                    self.tokens.next(); // Consume "down"
                    self.expect_token(Token::KeywordTo, "Expected 'to' after 'down'")?;
                    true
                } else if matches!(token.token, Token::KeywordTo) {
                    self.tokens.next(); // Consume "to"
                    false
                } else {
                    return Err(ParseError::new(
                        format!("Expected 'to' or 'down to', found {:?}", token.token),
                        token.line,
                        token.column,
                    ));
                }
            } else if matches!(token.token, Token::KeywordTo) {
                self.tokens.next(); // Consume "to"
                false
            } else {
                return Err(ParseError::new(
                    format!("Expected 'to' or 'down to', found {:?}", token.token),
                    token.line,
                    token.column,
                ));
            }
        } else {
            return Err(ParseError::new(
                "Unexpected end of input after count from expression".to_string(),
                0,
                0,
            ));
        };
        
        let end = self.parse_expression()?;
        
        let step = if let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordBy) {
                self.tokens.next(); // Consume "by"
                Some(self.parse_expression()?)
            } else {
                None
            }
        } else {
            None
        };
        
        self.expect_token(Token::Colon, "Expected ':' after count loop range")?;
        
        let mut body = Vec::new();
        
        while let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordEnd) {
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(e),
            }
        }
        
        self.expect_token(Token::KeywordEnd, "Expected 'end' after count loop body")?;
        self.expect_token(Token::KeywordCount, "Expected 'count' after 'end'")?;
        
        Ok(Statement::CountLoop {
            start,
            end,
            step,
            downward,
            body,
        })
    }
    
    fn parse_action_definition(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "define"
        
        self.expect_token(Token::KeywordAction, "Expected 'action' after 'define'")?;
        self.expect_token(Token::KeywordCalled, "Expected 'called' after 'action'")?;
        
        let name = if let Some(token) = self.tokens.peek() {
            if let Token::Identifier(id) = &token.token {
                self.tokens.next();
                id.clone()
            } else {
                return Err(ParseError::new(
                    format!("Expected identifier after 'called', found {:?}", token.token),
                    token.line,
                    token.column,
                ));
            }
        } else {
            return Err(ParseError::new(
                "Unexpected end of input after 'called'".to_string(),
                0,
                0,
            ));
        };
        
        let mut parameters = Vec::new();
        
        if let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordWith) {
                self.tokens.next(); // Consume "with"
                
                loop {
                    let param_name = if let Some(token) = self.tokens.peek() {
                        if let Token::Identifier(id) = &token.token {
                            self.tokens.next();
                            id.clone()
                        } else {
                            break;
                        }
                    } else {
                        break;
                    };
                    
                    let param_type = if let Some(token) = self.tokens.peek() {
                        if matches!(token.token, Token::KeywordAs) {
                            self.tokens.next(); // Consume "as"
                            
                            if let Some(type_token) = self.tokens.peek() {
                                if let Token::Identifier(type_name) = &type_token.token {
                                    self.tokens.next();
                                    
                                    let typ = match type_name.as_str() {
                                        "text" => Type::Text,
                                        "number" => Type::Number,
                                        "boolean" => Type::Boolean,
                                        "nothing" => Type::Nothing,
                                        _ => Type::Custom(type_name.clone()),
                                    };
                                    
                                    Some(typ)
                                } else {
                                    return Err(ParseError::new(
                                        format!("Expected type name after 'as', found {:?}", type_token.token),
                                        type_token.line,
                                        type_token.column,
                                    ));
                                }
                            } else {
                                return Err(ParseError::new(
                                    "Unexpected end of input after 'as'".to_string(),
                                    0,
                                    0,
                                ));
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    
                    let default_value = if let Some(token) = self.tokens.peek() {
                        if let Token::Identifier(id) = &token.token {
                            if id.to_lowercase() == "default" {
                                self.tokens.next(); // Consume "default"
                                
                                Some(self.parse_expression()?)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    
                    parameters.push(Parameter {
                        name: param_name,
                        param_type,
                        default_value,
                    });
                    
                    if let Some(token) = self.tokens.peek() {
                        if let Token::Identifier(id) = &token.token {
                            if id == "and" {
                                self.tokens.next(); // Consume "and"
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        
        let return_type = if let Some(token) = self.tokens.peek() {
            if let Token::Identifier(id) = &token.token {
                if id.to_lowercase() == "returns" {
                    self.tokens.next(); // Consume "returns"
                    
                    if let Some(type_token) = self.tokens.peek() {
                        if let Token::Identifier(type_name) = &type_token.token {
                            self.tokens.next();
                            
                            let typ = match type_name.as_str() {
                                "text" => Type::Text,
                                "number" => Type::Number,
                                "boolean" => Type::Boolean,
                                "nothing" => Type::Nothing,
                                _ => Type::Custom(type_name.clone()),
                            };
                            
                            Some(typ)
                        } else {
                            return Err(ParseError::new(
                                format!("Expected type name after 'returns', found {:?}", type_token.token),
                                type_token.line,
                                type_token.column,
                            ));
                        }
                    } else {
                        return Err(ParseError::new(
                            "Unexpected end of input after 'returns'".to_string(),
                            0,
                            0,
                        ));
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        self.expect_token(Token::Colon, "Expected ':' after action definition")?;
        
        let mut body = Vec::new();
        
        while let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::KeywordEnd) {
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(e),
            }
        }
        
        self.expect_token(Token::KeywordEnd, "Expected 'end' after action body")?;
        self.expect_token(Token::KeywordAction, "Expected 'action' after 'end'")?;
        
        Ok(Statement::ActionDefinition {
            name,
            parameters,
            body,
            return_type,
        })
    }
    
    fn parse_assignment(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "change"
        
        let mut name = String::new();
        
        while let Some(token) = self.tokens.peek() {
            if let Token::Identifier(id) = &token.token {
                if !name.is_empty() {
                    name.push(' ');
                }
                name.push_str(id);
                self.tokens.next();
            } else if let Token::KeywordTo = &token.token {
                break;
            } else {
                return Err(ParseError::new(
                    format!("Expected identifier or 'to', found {:?}", token.token),
                    token.line,
                    token.column,
                ));
            }
        }
        
        self.expect_token(Token::KeywordTo, "Expected 'to' after variable name")?;
        
        let value = self.parse_expression()?;
        
        Ok(Statement::Assignment { name, value })
    }
    
    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "give" or "return"
        
        let value = if let Some(token) = self.tokens.peek() {
            if matches!(token.token, Token::NothingLiteral) {
                self.tokens.next(); // Consume "nothing"
                None
            } else {
                Some(self.parse_expression()?)
            }
        } else {
            None
        };
        
        Ok(Statement::ReturnStatement { value })
    }
    
    fn parse_open_file_statement(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "open"
        
        self.expect_token(Token::KeywordFile, "Expected 'file' after 'open'")?;
        
        let path = self.parse_expression()?;
        
        self.expect_token(Token::KeywordAs, "Expected 'as' after file path")?;
        
        let variable_name = if let Some(token) = self.tokens.peek() {
            if let Token::Identifier(id) = &token.token {
                self.tokens.next();
                id.clone()
            } else {
                return Err(ParseError::new(
                    format!("Expected identifier after 'as', found {:?}", token.token),
                    token.line,
                    token.column,
                ));
            }
        } else {
            return Err(ParseError::new(
                "Unexpected end of input after 'as'".to_string(),
                0,
                0,
            ));
        };
        
        Ok(Statement::OpenFileStatement {
            path,
            variable_name,
        })
    }
    
    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression()?;
        
        Ok(Statement::ExpressionStatement { expression: expr })
    }
    

}
