use super::*;
use crate::lexer::lex_wfl_with_positions;

#[test]
fn test_parse_variable_declaration() {
    let input = "store greeting as \"Hello, World!\"";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);

    let result = parser.parse_statement();
    assert!(result.is_ok());

    if let Ok(Statement::VariableDeclaration { name, value, .. }) = result {
        assert_eq!(name, "greeting");
        if let Expression::Literal(Literal::String(s), ..) = value {
            assert_eq!(s, "Hello, World!");
        } else {
            panic!("Expected string literal");
        }
    } else {
        panic!("Expected variable declaration");
    }
}

#[test]
fn test_parse_if_statement() {
    let input = "check if x is equal to 10:\n  display \"x is 10\"\notherwise:\n  display \"x is not 10\"\nend check";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);

    let result = parser.parse_statement();
    assert!(result.is_ok());

    if let Ok(Statement::IfStatement {
        condition,
        then_block,
        else_block,
        ..
    }) = result
    {
        if let Expression::BinaryOperation {
            left,
            operator,
            right,
            ..
        } = condition
        {
            if let Expression::Variable(name, ..) = *left {
                assert_eq!(name, "x");
            } else {
                panic!("Expected variable in condition");
            }

            assert_eq!(operator, Operator::Equals);

            if let Expression::Literal(Literal::Integer(n), ..) = *right {
                assert_eq!(n, 10);
            } else {
                panic!("Expected integer literal in condition");
            }
        } else {
            panic!("Expected binary operation in condition");
        }

        assert_eq!(then_block.len(), 1);
        if let Statement::DisplayStatement { value, .. } = &then_block[0] {
            if let Expression::Literal(Literal::String(s), ..) = value {
                assert_eq!(s, "x is 10");
            } else {
                panic!("Expected string literal in then block");
            }
        } else {
            panic!("Expected display statement in then block");
        }

        assert!(else_block.is_some());
        let else_stmts = else_block.as_ref().unwrap();
        assert_eq!(else_stmts.len(), 1);
        if let Statement::DisplayStatement { value, .. } = &else_stmts[0] {
            if let Expression::Literal(Literal::String(s), ..) = value {
                assert_eq!(s, "x is not 10");
            } else {
                panic!("Expected string literal in else block");
            }
        } else {
            panic!("Expected display statement in else block");
        }
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn test_parse_expression() {
    let input = "5 plus 3 times 2";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);

    let result = parser.parse_expression();
    assert!(result.is_ok());

    if let Ok(Expression::BinaryOperation {
        left,
        operator,
        right,
        ..
    }) = result
    {
        if let Expression::Literal(Literal::Integer(n), ..) = *left {
            assert_eq!(n, 5);
        } else {
            panic!("Expected integer literal");
        }

        assert_eq!(operator, Operator::Plus);

        if let Expression::BinaryOperation {
            left,
            operator,
            right,
            ..
        } = *right
        {
            if let Expression::Literal(Literal::Integer(n), ..) = *left {
                assert_eq!(n, 3);
            } else {
                panic!("Expected integer literal");
            }

            assert_eq!(operator, Operator::Multiply);

            if let Expression::Literal(Literal::Integer(n), ..) = *right {
                assert_eq!(n, 2);
            } else {
                panic!("Expected integer literal");
            }
        } else {
            panic!("Expected binary operation");
        }
    } else {
        panic!("Expected binary operation");
    }
}

#[test]
fn test_parse_wait_for_open_file() {
    {
        let input = r#"open file at "data.txt" and read content as content"#;
        let tokens = lex_wfl_with_positions(input);
        let mut parser = Parser::new(&tokens);

        println!("Testing open file statement:");
        for (i, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, token);
        }

        let result = parser.parse_statement();
        if let Err(ref e) = result {
            println!("Parse error for open file: {:?}", e);
        } else {
            println!("Successfully parsed open file statement");
        }
        assert!(result.is_ok());
    }
    
    // Test the new syntax: "open file at "path" as variable"
    {
        let input = r#"open file at "nexus.log" as logHandle"#;
        let tokens = lex_wfl_with_positions(input);
        let mut parser = Parser::new(&tokens);

        println!("\nTesting new open file syntax:");
        for (i, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, token);
        }

        let result = parser.parse_statement();
        if let Err(ref e) = result {
            println!("Parse error for new open file syntax: {:?}", e);
        } else {
            println!("Successfully parsed new open file syntax");
        }
        assert!(result.is_ok());

        if let Ok(Statement::OpenFileStatement { path, variable_name, .. }) = result {
            if let Expression::Literal(Literal::String(s), ..) = path {
                assert_eq!(s, "nexus.log");
            } else {
                panic!("Expected string literal for path");
            }
            assert_eq!(variable_name, "logHandle");
        } else {
            panic!("Expected OpenFileStatement");
        }
    }

    {
        let input = r#"wait for open file at "data.txt" and read content as content"#;
        let tokens = lex_wfl_with_positions(input);
        let mut parser = Parser::new(&tokens);

        println!("\nTesting wait for statement:");
        for (i, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, token);
        }

        let result = parser.parse_statement();
        if let Err(ref e) = result {
            println!("Parse error for wait for: {:?}", e);
        } else {
            println!("Successfully parsed wait for statement");
        }
        assert!(result.is_ok());

        if let Ok(Statement::WaitForStatement { inner, .. }) = result {
            if let Statement::ReadFileStatement {
                path,
                variable_name,
                ..
            } = *inner
            {
                if let Expression::Literal(Literal::String(s), ..) = path {
                    assert_eq!(s, "data.txt");
                } else {
                    panic!("Expected string literal for path");
                }
                assert_eq!(variable_name, "content");
            } else {
                panic!("Expected ReadFileStatement");
            }
        } else {
            panic!("Expected WaitForStatement");
        }
    }
}

#[test]
fn test_missing_as_in_store_statement() {
    let input = "store greeting 42";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);

    let result = parser.parse_statement();
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.message.contains("Expected 'as' after variable name"));
        assert!(error.message.contains("42"));
    }
}

#[test]
fn test_missing_as_in_create_statement() {
    let input = "create user \"John\"";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);

    let result = parser.parse_statement();
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.message.contains("Expected 'as' after variable name"));
        assert!(error.message.contains("StringLiteral"));
    }
}

#[test]
fn test_missing_to_in_change_statement() {
    let input = "change counter 10";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);

    let result = parser.parse_statement();
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.message.contains("Expected 'to' after identifier(s)"));
        assert!(error.message.contains("10"));
    }
}

#[test]
fn test_valid_store_statements() {
    let input = "store x as 1";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_ok());

    let input = "store first name as \"Alice\"";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_store_without_variable_name() {
    let input = "store";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(
            e[0].message.contains("Expected variable name"),
            "Got error: {}",
            e[0]
        );
    }
}

#[test]
fn test_store_with_incomplete_statement() {
    let input = "store a";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(
            e[0].message.contains("Expected 'as'"),
            "Got error: {}",
            e[0]
        );
    }
}

#[test]
fn test_store_with_missing_as() {
    let input = "store a a";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(
            e[0].message.contains("Expected 'as'"),
            "Got error: {}",
            e[0]
        );
    }
}

#[test]
fn test_store_with_number_as_variable_name() {
    let input = "store 1 as 1";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(
            e[0].message
                .contains("Cannot use a number as a variable name"),
            "Got error: {}",
            e[0]
        );
    }
}

#[test]
fn test_store_with_number_as_variable_name_without_as() {
    let input = "store 1 b";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(
            e[0].message
                .contains("Cannot use a number as a variable name"),
            "Got error: {}",
            e[0]
        );
    }
}

#[test]
fn test_store_with_keyword_as_variable_name() {
    let input = "store if as 1";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let result = parser.parse();
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(
            e[0].message.contains("Cannot use keyword"),
            "Got error: {}",
            e[0]
        );
    }
}
