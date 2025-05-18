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
            errors: Vec::with_capacity(4),
        }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut program = Program::new();
        program.statements.reserve(self.tokens.clone().count() / 5);

        while self.tokens.peek().is_some() {
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
        while let Some(token) = self.tokens.peek().cloned() {
            match &token.token {
                Token::KeywordStore
                | Token::KeywordCreate
                | Token::KeywordDisplay
                | Token::KeywordCheck
                | Token::KeywordCount
                | Token::KeywordFor
                | Token::KeywordDefine
                | Token::KeywordIf
                | Token::KeywordEnd => {
                    break;
                }
                _ => {
                    self.tokens.next(); // Skip the token
                }
            }
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        if let Some(token) = self.tokens.peek().cloned() {
            match &token.token {
                Token::KeywordStore | Token::KeywordCreate => self.parse_variable_declaration(),
                Token::KeywordDisplay => self.parse_display_statement(),
                Token::KeywordCheck => self.parse_if_statement(),
                Token::KeywordIf => self.parse_single_line_if(),
                Token::KeywordCount => self.parse_count_loop(),
                Token::KeywordFor => self.parse_for_each_loop(),
                Token::KeywordDefine => self.parse_action_definition(),
                Token::KeywordChange => self.parse_assignment(),
                Token::KeywordBreak => {
                    let token_pos = self.tokens.next().unwrap();
                    Ok(Statement::BreakStatement {
                        line: token_pos.line,
                        column: token_pos.column,
                    })
                }
                Token::KeywordContinue | Token::KeywordSkip => {
                    let token_pos = self.tokens.next().unwrap();
                    Ok(Statement::ContinueStatement {
                        line: token_pos.line,
                        column: token_pos.column,
                    })
                }
                Token::KeywordOpen => {
                    let mut tokens_clone = self.tokens.clone();
                    let mut has_read_pattern = false;

                    tokens_clone.next();

                    if let Some(token) = tokens_clone.next() {
                        if token.token == Token::KeywordFile {
                            if let Some(token) = tokens_clone.next() {
                                if token.token == Token::KeywordAt {
                                    if let Some(token) = tokens_clone.next() {
                                        if let Token::StringLiteral(_) = token.token {
                                            if let Some(token) = tokens_clone.next() {
                                                if token.token == Token::KeywordAnd {
                                                    if let Some(token) = tokens_clone.next() {
                                                        if token.token == Token::KeywordRead {
                                                            if let Some(token) = tokens_clone.next()
                                                            {
                                                                if token.token
                                                                    == Token::KeywordContent
                                                                {
                                                                    if let Some(token) =
                                                                        tokens_clone.next()
                                                                    {
                                                                        if token.token
                                                                            == Token::KeywordAs
                                                                        {
                                                                            if let Some(token) =
                                                                                tokens_clone.next()
                                                                            {
                                                                                if let Token::Identifier(_) = token.token {
                                                                                    has_read_pattern = true;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if has_read_pattern {
                        self.parse_open_file_read_statement()
                    } else {
                        self.parse_open_file_statement()
                    }
                }
                Token::KeywordWait => self.parse_wait_for_statement(),
                Token::KeywordGive | Token::KeywordReturn => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            }
        } else {
            Err(ParseError::new("Unexpected end of input".to_string(), 0, 0))
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParseError> {
        let token_pos = self.tokens.next().unwrap();
        let is_store = matches!(token_pos.token, Token::KeywordStore);
        let _keyword = if is_store { "store" } else { "create" };

        let name = self.parse_variable_name_list()?;

        if let Some(token) = self.tokens.peek().cloned() {
            if !matches!(token.token, Token::KeywordAs) {
                return Err(ParseError::new(
                    format!(
                        "Expected 'as' after variable name '{}', but found {:?}",
                        name, token.token
                    ),
                    token.line,
                    token.column,
                ));
            }
        } else {
            return Err(ParseError::new(
                format!(
                    "Expected 'as' after variable name '{}', but found end of input",
                    name
                ),
                token_pos.line,
                token_pos.column,
            ));
        }

        self.tokens.next(); // Consume the 'as' token

        let value = self.parse_expression()?;

        Ok(Statement::VariableDeclaration {
            name,
            value,
            line: token_pos.line,
            column: token_pos.column,
        })
    }

    fn parse_variable_name_list(&mut self) -> Result<String, ParseError> {
        let mut name_parts = Vec::with_capacity(3);

        if let Some(token) = self.tokens.peek().cloned() {
            match &token.token {
                Token::Identifier(id) => {
                    self.tokens.next(); // Consume the identifier
                    name_parts.push(id.clone());
                }
                Token::IntLiteral(_) | Token::FloatLiteral(_) => {
                    return Err(ParseError::new(
                        format!("Cannot use a number as a variable name: {:?}", token.token),
                        token.line,
                        token.column,
                    ));
                }
                Token::KeywordAs => {
                    return Err(ParseError::new(
                        "Expected a variable name before 'as'".to_string(),
                        token.line,
                        token.column,
                    ));
                }
                _ if token.token.is_keyword() => {
                    return Err(ParseError::new(
                        format!("Cannot use keyword '{:?}' as a variable name", token.token),
                        token.line,
                        token.column,
                    ));
                }
                _ => {
                    return Err(ParseError::new(
                        format!(
                            "Expected identifier for variable name, found {:?}",
                            token.token
                        ),
                        token.line,
                        token.column,
                    ));
                }
            }
        } else {
            return Err(ParseError::new(
                "Expected variable name but found end of input".to_string(),
                0,
                0,
            ));
        }

        while let Some(token) = self.tokens.peek().cloned() {
            match &token.token {
                Token::Identifier(id) => {
                    self.tokens.next(); // Consume the identifier
                    name_parts.push(id.clone());
                }
                Token::KeywordAs => {
                    break;
                }
                Token::IntLiteral(_) | Token::FloatLiteral(_) => {
                    return Err(ParseError::new(
                        format!(
                            "Expected 'as' after variable name, but found number: {:?}",
                            token.token
                        ),
                        token.line,
                        token.column,
                    ));
                }
                _ => {
                    return Err(ParseError::new(
                        format!(
                            "Expected 'as' after variable name, but found {:?}",
                            token.token
                        ),
                        token.line,
                        token.column,
                    ));
                }
            }
        }

        Ok(name_parts.join(" "))
    }

    fn expect_token(&mut self, expected: Token, error_message: &str) -> Result<(), ParseError> {
        if let Some(token) = self.tokens.peek().cloned() {
            if token.token == expected {
                self.tokens.next();
                Ok(())
            } else {
                Err(ParseError::new(
                    format!(
                        "{}: expected {:?}, found {:?}",
                        error_message, expected, token.token
                    ),
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
                }
                Token::KeywordMinus => {
                    self.tokens.next(); // Consume "minus"
                    Some((Operator::Minus, 1))
                }
                Token::KeywordTimes => {
                    self.tokens.next(); // Consume "times"
                    Some((Operator::Multiply, 2))
                }
                Token::KeywordDivided => {
                    self.tokens.next();
                    if let Some(by_token) = self.tokens.peek().cloned() {
                        if matches!(by_token.token, Token::KeywordBy) {
                            self.tokens.next(); // Consume "by"
                            Some((Operator::Divide, 2))
                        } else {
                            return Err(ParseError::new(
                                format!(
                                    "Expected 'by' after 'divided', found {:?}",
                                    by_token.token
                                ),
                                by_token.line,
                                by_token.column,
                            ));
                        }
                    } else {
                        return Err(ParseError::new(
                            "Unexpected end of input after 'divided'".into(),
                            line,
                            column,
                        ));
                    }
                }
                Token::KeywordIs => {
                    self.tokens.next(); // Consume "is"

                    if let Some(next_token) = self.tokens.peek().cloned() {
                        match &next_token.token {
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
                                        "Unexpected end of input after 'is equal'".into(),
                                        line,
                                        column,
                                    ));
                                }
                            }
                            Token::KeywordNot => {
                                self.tokens.next(); // Consume "not"
                                Some((Operator::NotEquals, 0))
                            }
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
                                        "Unexpected end of input after 'is greater'".into(),
                                        line,
                                        column,
                                    ));
                                }
                            }
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
                                        "Unexpected end of input after 'is less'".into(),
                                        line,
                                        column,
                                    ));
                                }
                            }
                            _ => Some((Operator::Equals, 0)), // Simple "is" means equals
                        }
                    } else {
                        return Err(ParseError::new(
                            "Unexpected end of input after 'is'".into(),
                            line,
                            column,
                        ));
                    }
                }
                Token::KeywordWith => {
                    self.tokens.next(); // Consume "with"
                    let right = self.parse_expression()?;
                    left = Expression::Concatenation {
                        left: Box::new(left),
                        right: Box::new(right),
                        line: token_pos.line,
                        column: token_pos.column,
                    };
                    continue; // Skip the rest of the loop since we've already updated left
                }
                Token::KeywordAnd => {
                    self.tokens.next(); // Consume "and"
                    Some((Operator::And, 0))
                }
                Token::KeywordOr => {
                    self.tokens.next(); // Consume "or"
                    Some((Operator::Or, 0))
                }
                Token::KeywordMatches => {
                    self.tokens.next(); // Consume "matches"

                    if let Some(pattern_token) = self.tokens.peek().cloned() {
                        if matches!(pattern_token.token, Token::KeywordPattern) {
                            self.tokens.next(); // Consume "pattern"

                            let pattern_expr = self.parse_binary_expression(precedence + 1)?;

                            left = Expression::PatternMatch {
                                text: Box::new(left),
                                pattern: Box::new(pattern_expr),
                                line,
                                column,
                            };
                            continue; // Skip the rest of the loop since we've already updated left
                        }
                    }

                    return Err(ParseError::new(
                        "Expected 'pattern' after 'matches'".to_string(),
                        line,
                        column,
                    ));
                }
                Token::KeywordFind => {
                    self.tokens.next(); // Consume "find"

                    if let Some(pattern_token) = self.tokens.peek().cloned() {
                        if matches!(pattern_token.token, Token::KeywordPattern) {
                            self.tokens.next(); // Consume "pattern"

                            let pattern_expr = self.parse_binary_expression(precedence + 1)?;

                            if let Some(in_token) = self.tokens.peek().cloned() {
                                if matches!(in_token.token, Token::KeywordIn) {
                                    self.tokens.next(); // Consume "in"

                                    let text_expr = self.parse_binary_expression(precedence + 1)?;

                                    left = Expression::PatternFind {
                                        text: Box::new(text_expr),
                                        pattern: Box::new(pattern_expr),
                                        line,
                                        column,
                                    };
                                    continue; // Skip the rest of the loop since we've already updated left
                                }
                            }

                            left = Expression::PatternFind {
                                text: Box::new(left),
                                pattern: Box::new(pattern_expr),
                                line,
                                column,
                            };
                            continue; // Skip the rest of the loop since we've already updated left
                        }
                    }

                    return Err(ParseError::new(
                        "Expected 'pattern' after 'find'".to_string(),
                        line,
                        column,
                    ));
                }
                Token::KeywordReplace => {
                    self.tokens.next(); // Consume "replace"

                    if let Some(pattern_token) = self.tokens.peek().cloned() {
                        if matches!(pattern_token.token, Token::KeywordPattern) {
                            self.tokens.next(); // Consume "pattern"

                            let pattern_expr = self.parse_binary_expression(precedence + 1)?;

                            if let Some(with_token) = self.tokens.peek().cloned() {
                                if matches!(with_token.token, Token::KeywordWith) {
                                    self.tokens.next(); // Consume "with"

                                    let replacement_expr =
                                        self.parse_binary_expression(precedence + 1)?;

                                    if let Some(in_token) = self.tokens.peek().cloned() {
                                        if matches!(in_token.token, Token::KeywordIn) {
                                            self.tokens.next(); // Consume "in"

                                            let text_expr =
                                                self.parse_binary_expression(precedence + 1)?;

                                            left = Expression::PatternReplace {
                                                text: Box::new(text_expr),
                                                pattern: Box::new(pattern_expr),
                                                replacement: Box::new(replacement_expr),
                                                line,
                                                column,
                                            };
                                            continue; // Skip the rest of the loop since we've already updated left
                                        }
                                    }

                                    left = Expression::PatternReplace {
                                        text: Box::new(left),
                                        pattern: Box::new(pattern_expr),
                                        replacement: Box::new(replacement_expr),
                                        line,
                                        column,
                                    };
                                    continue; // Skip the rest of the loop since we've already updated left
                                }
                            }

                            return Err(ParseError::new(
                                "Expected 'with' after pattern in replace operation".to_string(),
                                line,
                                column,
                            ));
                        }
                    }

                    return Err(ParseError::new(
                        "Expected 'pattern' after 'replace'".to_string(),
                        line,
                        column,
                    ));
                }
                Token::KeywordSplit => {
                    self.tokens.next(); // Consume "split"

                    if let Some(by_token) = self.tokens.peek().cloned() {
                        if matches!(by_token.token, Token::KeywordBy) {
                            self.tokens.next(); // Consume "by"

                            if let Some(pattern_token) = self.tokens.peek().cloned() {
                                if matches!(pattern_token.token, Token::KeywordPattern) {
                                    self.tokens.next(); // Consume "pattern"

                                    let pattern_expr =
                                        self.parse_binary_expression(precedence + 1)?;

                                    left = Expression::PatternSplit {
                                        text: Box::new(left),
                                        pattern: Box::new(pattern_expr),
                                        line,
                                        column,
                                    };
                                    continue; // Skip the rest of the loop since we've already updated left
                                }
                            }

                            return Err(ParseError::new(
                                "Expected 'pattern' after 'by' in split operation".to_string(),
                                line,
                                column,
                            ));
                        }
                    }

                    return Err(ParseError::new(
                        "Expected 'by' after 'split'".to_string(),
                        line,
                        column,
                    ));
                }
                Token::KeywordContains => {
                    self.tokens.next(); // Consume "contains"

                    if let Some(pattern_token) = self.tokens.peek().cloned() {
                        if matches!(pattern_token.token, Token::KeywordPattern) {
                            self.tokens.next(); // Consume "pattern"

                            let pattern_expr = self.parse_binary_expression(precedence + 1)?;

                            left = Expression::PatternMatch {
                                text: Box::new(left),
                                pattern: Box::new(pattern_expr),
                                line,
                                column,
                            };
                            continue; // Skip the rest of the loop since we've already updated left
                        }
                    }

                    Some((Operator::Contains, 0))
                }
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
                    line: token_pos.line,
                    column: token_pos.column,
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
                Token::LeftParen => {
                    self.tokens.next(); // Consume '('
                    let expr = self.parse_expression()?;

                    if let Some(token) = self.tokens.peek().cloned() {
                        if token.token == Token::RightParen {
                            self.tokens.next(); // Consume ')'
                            return Ok(expr);
                        } else {
                            return Err(ParseError::new(
                                format!("Expected closing parenthesis, found {:?}", token.token),
                                token.line,
                                token.column,
                            ));
                        }
                    } else {
                        return Err(ParseError::new(
                            "Expected closing parenthesis, found end of input".into(),
                            token.line,
                            token.column,
                        ));
                    }
                }
                Token::StringLiteral(s) => {
                    let token_pos = self.tokens.next().unwrap();
                    Ok(Expression::Literal(
                        Literal::String(s.to_string()),
                        token_pos.line,
                        token_pos.column,
                    ))
                }
                Token::IntLiteral(n) => {
                    let token_pos = self.tokens.next().unwrap();
                    Ok(Expression::Literal(
                        Literal::Integer(*n),
                        token_pos.line,
                        token_pos.column,
                    ))
                }
                Token::FloatLiteral(f) => {
                    let token_pos = self.tokens.next().unwrap();
                    Ok(Expression::Literal(
                        Literal::Float(*f),
                        token_pos.line,
                        token_pos.column,
                    ))
                }
                Token::BooleanLiteral(b) => {
                    let token_pos = self.tokens.next().unwrap();
                    Ok(Expression::Literal(
                        Literal::Boolean(*b),
                        token_pos.line,
                        token_pos.column,
                    ))
                }
                Token::NothingLiteral => {
                    let token_pos = self.tokens.next().unwrap();
                    Ok(Expression::Literal(
                        Literal::Nothing,
                        token_pos.line,
                        token_pos.column,
                    ))
                }
                Token::Identifier(name) => {
                    self.tokens.next();

                    if let Some(next_token) = self.tokens.peek().cloned() {
                        if let Token::Identifier(id) = &next_token.token {
                            if id.to_lowercase() == "with" {
                                self.tokens.next(); // Consume "with"

                                let mut arguments = Vec::with_capacity(4);

                                loop {
                                    let arg_name =
                                        if let Some(name_token) = self.tokens.peek().cloned() {
                                            if let Token::Identifier(id) = &name_token.token {
                                                if let Some(next) = self.tokens.clone().nth(1) {
                                                    if matches!(next.token, Token::Colon) {
                                                        self.tokens.next(); // Consume name
                                                        self.tokens.next(); // Consume ":"
                                                        Some(id.to_string())
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

                                let token_line = token.line;
                                let token_column = token.column;
                                return Ok(Expression::FunctionCall {
                                    function: Box::new(Expression::Variable(
                                        name.clone(),
                                        token_line,
                                        token_column,
                                    )),
                                    arguments,
                                    line: token_line,
                                    column: token_column,
                                });
                            }
                        }
                    }

                    let token_line = token.line;
                    let token_column = token.column;
                    Ok(Expression::Variable(name.clone(), token_line, token_column))
                }
                Token::KeywordNot => {
                    self.tokens.next(); // Consume "not"
                    let expr = self.parse_primary_expression()?;
                    let token_line = token.line;
                    let token_column = token.column;
                    Ok(Expression::UnaryOperation {
                        operator: UnaryOperator::Not,
                        expression: Box::new(expr),
                        line: token_line,
                        column: token_column,
                    })
                }
                Token::KeywordWith => {
                    self.tokens.next(); // Consume "with"
                    let expr = self.parse_expression()?;
                    Ok(expr)
                }
                Token::KeywordCount => {
                    self.tokens.next(); // Consume "count"
                    let token_line = token.line;
                    let token_column = token.column;
                    Ok(Expression::Variable(
                        "count".to_string(),
                        token_line,
                        token_column,
                    ))
                }
                Token::KeywordPattern => {
                    self.tokens.next(); // Consume "pattern"

                    if let Some(pattern_token) = self.tokens.peek().cloned() {
                        if let Token::StringLiteral(pattern) = &pattern_token.token {
                            let token_pos = self.tokens.next().unwrap();
                            return Ok(Expression::Literal(
                                Literal::Pattern(pattern.clone()),
                                token_pos.line,
                                token_pos.column,
                            ));
                        } else {
                            return Err(ParseError::new(
                                format!(
                                    "Expected string literal after 'pattern', found {:?}",
                                    pattern_token.token
                                ),
                                pattern_token.line,
                                pattern_token.column,
                            ));
                        }
                    } else {
                        return Err(ParseError::new(
                            "Unexpected end of input after 'pattern'".to_string(),
                            token.line,
                            token.column,
                        ));
                    }
                }
                _ => Err(ParseError::new(
                    format!("Unexpected token in expression: {:?}", token.token),
                    token.line,
                    token.column,
                )),
            };

            if let Ok(mut expr) = result {
                while let Some(token) = self.tokens.peek().cloned() {
                    match &token.token {
                        Token::Identifier(id) if id == "of" => {
                            self.tokens.next(); // Consume "of"

                            if let Some(prop_token) = self.tokens.peek().cloned() {
                                if let Token::Identifier(prop) = &prop_token.token {
                                    self.tokens.next(); // Consume property name or first argument

                                    // In member access: "property of object", the left side is usually a property name

                                    let is_function_call = matches!(
                                        expr,
                                        Expression::Variable(_, _, _)
                                            | Expression::FunctionCall { .. }
                                    );

                                    if is_function_call {
                                        let mut arguments = Vec::with_capacity(4);

                                        arguments.push(Argument {
                                            name: None,
                                            value: Expression::Variable(
                                                prop.to_string(),
                                                prop_token.line,
                                                prop_token.column,
                                            ),
                                        });

                                        while let Some(and_token) = self.tokens.peek().cloned() {
                                            if let Token::Identifier(id) = &and_token.token {
                                                if id.to_lowercase() == "and" {
                                                    self.tokens.next(); // Consume "and"

                                                    let arg_value = self.parse_expression()?;

                                                    arguments.push(Argument {
                                                        name: None,
                                                        value: arg_value,
                                                    });
                                                } else {
                                                    break;
                                                }
                                            } else {
                                                break;
                                            }
                                        }

                                        expr = Expression::FunctionCall {
                                            function: Box::new(expr),
                                            arguments,
                                            line: token.line,
                                            column: token.column,
                                        };
                                    } else {
                                        expr = Expression::MemberAccess {
                                            object: Box::new(expr),
                                            property: prop.to_string(),
                                            line: prop_token.line,
                                            column: prop_token.column,
                                        };
                                    }
                                } else {
                                    return Err(ParseError::new(
                                        format!(
                                            "Expected identifier after 'of', found {:?}",
                                            prop_token.token
                                        ),
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
                        }
                        Token::KeywordAt => {
                            self.tokens.next(); // Consume "at"

                            let index = self.parse_expression()?;

                            expr = Expression::IndexAccess {
                                collection: Box::new(expr),
                                index: Box::new(index),
                                line: token.line,
                                column: token.column,
                            };
                        }
                        _ => break,
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

        let token_pos = if let Some(token) = self.tokens.peek() {
            token
        } else {
            return match expr {
                Expression::Literal(_, line, column) => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::Variable(_, line, column) => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::BinaryOperation { line, column, .. } => {
                    Ok(Statement::DisplayStatement {
                        value: expr,
                        line,
                        column,
                    })
                }
                Expression::UnaryOperation { line, column, .. } => {
                    Ok(Statement::DisplayStatement {
                        value: expr,
                        line,
                        column,
                    })
                }
                Expression::FunctionCall { line, column, .. } => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::MemberAccess { line, column, .. } => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::IndexAccess { line, column, .. } => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::Concatenation { line, column, .. } => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::PatternMatch { line, column, .. } => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::PatternFind { line, column, .. } => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::PatternReplace { line, column, .. } => {
                    Ok(Statement::DisplayStatement {
                        value: expr,
                        line,
                        column,
                    })
                }
                Expression::PatternSplit { line, column, .. } => Ok(Statement::DisplayStatement {
                    value: expr,
                    line,
                    column,
                }),
                Expression::AwaitExpression { line, column, .. } => {
                    Ok(Statement::DisplayStatement {
                        value: expr,
                        line,
                        column,
                    })
                }
            };
        };

        Ok(Statement::DisplayStatement {
            value: expr,
            line: token_pos.line,
            column: token_pos.column,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "check"

        self.expect_token(Token::KeywordIf, "Expected 'if' after 'check'")?;

        let condition = self.parse_expression()?;

        self.expect_token(Token::Colon, "Expected ':' after if condition")?;

        let mut then_block = Vec::with_capacity(8);

        while let Some(token) = self.tokens.peek().cloned() {
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

                let mut else_stmts = Vec::with_capacity(8);

                while let Some(token) = self.tokens.peek().cloned() {
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

        let default_token = TokenWithPosition {
            token: Token::KeywordCheck,
            line: 0,
            column: 0,
            length: 0,
        };
        let token_pos = self.tokens.peek().map_or(&default_token, |v| v);
        Ok(Statement::IfStatement {
            condition,
            then_block,
            else_block,
            line: token_pos.line,
            column: token_pos.column,
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

        let token_pos = self.tokens.peek().map_or(
            &TokenWithPosition {
                token: Token::KeywordIf,
                line: 0,
                column: 0,
                length: 0,
            },
            |v| v,
        );
        Ok(Statement::SingleLineIf {
            condition,
            then_stmt,
            else_stmt,
            line: token_pos.line,
            column: token_pos.column,
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

        let mut body = Vec::with_capacity(10);

        while let Some(token) = self.tokens.peek().cloned() {
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

        let token_pos = self.tokens.peek().map_or(
            &TokenWithPosition {
                token: Token::KeywordFor,
                line: 0,
                column: 0,
                length: 0,
            },
            |v| v,
        );
        Ok(Statement::ForEachLoop {
            item_name,
            collection,
            reversed,
            body,
            line: token_pos.line,
            column: token_pos.column,
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

        let mut body = Vec::with_capacity(10);

        while let Some(token) = self.tokens.peek().cloned() {
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

        let token_pos = self.tokens.peek().map_or(
            &TokenWithPosition {
                token: Token::KeywordCount,
                line: 0,
                column: 0,
                length: 0,
            },
            |v| v,
        );
        Ok(Statement::CountLoop {
            start,
            end,
            step,
            downward,
            body,
            line: token_pos.line,
            column: token_pos.column,
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
                    format!(
                        "Expected identifier after 'called', found {:?}",
                        token.token
                    ),
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

        let mut parameters = Vec::with_capacity(4);

        if let Some(token) = self.tokens.peek().cloned() {
            if matches!(token.token, Token::KeywordWith) {
                self.tokens.next(); // Consume "with"

                while let Some(token) = self.tokens.peek().cloned() {
                    let param_name = if let Token::Identifier(id) = &token.token {
                        self.tokens.next();
                        id.clone()
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
                                        format!(
                                            "Expected type name after 'as', found {:?}",
                                            type_token.token
                                        ),
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

                    if let Some(token) = self.tokens.peek().cloned() {
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
                                format!(
                                    "Expected type name after 'returns', found {:?}",
                                    type_token.token
                                ),
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

        let mut body = Vec::with_capacity(10);

        while let Some(token) = self.tokens.peek().cloned() {
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

        let token_pos = self.tokens.peek().map_or(
            &TokenWithPosition {
                token: Token::KeywordDefine,
                line: 0,
                column: 0,
                length: 0,
            },
            |v| v,
        );
        Ok(Statement::ActionDefinition {
            name,
            parameters,
            body,
            return_type,
            line: token_pos.line,
            column: token_pos.column,
        })
    }

    fn parse_assignment(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "change"

        let mut name = String::new();
        let mut has_identifier = false;

        while let Some(token) = self.tokens.peek().cloned() {
            if let Token::Identifier(id) = &token.token {
                has_identifier = true;
                if !name.is_empty() {
                    name.push(' ');
                }
                name.push_str(id);
                self.tokens.next();
            } else if let Token::KeywordTo = &token.token {
                break;
            } else {
                // Provide a more specific error message if we've seen at least one identifier
                if has_identifier {
                    return Err(ParseError::new(
                        format!(
                            "Expected 'to' after identifier(s), but found {:?}",
                            token.token
                        ),
                        token.line,
                        token.column,
                    ));
                } else {
                    return Err(ParseError::new(
                        format!("Expected identifier or 'to', found {:?}", token.token),
                        token.line,
                        token.column,
                    ));
                }
            }
        }

        self.expect_token(
            Token::KeywordTo,
            "Expected 'to' after variable name in change statement",
        )?;

        let value = self.parse_expression()?;

        let token_pos = self.tokens.peek().map_or(
            &TokenWithPosition {
                token: Token::KeywordChange,
                line: 0,
                column: 0,
                length: 0,
            },
            |v| v,
        );
        Ok(Statement::Assignment {
            name,
            value,
            line: token_pos.line,
            column: token_pos.column,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.tokens.next(); // Consume "give" or "return"

        let value = if let Some(token) = self.tokens.peek().cloned() {
            if matches!(token.token, Token::NothingLiteral) {
                self.tokens.next(); // Consume "nothing"
                None
            } else {
                Some(self.parse_expression()?)
            }
        } else {
            None
        };

        let token_pos = self.tokens.peek().map_or(
            &TokenWithPosition {
                token: Token::KeywordGive,
                line: 0,
                column: 0,
                length: 0,
            },
            |v| v,
        );
        Ok(Statement::ReturnStatement {
            value,
            line: token_pos.line,
            column: token_pos.column,
        })
    }

    fn parse_open_file_statement(&mut self) -> Result<Statement, ParseError> {
        let open_token = self.tokens.next().unwrap(); // Consume "open"

        self.expect_token(Token::KeywordFile, "Expected 'file' after 'open'")?;

        if let Some(token) = self.tokens.peek().cloned() {
            if token.token == Token::KeywordAt {
                self.tokens.next(); // Consume "at"

                let path_expr = if let Some(token) = self.tokens.peek().cloned() {
                    if let Token::StringLiteral(path_str) = &token.token {
                        let token_clone = token;
                        self.tokens.next(); // Consume the string literal
                        Expression::Literal(
                            Literal::String(path_str.clone()),
                            token_clone.line,
                            token_clone.column,
                        )
                    } else {
                        return Err(ParseError::new(
                            format!(
                                "Expected string literal for file path, found {:?}",
                                token.token
                            ),
                            token.line,
                            token.column,
                        ));
                    }
                } else {
                    return Err(ParseError::new("Unexpected end of input".to_string(), 0, 0));
                };

                self.expect_token(Token::KeywordAnd, "Expected 'and' after file path")?;
                self.expect_token(Token::KeywordRead, "Expected 'read' after 'and'")?;
                self.expect_token(Token::KeywordContent, "Expected 'content' after 'read'")?;
                self.expect_token(Token::KeywordAs, "Expected 'as' after 'content'")?;

                let variable_name = if let Some(token) = self.tokens.peek().cloned() {
                    if let Token::Identifier(name) = &token.token {
                        self.tokens.next(); // Consume the identifier
                        name.clone()
                    } else if let Token::KeywordContent = &token.token {
                        // Special case for "content" as an identifier
                        self.tokens.next(); // Consume the "content" keyword
                        "content".to_string()
                    } else {
                        return Err(ParseError::new(
                            format!(
                                "Expected identifier for variable name, found {:?}",
                                token.token
                            ),
                            token.line,
                            token.column,
                        ));
                    }
                } else {
                    return Err(ParseError::new("Unexpected end of input".to_string(), 0, 0));
                };

                return Ok(Statement::ReadFileStatement {
                    path: path_expr,
                    variable_name,
                    line: open_token.line,
                    column: open_token.column,
                });
            }
        }

        let path = self.parse_expression()?;

        self.expect_token(Token::KeywordAs, "Expected 'as' after file path")?;

        let variable_name = if let Some(token) = self.tokens.peek().cloned() {
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
            line: open_token.line,
            column: open_token.column,
        })
    }

    fn parse_open_file_read_statement(&mut self) -> Result<Statement, ParseError> {
        let open_token = self.tokens.next().unwrap(); // Consume "open"

        self.expect_token(Token::KeywordFile, "Expected 'file' after 'open'")?;
        self.expect_token(Token::KeywordAt, "Expected 'at' after 'file'")?;

        let path_expr = if let Some(token) = self.tokens.peek().cloned() {
            if let Token::StringLiteral(path_str) = &token.token {
                let token_clone = token;
                self.tokens.next(); // Consume the string literal
                Expression::Literal(
                    Literal::String(path_str.clone()),
                    token_clone.line,
                    token_clone.column,
                )
            } else {
                return Err(ParseError::new(
                    format!(
                        "Expected string literal for file path, found {:?}",
                        token.token
                    ),
                    token.line,
                    token.column,
                ));
            }
        } else {
            return Err(ParseError::new("Unexpected end of input".to_string(), 0, 0));
        };

        self.expect_token(Token::KeywordAnd, "Expected 'and' after file path")?;
        self.expect_token(Token::KeywordRead, "Expected 'read' after 'and'")?;
        self.expect_token(Token::KeywordContent, "Expected 'content' after 'read'")?;
        self.expect_token(Token::KeywordAs, "Expected 'as' after 'content'")?;

        let variable_name = if let Some(token) = self.tokens.peek().cloned() {
            if let Token::Identifier(name) = &token.token {
                self.tokens.next(); // Consume the identifier
                name.clone()
            } else if let Token::KeywordContent = &token.token {
                self.tokens.next(); // Consume the "content" keyword
                "content".to_string()
            } else {
                return Err(ParseError::new(
                    format!(
                        "Expected identifier for variable name, found {:?}",
                        token.token
                    ),
                    token.line,
                    token.column,
                ));
            }
        } else {
            return Err(ParseError::new("Unexpected end of input".to_string(), 0, 0));
        };

        Ok(Statement::ReadFileStatement {
            path: path_expr,
            variable_name,
            line: open_token.line,
            column: open_token.column,
        })
    }

    fn parse_wait_for_statement(&mut self) -> Result<Statement, ParseError> {
        let wait_token_pos = self.tokens.peek().map_or(
            &TokenWithPosition {
                token: Token::KeywordWait,
                line: 0,
                column: 0,
                length: 0,
            },
            |v| v,
        );

        self.tokens.next(); // Consume "wait"
        self.expect_token(Token::KeywordFor, "Expected 'for' after 'wait'")?;

        let inner = Box::new(self.parse_statement()?);

        Ok(Statement::WaitForStatement {
            inner,
            line: wait_token_pos.line,
            column: wait_token_pos.column,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression()?;

        let default_token = TokenWithPosition {
            token: Token::Identifier("expression".to_string()),
            line: 0,
            column: 0,
            length: 0,
        };
        let token_pos = self.tokens.peek().map_or(&default_token, |v| v);
        Ok(Statement::ExpressionStatement {
            expression: expr,
            line: token_pos.line,
            column: token_pos.column,
        })
    }
}
