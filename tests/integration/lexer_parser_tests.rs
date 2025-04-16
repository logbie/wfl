#[cfg(test)]
mod lexer_parser_integration_tests {
    // Importing required modules
    use crate::lexer::Lexer;
    use crate::parser::{Parser, Expression, ExpressionKind, Statement, StatementKind};
    
    // Helper function to parse a program
    fn parse_program(input: &str) -> Vec<Statement> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse()
    }
    
    #[test]
    fn test_basic_program_parsing() {
        let input = r#"
        define variable x = 10
        define variable y = 20
        define variable sum = x + y
        "#;
        
        let statements = parse_program(input);
        
        // Check we have 3 statements
        assert_eq!(statements.len(), 3);
        
        // Check they're all variable declarations
        for stmt in &statements {
            match &stmt.kind {
                StatementKind::VarDeclaration { .. } => {},
                _ => panic!("Expected VarDeclaration, got {:?}", stmt.kind),
            }
        }
        
        // Check the third statement has a binary expression
        if let StatementKind::VarDeclaration { initializer, .. } = &statements[2].kind {
            match &initializer.kind {
                ExpressionKind::Binary { .. } => {},
                _ => panic!("Expected Binary expression, got {:?}", initializer.kind),
            }
        } else {
            panic!("Expected VarDeclaration");
        }
    }
    
    #[test]
    fn test_if_statement_parsing() {
        let input = r#"
        define variable x = 10
        
        if x > 5
            define variable result = "greater"
        else
            define variable result = "lesser"
        end if
        "#;
        
        let statements = parse_program(input);
        
        // Check we have 2 statements (variable declaration and if statement)
        assert_eq!(statements.len(), 2);
        
        // Check the second statement is an if statement
        match &statements[1].kind {
            StatementKind::If { .. } => {},
            _ => panic!("Expected If statement, got {:?}", statements[1].kind),
        }
    }
    
    #[test]
    fn test_while_loop_parsing() {
        let input = r#"
        define variable counter = 0
        
        while counter < 5
            counter = counter + 1
        end while
        "#;
        
        let statements = parse_program(input);
        
        // Check we have 2 statements (variable declaration and while loop)
        assert_eq!(statements.len(), 2);
        
        // Check the second statement is a while loop
        match &statements[1].kind {
            StatementKind::While { .. } => {},
            _ => panic!("Expected While statement, got {:?}", statements[1].kind),
        }
    }
    
    #[test]
    fn test_container_definition_parsing() {
        let input = r#"
        define container Person
            define field name
            define field age
            
            define action greet
                # function body would go here
            end action
        end container
        "#;
        
        let statements = parse_program(input);
        
        // Check we have 1 statement (container definition)
        assert_eq!(statements.len(), 1);
        
        // Check it's a container definition
        match &statements[0].kind {
            StatementKind::ContainerDefinition { .. } => {},
            _ => panic!("Expected ContainerDefinition, got {:?}", statements[0].kind),
        }
    }
    
    #[test]
    fn test_complex_program_parsing() {
        let input = r#"
        define variable count = 0
        
        define container Counter
            define field value
            
            define action increment
                value = value + 1
            end action
            
            define action getValue
                return value
            end action
        end container
        
        define variable counter = new Counter
        counter.value = 10
        
        while count < 5
            counter.increment()
            count = count + 1
        end while
        
        define variable result = counter.getValue()
        "#;
        
        let statements = parse_program(input);
        
        // We don't need to check every detail, just that parsing completes successfully
        // and we get the expected number of top-level statements
        
        // Expected top-level statements:
        // 1. Variable declaration (count)
        // 2. Container definition (Counter)
        // 3. Variable declaration (counter)
        // 4. Assignment (counter.value)
        // 5. While loop
        // 6. Variable declaration (result)
        
        assert_eq!(statements.len(), 6);
    }
} 