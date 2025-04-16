#[cfg(test)]
mod parser_statement_tests {
    // Importing required modules
    use crate::parser::{Parser, Statement, StatementKind, Expression, ExpressionKind};
    use crate::lexer::Lexer;
    
    // Helper function to parse a statement
    fn parse_statement(input: &str) -> Statement {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_statement()
    }
    
    #[test]
    fn test_variable_declaration() {
        let stmt = parse_statement("define variable x = 42");
        
        match &stmt.kind {
            StatementKind::VarDeclaration { name, initializer } => {
                assert_eq!(name, "x");
                
                match &initializer.kind {
                    ExpressionKind::Literal(value) => {
                        assert_eq!(value.to_string(), "42");
                    },
                    _ => panic!("Expected Literal expression, got {:?}", initializer.kind),
                }
            },
            _ => panic!("Expected VarDeclaration, got {:?}", stmt.kind),
        }
        
        // Test with string initializer
        let stmt = parse_statement(r#"define variable greeting = "hello""#);
        
        match &stmt.kind {
            StatementKind::VarDeclaration { name, initializer } => {
                assert_eq!(name, "greeting");
                
                match &initializer.kind {
                    ExpressionKind::Literal(value) => {
                        assert_eq!(value.to_string(), "hello");
                    },
                    _ => panic!("Expected Literal expression, got {:?}", initializer.kind),
                }
            },
            _ => panic!("Expected VarDeclaration, got {:?}", stmt.kind),
        }
        
        // Test with expression initializer
        let stmt = parse_statement("define variable sum = a + b");
        
        match &stmt.kind {
            StatementKind::VarDeclaration { name, initializer } => {
                assert_eq!(name, "sum");
                
                match &initializer.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", initializer.kind),
                }
            },
            _ => panic!("Expected VarDeclaration, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_assignment() {
        let stmt = parse_statement("x = 42");
        
        match &stmt.kind {
            StatementKind::Assignment { target, value } => {
                match &target.kind {
                    ExpressionKind::Variable(name) => {
                        assert_eq!(name, "x");
                    },
                    _ => panic!("Expected Variable expression, got {:?}", target.kind),
                }
                
                match &value.kind {
                    ExpressionKind::Literal(val) => {
                        assert_eq!(val.to_string(), "42");
                    },
                    _ => panic!("Expected Literal expression, got {:?}", value.kind),
                }
            },
            _ => panic!("Expected Assignment, got {:?}", stmt.kind),
        }
        
        // Test with expression value
        let stmt = parse_statement("result = a + b * c");
        
        match &stmt.kind {
            StatementKind::Assignment { target, value } => {
                match &target.kind {
                    ExpressionKind::Variable(name) => {
                        assert_eq!(name, "result");
                    },
                    _ => panic!("Expected Variable expression, got {:?}", target.kind),
                }
                
                match &value.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", value.kind),
                }
            },
            _ => panic!("Expected Assignment, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_if_statement() {
        let stmt = parse_statement("if x > 10\nx = x - 1\nend if");
        
        match &stmt.kind {
            StatementKind::If { condition, then_branch, else_branch } => {
                // Check condition
                match &condition.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", condition.kind),
                }
                
                // Check then branch
                assert_eq!(then_branch.len(), 1);
                match &then_branch[0].kind {
                    StatementKind::Assignment { .. } => {},
                    _ => panic!("Expected Assignment, got {:?}", then_branch[0].kind),
                }
                
                // Check else branch (should be None in this case)
                assert!(else_branch.is_none());
            },
            _ => panic!("Expected If statement, got {:?}", stmt.kind),
        }
        
        // Test if-else statement
        let stmt = parse_statement("if x > 10\nx = x - 1\nelse\nx = x + 1\nend if");
        
        match &stmt.kind {
            StatementKind::If { condition: _, then_branch, else_branch } => {
                // Check then branch
                assert_eq!(then_branch.len(), 1);
                
                // Check else branch (should be Some in this case)
                assert!(else_branch.is_some());
                
                let else_stmts = else_branch.as_ref().unwrap();
                assert_eq!(else_stmts.len(), 1);
                match &else_stmts[0].kind {
                    StatementKind::Assignment { .. } => {},
                    _ => panic!("Expected Assignment, got {:?}", else_stmts[0].kind),
                }
            },
            _ => panic!("Expected If statement, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_while_loop() {
        let stmt = parse_statement("while counter < 10\ncounter = counter + 1\nend while");
        
        match &stmt.kind {
            StatementKind::While { condition, body } => {
                // Check condition
                match &condition.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", condition.kind),
                }
                
                // Check body
                assert_eq!(body.len(), 1);
                match &body[0].kind {
                    StatementKind::Assignment { .. } => {},
                    _ => panic!("Expected Assignment, got {:?}", body[0].kind),
                }
            },
            _ => panic!("Expected While statement, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_check_statement() {
        let stmt = parse_statement("check x > 0:\nx = 0\nend check");
        
        match &stmt.kind {
            StatementKind::Check { condition, body } => {
                // Check condition
                match &condition.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", condition.kind),
                }
                
                // Check body
                assert_eq!(body.len(), 1);
                match &body[0].kind {
                    StatementKind::Assignment { .. } => {},
                    _ => panic!("Expected Assignment, got {:?}", body[0].kind),
                }
            },
            _ => panic!("Expected Check statement, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_expression_statement() {
        let stmt = parse_statement("print(\"Hello, World!\")");
        
        match &stmt.kind {
            StatementKind::Expression(expr) => {
                match &expr.kind {
                    ExpressionKind::Call { .. } => {},
                    _ => panic!("Expected Call expression, got {:?}", expr.kind),
                }
            },
            _ => panic!("Expected Expression statement, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_block_statement() {
        let stmt = parse_statement("block\nx = 1\ny = 2\nend block");
        
        match &stmt.kind {
            StatementKind::Block(statements) => {
                assert_eq!(statements.len(), 2);
                
                // Check first statement
                match &statements[0].kind {
                    StatementKind::Assignment { .. } => {},
                    _ => panic!("Expected Assignment, got {:?}", statements[0].kind),
                }
                
                // Check second statement
                match &statements[1].kind {
                    StatementKind::Assignment { .. } => {},
                    _ => panic!("Expected Assignment, got {:?}", statements[1].kind),
                }
            },
            _ => panic!("Expected Block statement, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_return_statement() {
        let stmt = parse_statement("return 42");
        
        match &stmt.kind {
            StatementKind::Return(expr) => {
                match &expr.kind {
                    ExpressionKind::Literal(value) => {
                        assert_eq!(value.to_string(), "42");
                    },
                    _ => panic!("Expected Literal expression, got {:?}", expr.kind),
                }
            },
            _ => panic!("Expected Return statement, got {:?}", stmt.kind),
        }
        
        // Test return with expression
        let stmt = parse_statement("return x + y");
        
        match &stmt.kind {
            StatementKind::Return(expr) => {
                match &expr.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", expr.kind),
                }
            },
            _ => panic!("Expected Return statement, got {:?}", stmt.kind),
        }
    }
} 