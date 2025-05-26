use std::cell::RefCell;
use std::rc::Rc;
use wfl::interpreter::{Interpreter, Value, control_flow::ControlFlow};
use wfl::parser::{Parser, Program};

async fn execute_wfl(code: &str) -> Result<Value, String> {
    let mut parser = Parser::new(code);
    let program = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut interpreter = Interpreter::default();
    interpreter
        .interpret(&program)
        .await
        .map_err(|e| format!("Runtime error: {:?}", e))
}

async fn capture_output(code: &str) -> Vec<String> {
    let mut output = Vec::new();

    let capture_fn = Rc::new(RefCell::new(move |s: &str| {
        output.push(s.to_string());
    }));

    let mut parser = Parser::new(code);
    let program = parser.parse().unwrap();

    let mut interpreter = Interpreter::default();

    let _ = interpreter.interpret(&program).await;

    output
}

#[tokio::test]
async fn test_break_in_forever_loop() {
    let code = r#"
    store counter as 0
    repeat forever:
        store counter as counter + 1
        if counter > 5:
            break
        end if
    end repeat
    display counter
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null); // The last statement is display, which returns Null
}

#[tokio::test]
async fn test_break_in_count_loop() {
    let code = r#"
    store result as 0
    count from 1 to 10:
        store result as result + count
        if count > 5:
            break
        end if
    end count
    display result
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_break_in_repeat_while_loop() {
    let code = r#"
    store counter as 0
    store result as 0
    repeat while counter < 10:
        store counter as counter + 1
        store result as result + counter
        if counter > 5:
            break
        end if
    end repeat
    display result
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_break_in_repeat_until_loop() {
    let code = r#"
    store counter as 0
    store result as 0
    repeat until counter >= 10:
        store counter as counter + 1
        store result as result + counter
        if counter > 5:
            break
        end if
    end repeat
    display result
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_continue_in_count_loop() {
    let code = r#"
    store result as 0
    count from 1 to 10:
        if count % 2 == 0:
            continue
        end if
        store result as result + count
    end count
    display result
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_exit_from_nested_loops() {
    let code = r#"
    store result as 0
    count from 1 to 5:
        count from 1 to 5:
            store result as result + 1
            if count == 3:
                exit loop
            end if
        end count
    end count
    display result
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_nested_loops_with_break() {
    let code = r#"
    store outer_count as 0
    store inner_count as 0
    store total as 0
    
    repeat while outer_count < 5:
        store outer_count as outer_count + 1
        store inner_count as 0
        
        repeat while inner_count < 5:
            store inner_count as inner_count + 1
            store total as total + 1
            
            if inner_count == 3:
                break
            end if
        end repeat
    end repeat
    
    display total
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_return_from_action() {
    let code = r#"
    action test_return(x):
        if x > 5:
            return x
        end if
        return x * 2
    end action
    
    store result as test_return(10)
    display result
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_return_from_loop_in_action() {
    let code = r#"
    action find_value(target):
        count from 1 to 10:
            if count == target:
                return count
            end if
        end count
        return 0
    end action
    
    store result as find_value(5)
    display result
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_break_from_foreach_loop() {
    let code = r#"
    store items as [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    store sum as 0
    
    for each item in items:
        store sum as sum + item
        if item >= 5:
            break
        end if
    end for
    
    display sum
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_continue_from_foreach_loop() {
    let code = r#"
    store items as [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    store sum as 0
    
    for each item in items:
        if item % 2 == 0:
            continue
        end if
        store sum as sum + item
    end for
    
    display sum
    "#;

    let result = execute_wfl(code).await.unwrap();
    assert_eq!(result, Value::Null);
}
