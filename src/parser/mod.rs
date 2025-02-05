use chumsky::{prelude::*, Parser};
use crate::ast::*;
use crate::lexer::{Token, TokenType};

/// Parser error type
#[derive(Debug, Clone)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}

/// Parser result type
pub type ParseResult<T> = Result<T, Vec<ParseError>>;

/// Main parser struct
pub struct WflParser {
    // Configuration options can be added here
}

impl WflParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a sequence of tokens into an AST
    pub fn parse(&self, tokens: Vec<Token>) -> ParseResult<Program> {
        let parser = program();
        parser.parse(tokens)
            .map_err(|errors| {
                errors.into_iter()
                    .map(|e| ParseError {
                        span: Span {
                            start: e.span().start,
                            end: e.span().end,
                            line: 0, // TODO: Calculate from token
                            column: 0,
                        },
                        message: e.to_string(),
                    })
                    .collect()
            })
    }
}

/// Convert token span to AST span
fn token_span(token: &Token) -> Span {
    Span {
        start: 0, // TODO: Track actual positions in lexer
        end: 0,
        line: token.line,
        column: token.column,
    }
}

/// Create a node with span information
fn spanned<T>(span: Span, node: T) -> Node<T> {
    Node { span, node }
}

/// Parse a literal value
fn literal() -> impl Parser<Token, Node<Expression>, Error = Simple<Token>> {
    filter_map(|span, token: Token| {
        let literal = match token.token_type {
            TokenType::NumberLiteral => {
                Some(Literal::Number(token.value.parse().unwrap_or(0.0)))
            }
            TokenType::StringLiteral => {
                Some(Literal::Text(token.value))
            }
            TokenType::TruthLiteral => {
                let value = match token.value.to_lowercase().as_str() {
                    "yes" | "true" => true,
                    "no" | "false" => false,
                    _ => false,
                };
                Some(Literal::Truth(value))
            }
            TokenType::Nothing => Some(Literal::Nothing),
            TokenType::Missing => Some(Literal::Missing),
            TokenType::Undefined => Some(Literal::Undefined),
            TokenType::Empty => Some(Literal::Empty),
            _ => None,
        };
        
        literal.map(|lit| {
            spanned(
                token_span(&span),
                Expression::Literal(lit)
            )
        })
    })
}

/// Parse a type expression
fn type_expr() -> impl Parser<Token, Type, Error = Simple<Token>> {
    recursive(|type_expr| {
        let simple_type = select! {
            TokenType::Number => Type::Number,
            TokenType::Text => Type::Text,
            TokenType::Truth => Type::Truth,
            TokenType::Any => Type::Any,
        };

        let generic_type = just(TokenType::Identifier)
            .map(|token| Type::Generic(token.value));

        let list_type = just(TokenType::List)
            .ignore_then(just(TokenType::Of))
            .ignore_then(type_expr.clone())
            .map(|elem_type| Type::List(Box::new(elem_type)));

        let map_type = just(TokenType::Map)
            .ignore_then(just(TokenType::From))
            .ignore_then(type_expr.clone())
            .then_ignore(just(TokenType::To))
            .then(type_expr.clone())
            .map(|(key_type, value_type)| {
                Type::Map(Box::new(key_type), Box::new(value_type))
            });

        let set_type = just(TokenType::Set)
            .ignore_then(just(TokenType::Of))
            .ignore_then(type_expr.clone())
            .map(|elem_type| Type::Set(Box::new(elem_type)));

        let record_type = just(TokenType::Record)
            .ignore_then(just(TokenType::LeftBrace))
            .ignore_then(
                just(TokenType::Identifier)
                    .then_ignore(just(TokenType::Colon))
                    .then(type_expr.clone())
                    .separated_by(just(TokenType::Comma))
            )
            .then_ignore(just(TokenType::RightBrace))
            .map(|fields| {
                Type::Record(
                    fields
                        .into_iter()
                        .map(|(name, typ)| (name.value, typ))
                        .collect()
                )
            });

        let action_type = just(TokenType::Action)
            .ignore_then(just(TokenType::LeftParen))
            .ignore_then(
                type_expr
                    .clone()
                    .separated_by(just(TokenType::Comma))
            )
            .then_ignore(just(TokenType::RightParen))
            .then_ignore(just(TokenType::Give))
            .then_ignore(just(TokenType::Back))
            .then(type_expr)
            .map(|(params, ret)| Type::Action(params, Box::new(ret)));

        choice((
            simple_type,
            generic_type,
            list_type,
            map_type,
            set_type,
            record_type,
            action_type,
        ))
    })
}

/// Parse an expression
fn expression() -> impl Parser<Token, Node<Expression>, Error = Simple<Token>> {
    recursive(|expr| {
        let atom = literal()
            .or(just(TokenType::Identifier).map_with_span(|token, span| {
                spanned(
                    token_span(&span),
                    Expression::Variable(token.value)
                )
            }));

        let list = just(TokenType::LeftBracket)
            .ignore_then(expr.clone().separated_by(just(TokenType::Comma)))
            .then_ignore(just(TokenType::RightBracket))
            .map_with_span(|items, span| {
                spanned(token_span(&span), Expression::List(items))
            });

        let map = just(TokenType::LeftBrace)
            .ignore_then(
                expr.clone()
                    .then_ignore(just(TokenType::Colon))
                    .then(expr.clone())
                    .separated_by(just(TokenType::Comma))
            )
            .then_ignore(just(TokenType::RightBrace))
            .map_with_span(|pairs, span| {
                spanned(token_span(&span), Expression::Map(pairs))
            });

        let set = just(TokenType::Set)
            .ignore_then(just(TokenType::LeftBrace))
            .ignore_then(expr.clone().separated_by(just(TokenType::Comma)))
            .then_ignore(just(TokenType::RightBrace))
            .map_with_span(|items, span| {
                spanned(token_span(&span), Expression::Set(items))
            });

        let record = just(TokenType::Record)
            .ignore_then(just(TokenType::LeftBrace))
            .ignore_then(
                just(TokenType::Identifier)
                    .then_ignore(just(TokenType::Is))
                    .then(expr.clone())
                    .separated_by(just(TokenType::Comma))
            )
            .then_ignore(just(TokenType::RightBrace))
            .map_with_span(|fields, span| {
                spanned(
                    token_span(&span),
                    Expression::Record(
                        fields
                            .into_iter()
                            .map(|(name, value)| (name.value, value))
                            .collect()
                    )
                )
            });

        let call = atom
            .clone()
            .then(
                expr.clone()
                    .or(
                        just(TokenType::Identifier)
                            .then_ignore(just(TokenType::Is))
                            .then(expr.clone())
                            .map(|(name, value)| {
                                (name.value, value)
                            })
                    )
                    .repeated()
            )
            .map_with_span(|(func, args), span| {
                let (positional, named): (Vec<_>, Vec<_>) = args
                    .into_iter()
                    .partition(|arg| matches!(arg, Node { .. }));
                
                spanned(
                    token_span(&span),
                    Expression::Call {
                        action: Box::new(func),
                        args: positional,
                        named_args: named,
                    }
                )
            });

        let access = atom
            .clone()
            .then(
                just(TokenType::Dot)
                    .ignore_then(just(TokenType::Identifier))
                    .repeated()
            )
            .map_with_span(|(obj, fields), span| {
                fields.into_iter().fold(obj, |acc, field| {
                    spanned(
                        token_span(&span),
                        Expression::Access {
                            object: Box::new(acc),
                            field: field.value,
                        }
                    )
                })
            });

        // Define operator precedence
        let op = choice((
            just(TokenType::Plus).to(Operator::Plus),
            just(TokenType::Minus).to(Operator::Minus),
            just(TokenType::Multiply).to(Operator::Multiply),
            just(TokenType::Divide).to(Operator::Divide),
            just(TokenType::Modulo).to(Operator::Modulo),
            just(TokenType::Equals).to(Operator::Equals),
            just(TokenType::NotEquals).to(Operator::NotEquals),
            just(TokenType::Greater).to(Operator::Greater),
            just(TokenType::Less).to(Operator::Less),
            just(TokenType::GreaterEquals).to(Operator::GreaterEquals),
            just(TokenType::LessEquals).to(Operator::LessEquals),
            just(TokenType::And).to(Operator::And),
            just(TokenType::Or).to(Operator::Or),
        ));

        let unary = just(TokenType::Not)
            .map(|_| Operator::Not)
            .then(atom.clone())
            .map_with_span(|(op, expr), span| {
                spanned(
                    token_span(&span),
                    Expression::UnaryOp {
                        op,
                        expr: Box::new(expr),
                    }
                )
            });

        let binary = unary
            .clone()
            .then(op.then(unary).repeated())
            .map_with_span(|(first, rest), span| {
                rest.into_iter().fold(first, |acc, (op, right)| {
                    spanned(
                        token_span(&span),
                        Expression::BinaryOp {
                            left: Box::new(acc),
                            op,
                            right: Box::new(right),
                        }
                    )
                })
            });

        choice((
            binary,
            unary,
            call,
            access,
            list,
            map,
            set,
            record,
            atom,
        ))
    })
}

/// Parse a pattern
fn pattern() -> impl Parser<Token, Node<Pattern>, Error = Simple<Token>> {
    recursive(|pattern| {
        let literal_pattern = literal().map_with_span(|expr, span| {
            if let Expression::Literal(lit) = expr.node {
                spanned(token_span(&span), Pattern::Literal(lit))
            } else {
                unreachable!()
            }
        });

        let variable_pattern = just(TokenType::Identifier)
            .map_with_span(|token, span| {
                spanned(
                    token_span(&span),
                    Pattern::Variable(token.value)
                )
            });

        let list_pattern = just(TokenType::LeftBracket)
            .ignore_then(pattern.clone().separated_by(just(TokenType::Comma)))
            .then_ignore(just(TokenType::RightBracket))
            .map_with_span(|items, span| {
                spanned(token_span(&span), Pattern::List(items))
            });

        let record_pattern = just(TokenType::LeftBrace)
            .ignore_then(
                just(TokenType::Identifier)
                    .then_ignore(just(TokenType::Colon))
                    .then(pattern.clone())
                    .separated_by(just(TokenType::Comma))
            )
            .then_ignore(just(TokenType::RightBrace))
            .map_with_span(|fields, span| {
                spanned(
                    token_span(&span),
                    Pattern::Record(
                        fields
                            .into_iter()
                            .map(|(name, pat)| (name.value, pat))
                            .collect()
                    )
                )
            });

        let type_pattern = type_expr()
            .then(
                just(TokenType::Where)
                    .ignore_then(expression())
                    .repeated()
            )
            .map_with_span(|(typ, constraints), span| {
                spanned(
                    token_span(&span),
                    Pattern::TypePattern {
                        type_name: typ,
                        constraints,
                    }
                )
            });

        choice((
            literal_pattern,
            variable_pattern,
            list_pattern,
            record_pattern,
            type_pattern,
        ))
    })
}

/// Parse a statement
fn statement() -> impl Parser<Token, Node<Statement>, Error = Simple<Token>> {
    recursive(|stmt| {
        let store = just(TokenType::Store)
            .ignore_then(just(TokenType::Identifier))
            .then_ignore(just(TokenType::As))
            .then(expression())
            .then(
                just(TokenType::With)
                    .ignore_then(just(TokenType::Type))
                    .ignore_then(type_expr())
                    .or_not()
            )
            .map_with_span(|((name, value), type_annotation), span| {
                spanned(
                    token_span(&span),
                    Statement::Store {
                        name: name.value,
                        value,
                        type_annotation,
                    }
                )
            });

        let assign = expression()
            .then_ignore(just(TokenType::Equals))
            .then(expression())
            .map_with_span(|(target, value), span| {
                spanned(
                    token_span(&span),
                    Statement::Assign { target, value }
                )
            });

        let if_stmt = just(TokenType::If)
            .ignore_then(expression())
            .then_ignore(just(TokenType::Colon))
            .then(stmt.clone().repeated())
            .then(
                just(TokenType::Otherwise)
                    .ignore_then(just(TokenType::Colon))
                    .ignore_then(stmt.clone().repeated())
                    .or_not()
            )
            .then_ignore(just(TokenType::End))
            .then_ignore(just(TokenType::If))
            .map_with_span(|((condition, then_block), else_block), span| {
                spanned(
                    token_span(&span),
                    Statement::If {
                        condition,
                        then_block,
                        else_block,
                    }
                )
            });

        let check = just(TokenType::Check)
            .ignore_then(expression())
            .then_ignore(just(TokenType::Colon))
            .then(
                pattern()
                    .then_ignore(just(TokenType::Then))
                    .then(stmt.clone().repeated())
                    .repeated()
            )
            .then(
                just(TokenType::Otherwise)
                    .ignore_then(just(TokenType::Colon))
                    .ignore_then(stmt.clone().repeated())
                    .or_not()
            )
            .then_ignore(just(TokenType::End))
            .then_ignore(just(TokenType::Check))
            .map_with_span(|((value, patterns), else_block), span| {
                spanned(
                    token_span(&span),
                    Statement::Check {
                        value,
                        patterns,
                        else_block,
                    }
                )
            });

        let for_each = just(TokenType::For)
            .ignore_then(just(TokenType::Each))
            .ignore_then(just(TokenType::Identifier))
            .then_ignore(just(TokenType::In))
            .then(expression())
            .then_ignore(just(TokenType::Colon))
            .then(stmt.clone().repeated())
            .then_ignore(just(TokenType::End))
            .then_ignore(just(TokenType::For))
            .map_with_span(|((item, collection), body), span| {
                spanned(
                    token_span(&span),
                    Statement::ForEach {
                        item: item.value,
                        collection,
                        body,
                    }
                )
            });

        let while_stmt = just(TokenType::While)
            .ignore_then(expression())
            .then_ignore(just(TokenType::Colon))
            .then(stmt.clone().repeated())
            .then_ignore(just(TokenType::End))
            .then_ignore(just(TokenType::While))
            .map_with_span(|(condition, body), span| {
                spanned(
                    token_span(&span),
                    Statement::While { condition, body }
                )
            });

        let until = just(TokenType::Until)
            .ignore_then(expression())
            .then_ignore(just(TokenType::Colon))
            .then(stmt.clone().repeated())
            .then_ignore(just(TokenType::End))
            .then_ignore(just(TokenType::Until))
            .map_with_span(|(condition, body), span| {
                spanned(
                    token_span(&span),
                    Statement::Until { condition, body }
                )
            });

        let try_stmt = just(TokenType::Try)
            .ignore_then(stmt.clone().repeated())
            .then(
                just(TokenType::Catch)
                    .ignore_then(pattern())
                    .then_ignore(just(TokenType::Colon))
                    .then(stmt.clone().repeated())
                    .repeated()
            )
            .then(
                just(TokenType::Finally)
                    .ignore_then(just(TokenType::Colon))
                    .ignore_then(stmt.clone().repeated())
                    .or_not()
            )
            .then_ignore(just(TokenType::End))
            .then_ignore(just(TokenType::Try))
            .map_with_span(|((body, catch_blocks), finally_block), span| {
                spanned(
                    token_span(&span),
                    Statement::Try {
                        body,
                        catch_blocks,
                        finally_block,
                    }
                )
            });

        let return_stmt = just(TokenType::Give)
            .ignore_then(just(TokenType::Back))
            .ignore_then(expression())
            .map_with_span(|value, span| {
                spanned(token_span(&span), Statement::Return(value))
            });

        let break_stmt = just(TokenType::Break)
            .ignore_then(just(TokenType::Identifier).or_not())
            .map_with_span(|label, span| {
                spanned(
                    token_span(&span),
                    Statement::Break(label.map(|t| t.value))
                )
            });

        let continue_stmt = just(TokenType::Continue)
            .ignore_then(just(TokenType::Identifier).or_not())
            .map_with_span(|label, span| {
                spanned(
                    token_span(&span),
                    Statement::Continue(label.map(|t| t.value))
                )
            });

        let expr_stmt = expression().map_with_span(|expr, span| {
            spanned(token_span(&span), Statement::Expression(expr))
        });

        choice((
            store,
            assign,
            if_stmt,
            check,
            for_each,
            while_stmt,
            until,
            try_stmt,
            return_stmt,
            break_stmt,
            continue_stmt,
            expr_stmt,
        ))
    })
}

/// Parse a parameter list
fn parameters() -> impl Parser<Token, Vec<Parameter>, Error = Simple<Token>> {
    just(TokenType::LeftParen)
        .ignore_then(
            just(TokenType::Identifier)
                .then(
                    just(TokenType::With)
                        .ignore_then(just(TokenType::Type))
                        .ignore_then(type_expr())
                        .or_not()
                )
                .then(
                    just(TokenType::With)
                        .ignore_then(just(TokenType::Default))
                        .ignore_then(expression())
                        .or_not()
                )
                .map(|((name, type_annotation), default_value)| Parameter {
                    name: name.value,
                    type_annotation,
                    default_value: default_value.map(|e| Node {
                        span: token_span(&e),
                        node: e.node,
                    }),
                })
                .separated_by(just(TokenType::Comma))
        )
        .then_ignore(just(TokenType::RightParen))
}

/// Parse a declaration
fn declaration() -> impl Parser<Token, Node<Declaration>, Error = Simple<Token>> {
    let visibility = choice((
        just(TokenType::Public).to(Visibility::Public),
        just(TokenType::Private).to(Visibility::Private),
        just(TokenType::Protected).to(Visibility::Protected),
    )).or_not().map(|v| v.unwrap_or(Visibility::Private));

    let generic_params = just(TokenType::Of)
        .ignore_then(just(TokenType::Type))
        .ignore_then(just(TokenType::Identifier))
        .map(|t| t.value)
        .separated_by(just(TokenType::Comma))
        .or_not()
        .map(|v| v.unwrap_or_default());

    let action = visibility
        .then(just(TokenType::Define))
        .ignore_then(just(TokenType::Action))
        .ignore_then(just(TokenType::Called))
        .ignore_then(just(TokenType::Identifier))
        .then(generic_params.clone())
        .then(parameters())
        .then(
            just(TokenType::Gives)
                .ignore_then(just(TokenType::Back))
                .ignore_then(type_expr())
                .or_not()
        )
        .then_ignore(just(TokenType::Colon))
        .then(statement().repeated())
        .then_ignore(just(TokenType::End))
        .then_ignore(just(TokenType::Action))
        .map_with_span(
            |((((((visibility, name), generic_params), params), return_type), body), span)| {
                spanned(
                    token_span(&span),
                    Declaration::Action {
                        name: name.value,
                        visibility,
                        generic_params,
                        params,
                        return_type,
                        body,
                    }
                )
            }
        );

    let container = visibility
        .then(just(TokenType::Create))
        .ignore_then(just(TokenType::Container))
        .ignore_then(just(TokenType::Called))
        .ignore_then(just(TokenType::Identifier))
        .then(generic_params.clone())
        .then(
            just(TokenType::From)
                .ignore_then(type_expr())
                .or_not()
        )
        .then(
            just(TokenType::Implements)
                .ignore_then(type_expr())
                .separated_by(just(TokenType::Comma))
                .or_not()
                .map(|v| v.unwrap_or_default())
        )
        .then_ignore(just(TokenType::Colon))
        .then(
            visibility
                .then(just(TokenType::Identifier))
                .then_ignore(just(TokenType::Is))
                .then(type_expr())
                .map(|((vis, name), typ)| (name.value, typ, vis))
                .repeated()
        )
        .then(declaration().repeated())
        .then_ignore(just(TokenType::End))
        .then_ignore(just(TokenType::Container))
        .map_with_span(
            |(((((((visibility, name), generic_params), extends), implements), fields), methods), span)| {
                spanned(
                    token_span(&span),
                    Declaration::Container {
                        name: name.value,
                        visibility,
                        generic_params,
                        extends,
                        implements,
                        fields,
                        methods,
                    }
                )
            }
        );

    let interface = visibility
        .then(just(TokenType::Define))
        .ignore_then(just(TokenType::Interface))
        .ignore_then(just(TokenType::Called))
        .ignore_then(just(TokenType::Identifier))
        .then(generic_params)
        .then(
            just(TokenType::From)
                .ignore_then(type_expr())
                .separated_by(just(TokenType::Comma))
                .or_not()
                .map(|v| v.unwrap_or_default())
        )
        .then_ignore(just(TokenType::Colon))
        .then(
            just(TokenType::Action)
                .ignore_then(just(TokenType::Called))
                .ignore_then(just(TokenType::Identifier))
                .then(parameters())
                .then(
                    just(TokenType::Gives)
                        .ignore_then(just(TokenType::Back))
                        .ignore_then(type_expr())
                        .or_not()
                )
                .map(|((name, params), return_type)| {
                    (name.value, params, return_type)
                })
                .repeated()
        )
        .then_ignore(just(TokenType::End))
        .then_ignore(just(TokenType::Interface))
        .map_with_span(
            |((((visibility, name), generic_params), extends), methods), span| {
                spanned(
                    token_span(&span),
                    Declaration::Interface {
                        name: name.value,
                        visibility,
                        generic_params,
                        extends,
                        methods,
                    }
                )
            }
        );

    choice((action, container, interface))
}

/// Parse a complete program
fn program() -> impl Parser<Token, Program, Error = Simple<Token>> {
    declaration()
        .repeated()
        .then_ignore(end())
        .map(|declarations| Program { declarations })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_str(input: &str) -> ParseResult<Program> {
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let parser = WflParser::new();
        parser.parse(tokens)
    }

    #[test]
    fn test_parse_literal() {
        let input = r#"
            define action called test:
                store x as 42
                store msg as "hello"
                store flag as yes
            end action
        "#;
        
        let result = parse_str(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.declarations.len(), 1);
        
        if let Declaration::Action { body, .. } = &program.declarations[0].node {
            assert_eq!(body.len(), 3);
            
            // Check number literal
            if let Statement::Store { value, .. } = &body[0].node {
                if let Expression::Literal(Literal::Number(n)) = &value.node {
                    assert_eq!(*n, 42.0);
                } else {
                    panic!("Expected number literal");
                }
            }
            
            // Check string literal
            if let Statement::Store { value, .. } = &body[1].node {
                if let Expression::Literal(Literal::Text(s)) = &value.node {
                    assert_eq!(s, "hello");
                } else {
                    panic!("Expected string literal");
                }
            }
            
            // Check truth literal
            if let Statement::Store { value, .. } = &body[2].node {
                if let Expression::Literal(Literal::Truth(b)) = &value.node {
                    assert_eq!(*b, true);
                } else {
                    panic!("Expected truth literal");
                }
            }
        } else {
            panic!("Expected action declaration");
        }
    }

    #[test]
    fn test_parse_container() {
        let input = r#"
            create container called Person:
                private name is text
                private age is number
                
                public define action called get_name:
                    give back name
                end action
            end container
        "#;
        
        let result = parse_str(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.declarations.len(), 1);
        
        if let Declaration::Container {
            name,
            visibility,
            fields,
            methods,
            ..
        } = &program.declarations[0].node {
            assert_eq!(name, "Person");
            assert_eq!(visibility, &Visibility::Private); // Default visibility
            
            // Check fields
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].0, "name");
            assert_eq!(fields[0].1, Type::Text);
            assert_eq!(fields[0].2, Visibility::Private);
            
            assert_eq!(fields[1].0, "age");
            assert_eq!(fields[1].1, Type::Number);
            assert_eq!(fields[1].2, Visibility::Private);
            
            // Check method
            assert_eq!(methods.len(), 1);
            if let Declaration::Action {
                name,
                visibility,
                ..
            } = &methods[0].node {
                assert_eq!(name, "get_name");
                assert_eq!(visibility, &Visibility::Public);
            } else {
                panic!("Expected action declaration");
            }
        } else {
            panic!("Expected container declaration");
        }
    }

    #[test]
    fn test_parse_interface() {
        let input = r#"
            define interface called DataStore:
                action called save(data with type any) gives back truth
                action called load(id with type text) gives back any
            end interface
        "#;
        
        let result = parse_str(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.declarations.len(), 1);
        
        if let Declaration::Interface {
            name,
            methods,
            ..
        } = &program.declarations[0].node {
            assert_eq!(name, "DataStore");
            assert_eq!(methods.len(), 2);
            
            // Check save method
            assert_eq!(methods[0].0, "save");
            assert_eq!(methods[0].1.len(), 1);
            assert_eq!(methods[0].1[0].name, "data");
            assert_eq!(methods[0].1[0].type_annotation, Some(Type::Any));
            assert_eq!(methods[0].2, Some(Type::Truth));
            
            // Check load method
            assert_eq!(methods[1].0, "load");
            assert_eq!(methods[1].1.len(), 1);
            assert_eq!(methods[1].1[0].name, "id");
            assert_eq!(methods[1].1[0].type_annotation, Some(Type::Text));
            assert_eq!(methods[1].2, Some(Type::Any));
        } else {
            panic!("Expected interface declaration");
        }
    }

    #[test]
    fn test_parse_expressions() {
        let input = r#"
            define action called test:
                store x as 1 plus 2 times 3
                store y as not (a and b or c)
                store list as [1, 2, 3]
                store map as { "key": 42 }
                store record as record {
                    name is "test",
                    value is 123
                }
            end action
        "#;
        
        let result = parse_str(input);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        if let Declaration::Action { body, .. } = &program.declarations[0].node {
            assert_eq!(body.len(), 5);
            
            // Check arithmetic expression
            if let Statement::Store { value, .. } = &body[0].node {
                if let Expression::BinaryOp { op: op1, left, right } = &value.node {
                    assert!(matches!(op1, Operator::Plus));
                    
                    if let Expression::Literal(Literal::Number(n)) = &left.node {
                        assert_eq!(*n, 1.0);
                    } else {
                        panic!("Expected number literal");
                    }
                    
                    if let Expression::BinaryOp { op: op2, left, right } = &right.node {
                        assert!(matches!(op2, Operator::Multiply));
                        
                        if let Expression::Literal(Literal::Number(n1)) = &left.node {
                            assert_eq!(*n1, 2.0);
                        } else {
                            panic!("Expected number literal");
                        }
                        
                        if let Expression::Literal(Literal::Number(n2)) = &right.node {
                            assert_eq!(*n2, 3.0);
                        } else {
                            panic!("Expected number literal");
                        }
                    } else {
                        panic!("Expected binary operation");
                    }
                } else {
                    panic!("Expected binary operation");
                }
            }
            
            // Check logical expression
            if let Statement::Store { value, .. } = &body[1].node {
                if let Expression::UnaryOp { op, expr } = &value.node {
                    assert!(matches!(op, Operator::Not));
                    
                    if let Expression::BinaryOp { op, .. } = &expr.node {
                        assert!(matches!(op, Operator::Or));
                    } else {
                        panic!("Expected binary operation");
                    }
                } else {
                    panic!("Expected unary operation");
                }
            }
            
            // Check list expression
            if let Statement::Store { value, .. } = &body[2].node {
                if let Expression::List(items) = &value.node {
                    assert_eq!(items.len(), 3);
                } else {
                    panic!("Expected list");
                }
            }
            
            // Check map expression
            if let Statement::Store { value, .. } = &body[3].node {
                if let Expression::Map(pairs) = &value.node {
                    assert_eq!(pairs.len(), 1);
                } else {
                    panic!("Expected map");
                }
            }
            
            // Check record expression
            if let Statement::Store { value, .. } = &body[4].node {
                if let Expression::Record(fields) = &value.node {
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0].0, "name");
                    assert_eq!(fields[1].0, "value");
                } else {
                    panic!("Expected record");
                }
            }
        } else {
            panic!("Expected action declaration");
        }
    }
}