use wfl::lexer::lex_wfl_with_positions;
use wfl::parser::Parser;
use wfl::typechecker::TypeChecker;

fn assert_typechecks(src: &str) {
    let tokens = lex_wfl_with_positions(src);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().expect("Failed to parse test program");
    
    for stmt in &program.statements {
        println!("Statement: {:?}", stmt);
    }
    
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_types(&program);
    assert!(result.is_ok(), "Expected program to type-check successfully: {:?}", result.err());
}

fn assert_type_error(src: &str, expected_error: &str) {
    let tokens = lex_wfl_with_positions(src);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().expect("Failed to parse test program");
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_types(&program);
    assert!(result.is_err(), "Expected type error but program type-checked successfully");
    let errors = result.err().unwrap();
    assert!(errors.iter().any(|e| e.message.contains(expected_error)), 
           "Expected error containing '{}', got: {:?}", expected_error, errors);
}

#[test]
fn test_equality_comparison_in_compound_expressions() {
    assert_typechecks(r#"
        store result as 3 times 2
        check if result is equal to 6:
            display "pass"
        end check
    "#);
    
    assert_typechecks(r#"
        store x as 5
        store y as 7
        check if x is not equal to y:
            display "pass"
        end check
    "#);
    
    let src = r#"
        store x as 5
        store y as 7
        check if x plus 1 is not equal to y:
            display "pass"
        end check
    "#;
    let tokens = wfl::lexer::lex_wfl_with_positions(src);
    let mut parser = wfl::parser::Parser::new(&tokens);
    let program = parser.parse().expect("Failed to parse test program");
    
    for stmt in &program.statements {
        println!("Debug Statement: {:?}", stmt);
    }
    
    assert_typechecks(r#"
        check if 3 times 2 is equal to 6:
            display "pass"
        end check
    "#);
    
    assert_type_error(r#"
        check if 5 is equal to "5":
            display "fail"
        end check
    "#, "Cannot compare");
}

#[test]
fn test_polymorphic_equality_comparisons() {
    assert_typechecks(r#"
        store a as true
        store b as false
        check if a is equal to b:
            display "pass"
        end check
    "#);
    
    assert_typechecks(r#"
        store x as "hello"
        store y as "world"
        check if x is equal to y:
            display "pass"
        end check
    "#);
}
