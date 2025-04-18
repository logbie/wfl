use crate::lexer::lex_wfl_with_positions;
use crate::parser::Parser;
use super::{Interpreter, Value, Environment};

#[test]
fn test_literal_evaluation() {
    let mut interpreter = Interpreter::new();
    let env = Environment::new_global();
    
    let source = "42";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    
    if let Some(stmt) = program.statements.first() {
        if let crate::parser::ast::Statement::ExpressionStatement { expression, .. } = stmt {
            let result = interpreter.evaluate_expression(expression, env).unwrap();
            match result {
                Value::Number(n) => assert_eq!(n, 42.0),
                _ => panic!("Expected number, got {:?}", result),
            }
        } else {
            panic!("Expected expression statement");
        }
    } else {
        panic!("No statements in program");
    }
}

#[test]
fn test_variable_declaration_and_access() {
    let mut interpreter = Interpreter::new();
    
    let source = "store x as 42\nx";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    
    let result = interpreter.interpret(&program).unwrap();
    
    match result {
        Value::Number(n) => assert_eq!(n, 42.0),
        _ => panic!("Expected number, got {:?}", result),
    }
}

#[test]
fn test_binary_operations() {
    let mut interpreter = Interpreter::new();
    
    let source = "2 plus 3";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 5.0),
        _ => panic!("Expected number, got {:?}", result),
    }
    
    let source = "2 is less than 3";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).unwrap();
    match result {
        Value::Bool(b) => assert!(b),
        _ => panic!("Expected boolean, got {:?}", result),
    }
}

#[test]
fn test_if_statement() {
    let mut interpreter = Interpreter::new();
    
    let source = "check if yes: display \"true\" otherwise: display \"false\" end check";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).unwrap();
    
    match result {
        Value::Null => {},
        _ => panic!("Expected null, got {:?}", result),
    }
}

/*
#[test]
fn test_function_definition_and_call() {
    let mut interpreter = Interpreter::new();
    
    let source = "define action called add: give back 2 plus 3 end action\nadd";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).unwrap();
    
    match result {
        Value::Number(n) => assert_eq!(n, 5.0),
        _ => panic!("Expected number, got {:?}", result),
    }
}
*/
