#[cfg(test)]
mod parser_collection_tests {
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
    fn test_list_literals() {
        let expr = parse_expression("[1, 2, 3]");
        
        match &expr.kind {
            ExpressionKind::ListLiteral(elements) => {
                assert_eq!(elements.len(), 3);
                
                // Check elements
                match &elements[0].kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "1"),
                    _ => panic!("Expected Literal expression, got {:?}", elements[0].kind),
                }
                
                match &elements[1].kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "2"),
                    _ => panic!("Expected Literal expression, got {:?}", elements[1].kind),
                }
                
                match &elements[2].kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "3"),
                    _ => panic!("Expected Literal expression, got {:?}", elements[2].kind),
                }
            },
            _ => panic!("Expected ListLiteral, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_empty_list() {
        let expr = parse_expression("[]");
        
        match &expr.kind {
            ExpressionKind::ListLiteral(elements) => {
                assert_eq!(elements.len(), 0);
            },
            _ => panic!("Expected ListLiteral, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_list_with_expressions() {
        let expr = parse_expression("[1 + 2, x * y, true and false]");
        
        match &expr.kind {
            ExpressionKind::ListLiteral(elements) => {
                assert_eq!(elements.len(), 3);
                
                // Check elements are expressions
                match &elements[0].kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", elements[0].kind),
                }
                
                match &elements[1].kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", elements[1].kind),
                }
                
                match &elements[2].kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", elements[2].kind),
                }
            },
            _ => panic!("Expected ListLiteral, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_nested_lists() {
        let expr = parse_expression("[[1, 2], [3, 4]]");
        
        match &expr.kind {
            ExpressionKind::ListLiteral(elements) => {
                assert_eq!(elements.len(), 2);
                
                // Check both elements are list literals
                match &elements[0].kind {
                    ExpressionKind::ListLiteral(inner) => assert_eq!(inner.len(), 2),
                    _ => panic!("Expected ListLiteral, got {:?}", elements[0].kind),
                }
                
                match &elements[1].kind {
                    ExpressionKind::ListLiteral(inner) => assert_eq!(inner.len(), 2),
                    _ => panic!("Expected ListLiteral, got {:?}", elements[1].kind),
                }
            },
            _ => panic!("Expected ListLiteral, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_map_literals() {
        let expr = parse_expression("{\"name\": \"John\", \"age\": 30}");
        
        match &expr.kind {
            ExpressionKind::MapLiteral(entries) => {
                assert_eq!(entries.len(), 2);
                
                // Check keys and values
                let entry1 = &entries[0];
                match &entry1.0.kind {
                    ExpressionKind::Literal(key) => assert_eq!(key.to_string(), "name"),
                    _ => panic!("Expected Literal expression for key, got {:?}", entry1.0.kind),
                }
                
                match &entry1.1.kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "John"),
                    _ => panic!("Expected Literal expression for value, got {:?}", entry1.1.kind),
                }
                
                let entry2 = &entries[1];
                match &entry2.0.kind {
                    ExpressionKind::Literal(key) => assert_eq!(key.to_string(), "age"),
                    _ => panic!("Expected Literal expression for key, got {:?}", entry2.0.kind),
                }
                
                match &entry2.1.kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "30"),
                    _ => panic!("Expected Literal expression for value, got {:?}", entry2.1.kind),
                }
            },
            _ => panic!("Expected MapLiteral, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_empty_map() {
        let expr = parse_expression("{}");
        
        match &expr.kind {
            ExpressionKind::MapLiteral(entries) => {
                assert_eq!(entries.len(), 0);
            },
            _ => panic!("Expected MapLiteral, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_map_with_expressions() {
        let expr = parse_expression("{\"sum\": 1 + 2, \"product\": x * y}");
        
        match &expr.kind {
            ExpressionKind::MapLiteral(entries) => {
                assert_eq!(entries.len(), 2);
                
                // Check values are expressions
                let entry1 = &entries[0];
                match &entry1.1.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", entry1.1.kind),
                }
                
                let entry2 = &entries[1];
                match &entry2.1.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", entry2.1.kind),
                }
            },
            _ => panic!("Expected MapLiteral, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_indexing() {
        let expr = parse_expression("myList[0]");
        
        match &expr.kind {
            ExpressionKind::Index { target, index } => {
                // Check target
                match &target.kind {
                    ExpressionKind::Variable(name) => assert_eq!(name, "myList"),
                    _ => panic!("Expected Variable expression, got {:?}", target.kind),
                }
                
                // Check index
                match &index.kind {
                    ExpressionKind::Literal(value) => assert_eq!(value.to_string(), "0"),
                    _ => panic!("Expected Literal expression, got {:?}", index.kind),
                }
            },
            _ => panic!("Expected Index expression, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_indexing_with_expressions() {
        let expr = parse_expression("myList[i + 1]");
        
        match &expr.kind {
            ExpressionKind::Index { target, index } => {
                // Check target
                match &target.kind {
                    ExpressionKind::Variable(name) => assert_eq!(name, "myList"),
                    _ => panic!("Expected Variable expression, got {:?}", target.kind),
                }
                
                // Check index is an expression
                match &index.kind {
                    ExpressionKind::Binary { .. } => {},
                    _ => panic!("Expected Binary expression, got {:?}", index.kind),
                }
            },
            _ => panic!("Expected Index expression, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_nested_indexing() {
        let expr = parse_expression("matrix[i][j]");
        
        match &expr.kind {
            ExpressionKind::Index { target, index } => {
                // Check target is also an index expression
                match &target.kind {
                    ExpressionKind::Index { .. } => {},
                    _ => panic!("Expected Index expression, got {:?}", target.kind),
                }
                
                // Check index
                match &index.kind {
                    ExpressionKind::Variable(name) => assert_eq!(name, "j"),
                    _ => panic!("Expected Variable expression, got {:?}", index.kind),
                }
            },
            _ => panic!("Expected Index expression, got {:?}", expr.kind),
        }
    }
    
    #[test]
    fn test_member_and_index_access() {
        let expr = parse_expression("person.addresses[0]");
        
        match &expr.kind {
            ExpressionKind::Index { target, index: _ } => {
                // Check target is a member access
                match &target.kind {
                    ExpressionKind::MemberAccess { .. } => {},
                    _ => panic!("Expected MemberAccess expression, got {:?}", target.kind),
                }
            },
            _ => panic!("Expected Index expression, got {:?}", expr.kind),
        }
    }
} 