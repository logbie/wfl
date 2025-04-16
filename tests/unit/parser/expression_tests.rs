#[cfg(test)]
mod parser_expression_tests {
    // Importing required modules
    use crate::parser::{Parser, Expression, ExpressionKind};
    use crate::lexer::Lexer;
    
    // Helper function to parse an expression
    fn parse_expression(input: &str) -> Expression {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_expression()
    }
    
    #[test]
    fn test_literal_expressions() {
        // Test number literals
        let expr = parse_expression("42");
        match expr.kind {
            ExpressionKind::Literal(value) => {
                assert_eq!(value.to_string(), "42");
            },
            _ => panic!("Expected Literal expression, got {:?}", expr.kind),
        }
        
        // Test string literals
        let expr = parse_expression(r#""hello""#);
        match expr.kind {
            ExpressionKind::Literal(value) => {
                assert_eq!(value.to_string(), "hello");
            },
            _ => panic!("Expected Literal expression, got {:?}", expr.kind),
        }
        
        // Test boolean literals
        let expr = parse_expression("true");
        match expr.kind {
            ExpressionKind::Literal(value) => {
                assert_eq!(value.to_string(), "true");
            },
            _ => panic!("Expected Literal expression, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_variable_expressions() {
        let expr = parse_expression("myVariable");
        match expr.kind {
            ExpressionKind::Variable(name) => {
                assert_eq!(name, "myVariable");
            },
            _ => panic!("Expected Variable expression, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_binary_expressions() {
        // Test addition
        let expr = parse_expression("5 + 3");
        match expr.kind {
            ExpressionKind::Binary { left, operator, right } => {
                assert_eq!(operator.to_string(), "+");
                
                match left.kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "5"),
                    _ => panic!("Expected Literal expression, got {:?}", left.kind),
                }
                
                match right.kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "3"),
                    _ => panic!("Expected Literal expression, got {:?}", right.kind),
                }
            },
            _ => panic!("Expected Binary expression, got {:?}", expr.kind),
        }
        
        // Test other binary operations
        let operations = [
            ("10 - 2", "-"),
            ("7 * 4", "*"),
            ("20 / 5", "/"),
            ("x > y", ">"),
            ("a < b", "<"),
            ("p >= q", ">="),
            ("m <= n", "<="),
            ("foo == bar", "=="),
            ("x != y", "!="),
            ("true and false", "and"),
            ("a or b", "or"),
        ];
        
        for (input, op) in operations.iter() {
            let expr = parse_expression(input);
            match expr.kind {
                ExpressionKind::Binary { operator, .. } => {
                    assert_eq!(operator.to_string(), *op);
                },
                _ => panic!("Expected Binary expression for {}, got {:?}", input, expr.kind),
            }
        }
    }
    
    #[test]
    fn test_unary_expressions() {
        // Test negation
        let expr = parse_expression("-42");
        match expr.kind {
            ExpressionKind::Unary { operator, right } => {
                assert_eq!(operator.to_string(), "-");
                
                match right.kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "42"),
                    _ => panic!("Expected Literal expression, got {:?}", right.kind),
                }
            },
            _ => panic!("Expected Unary expression, got {:?}", expr.kind),
        }
        
        // Test logical not
        let expr = parse_expression("not true");
        match expr.kind {
            ExpressionKind::Unary { operator, right } => {
                assert_eq!(operator.to_string(), "not");
                
                match right.kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "true"),
                    _ => panic!("Expected Literal expression, got {:?}", right.kind),
                }
            },
            _ => panic!("Expected Unary expression, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_grouping_expressions() {
        let expr = parse_expression("(2 + 3)");
        match expr.kind {
            ExpressionKind::Grouping(inner) => {
                match inner.kind {
                    ExpressionKind::Binary { operator, .. } => {
                        assert_eq!(operator.to_string(), "+");
                    },
                    _ => panic!("Expected Binary expression inside Grouping, got {:?}", inner.kind),
                }
            },
            _ => panic!("Expected Grouping expression, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_complex_expressions() {
        // Test a more complex expression combining multiple operations
        let expr = parse_expression("(5 + 3) * 2 - 4 / (1 + 1)");
        
        // The exact assertions will depend on the precedence rules and AST structure
        // This is a basic check that parsing succeeds without errors
        assert!(matches!(expr.kind, ExpressionKind::Binary { .. }));
    }
    
    // Add more tests as needed
} 