#[cfg(test)]
mod parser_container_tests {
    // Importing required modules
    use crate::parser::{Parser, Statement, StatementKind};
    use crate::lexer::Lexer;
    
    // Helper function to parse a container definition
    fn parse_container(input: &str) -> Statement {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_statement()
    }
    
    #[test]
    fn test_basic_container() {
        let stmt = parse_container("define container Person\nend container");
        
        match &stmt.kind {
            StatementKind::ContainerDefinition { name, fields, methods } => {
                assert_eq!(name, "Person");
                assert_eq!(fields.len(), 0);
                assert_eq!(methods.len(), 0);
            },
            _ => panic!("Expected ContainerDefinition, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_container_with_fields() {
        let stmt = parse_container("define container Person\ndefine field name\ndefine field age\nend container");
        
        match &stmt.kind {
            StatementKind::ContainerDefinition { name, fields, methods } => {
                assert_eq!(name, "Person");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0], "name");
                assert_eq!(fields[1], "age");
                assert_eq!(methods.len(), 0);
            },
            _ => panic!("Expected ContainerDefinition, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_container_with_methods() {
        let stmt = parse_container(r#"
define container Person
    define field name
    
    define action greet
        print("Hello, " + name)
    end action
    
    define action setName(newName)
        name = newName
    end action
end container
"#);
        
        match &stmt.kind {
            StatementKind::ContainerDefinition { name, fields, methods } => {
                assert_eq!(name, "Person");
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0], "name");
                assert_eq!(methods.len(), 2);
                
                // Check method names
                assert_eq!(methods[0].name, "greet");
                assert_eq!(methods[1].name, "setName");
                
                // Check parameters of setName method
                assert_eq!(methods[1].parameters.len(), 1);
                assert_eq!(methods[1].parameters[0], "newName");
            },
            _ => panic!("Expected ContainerDefinition, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_constructor() {
        let stmt = parse_container(r#"
define container Counter
    define field value
    
    when created
        value = 0
    end when
    
    define action increment
        value = value + 1
    end action
end container
"#);
        
        match &stmt.kind {
            StatementKind::ContainerDefinition { name, fields, methods } => {
                assert_eq!(name, "Counter");
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0], "value");
                assert_eq!(methods.len(), 1);
                
                // Check constructor exists (implementation-specific)
                // This will depend on how your parser represents constructors
                // It might be a special method or a separate field
            },
            _ => panic!("Expected ContainerDefinition, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_container_inheritance() {
        // If WFL supports inheritance, test it here
        let stmt = parse_container("define container Student extends Person\ndefine field grade\nend container");
        
        match &stmt.kind {
            StatementKind::ContainerDefinition { name, fields, .. } => {
                assert_eq!(name, "Student");
                assert!(fields.contains(&"grade".to_string()));
                
                // Check inheritance relationship
                // This is implementation-specific
            },
            _ => panic!("Expected ContainerDefinition, got {:?}", stmt.kind),
        }
    }
    
    #[test]
    fn test_container_instantiation() {
        // Test code that creates an instance of a container
        let program = r#"
define container Person
    define field name
    define field age
end container

define variable person = new Person
person.name = "John"
person.age = 30
"#;
        
        let lexer = Lexer::new(program);
        let mut parser = Parser::new(lexer);
        let statements = parser.parse();
        
        // Check we have 3 statements (container definition, variable declaration, assignment)
        assert_eq!(statements.len(), 3);
        
        // Check first statement is a container definition
        match &statements[0].kind {
            StatementKind::ContainerDefinition { .. } => {},
            _ => panic!("Expected ContainerDefinition, got {:?}", statements[0].kind),
        }
        
        // Check second statement is a variable declaration
        match &statements[1].kind {
            StatementKind::VarDeclaration { .. } => {},
            _ => panic!("Expected VarDeclaration, got {:?}", statements[1].kind),
        }
        
        // Check third statement is an assignment
        match &statements[2].kind {
            StatementKind::Assignment { .. } => {},
            _ => panic!("Expected Assignment, got {:?}", statements[2].kind),
        }
    }
} 