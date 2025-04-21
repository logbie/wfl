use super::{Environment, Interpreter, Value};
use crate::lexer::lex_wfl_with_positions;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;
// use std::io::Write;

#[tokio::test]
async fn test_literal_evaluation() {
    let interpreter = Interpreter::new();
    let env = Environment::new_global();

    let source = "42";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    if let Some(stmt) = program.statements.first() {
        if let crate::parser::ast::Statement::ExpressionStatement { expression, .. } = stmt {
            let result = interpreter
                .evaluate_expression(expression, env)
                .await
                .unwrap();
            match result {
                (Value::Number(n), _) => assert_eq!(n, 42.0),
                _ => panic!("Expected number, got {:?}", result),
            }
        } else {
            panic!("Expected expression statement");
        }
    } else {
        panic!("No statements in program");
    }
}

#[tokio::test]
async fn test_variable_declaration_and_access() {
    let mut interpreter = Interpreter::new();

    let source = "store x as 42\nx";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    let result = interpreter.interpret(&program).await.unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 42.0),
        _ => panic!("Expected number, got {:?}", result),
    }
}

#[tokio::test]
async fn test_binary_operations() {
    let mut interpreter = Interpreter::new();

    let source = "2 plus 3";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).await.unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 5.0),
        _ => panic!("Expected number, got {:?}", result),
    }

    let source = "2 is less than 3";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).await.unwrap();
    match result {
        Value::Bool(b) => assert!(b),
        _ => panic!("Expected boolean, got {:?}", result),
    }
}

#[tokio::test]
async fn test_if_statement() {
    let mut interpreter = Interpreter::new();

    let source = "check if yes: display \"true\" otherwise: display \"false\" end check";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).await.unwrap();

    match result {
        Value::Null => {}
        _ => panic!("Expected null, got {:?}", result),
    }
}

/*
#[tokio::test]
async fn test_function_definition_and_call() {
    let mut interpreter = Interpreter::new();

    let source = "define action called add: give back 2 plus 3 end action\nadd";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();
    let result = interpreter.interpret(&program).await.unwrap();

    match result {
        Value::Number(n) => assert_eq!(n, 5.0),
        _ => panic!("Expected number, got {:?}", result),
    }
}
*/

#[tokio::test]
async fn test_count_loop_with_direct_access() {
    let mut interpreter = Interpreter::new();

    let source = "
    count from 1 to 5:
        display \"Count: \" with count
    end count
    ";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    let result = interpreter.interpret(&program).await.unwrap();

    match result {
        Value::Null => {}
        _ => panic!("Expected null, got {:?}", result),
    }
}
#[tokio::test]
async fn test_timeout_happy_path() {
    let mut interpreter = Interpreter::with_timeout(1); // 1 second timeout

    let source = "store x as 42\nx"; // A quick script
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    let result = interpreter.interpret(&program);
    assert!(result.await.is_ok());
}

#[tokio::test]
async fn test_timeout_forever_loop() {
    let mut interpreter = Interpreter::with_timeout(1); // 1 second timeout

    let source = "
    count from 1 to 1000000000:
        store x as 1 plus 1
    end count
    ";
    let tokens = lex_wfl_with_positions(source);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    let start = std::time::Instant::now();
    let result = interpreter.interpret(&program);
    let elapsed = start.elapsed();

    let result_value = result.await;
    assert!(result_value.is_err());
    if let Err(errors) = result_value {
        assert!(!errors.is_empty());
        println!("Actual error message: {}", errors[0].message);
        assert!(errors[0].message.contains("Execution exceeded timeout"));
    }

    assert!(
        elapsed.as_millis() <= 1100,
        "Timeout took too long: {:?}",
        elapsed
    );
}

#[tokio::test]
async fn test_exit_loop_statement() {
    let input = r#"
    store count as 0
    repeat count from 1 to 3:
      repeat count from 1 to 3:
        exit loop
      end repeat
      store count as count plus 1
    end repeat
    "#;

    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program).await;

    assert!(result.is_ok());

    let env = interpreter.global_env().borrow();
    if let Some(Value::Number(count)) = env.get("count") {
        assert_eq!(count, 3.0);
    } else {
        panic!("count variable missing or not a number");
    }
}

#[tokio::test]
async fn test_exit_loop_outside_loop() {
    let input = "exit loop";

    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program).await;

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("outside of any loop"));
}

#[tokio::test]
async fn test_exit_loop_three_level_nested() {
    let input = r#"
    store result as ""
    repeat count from 1 to 3:
      store outer as count
      repeat count from 1 to 3:
        store middle as count
        repeat count from 1 to 3:
          store inner as count
          store result as result with " " with outer with "-" with middle with "-" with inner
          exit loop
        end repeat
        store result as result with " MIDDLE"
      end repeat
      store result as result with " OUTER"
    end repeat
    "#;

    let tokens = lex_wfl_with_positions(input);
    let mut parser = Parser::new(&tokens);
    let program = parser.parse().unwrap();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program).await;

    assert!(result.is_ok());

    let env = interpreter.global_env().borrow();
    if let Some(Value::Text(result_str)) = env.get("result") {
        assert!(result_str.contains("MIDDLE"));
        assert!(result_str.contains("OUTER"));

        let middle_count = result_str.matches("MIDDLE").count();
        let outer_count = result_str.matches("OUTER").count();

        assert_eq!(outer_count, 3);
        assert_eq!(middle_count, 9);
    } else {
        panic!("result variable missing or not a string");
    }
}

#[tokio::test]
async fn test_type_error_blocked_by_default() {
    let input = "store x as 1\nstore x as \"oops\"";
    let tokens = lex_wfl_with_positions(input);
    let program = Parser::new(&tokens).parse().unwrap();

    assert!(!program.statements.is_empty());

    let mut tc = TypeChecker::new();
    assert!(tc.check_types(&program).is_err());
}
