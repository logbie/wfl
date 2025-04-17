use super::*;
use crate::lexer::lex_wfl_with_positions;

#[test]
fn test_parse_variable_declaration() {
    let input = "store greeting as \"Hello, World!\"";
    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);

    let result = parser.parse_statement();
    assert!(result.is_ok());

    if let Ok(Statement::VariableDeclaration { name, value }) = result {
        assert_eq!(name, "greeting");
        if let Expression::Literal(Literal::String(s)) = value {
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
    }) = result
    {
        if let Expression::BinaryOperation {
            left,
            operator,
            right,
        } = condition
        {
            if let Expression::Variable(name) = *left {
                assert_eq!(name, "x");
            } else {
                panic!("Expected variable in condition");
            }

            assert_eq!(operator, Operator::Equals);

            if let Expression::Literal(Literal::Integer(n)) = *right {
                assert_eq!(n, 10);
            } else {
                panic!("Expected integer literal in condition");
            }
        } else {
            panic!("Expected binary operation in condition");
        }

        assert_eq!(then_block.len(), 1);
        if let Statement::DisplayStatement { value } = &then_block[0] {
            if let Expression::Literal(Literal::String(s)) = value {
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
        if let Statement::DisplayStatement { value } = &else_stmts[0] {
            if let Expression::Literal(Literal::String(s)) = value {
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
    }) = result
    {
        if let Expression::Literal(Literal::Integer(n)) = *left {
            assert_eq!(n, 5);
        } else {
            panic!("Expected integer literal");
        }

        assert_eq!(operator, Operator::Plus);

        if let Expression::BinaryOperation {
            left,
            operator,
            right,
        } = *right
        {
            if let Expression::Literal(Literal::Integer(n)) = *left {
                assert_eq!(n, 3);
            } else {
                panic!("Expected integer literal");
            }

            assert_eq!(operator, Operator::Multiply);

            if let Expression::Literal(Literal::Integer(n)) = *right {
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
