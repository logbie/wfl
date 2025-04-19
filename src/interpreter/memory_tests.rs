#[cfg(test)]
mod tests {
    use super::super::{Environment, Interpreter, Value};
    use crate::lexer::lex_wfl_with_positions;
    use crate::parser::Parser;
    use std::rc::Rc;
    
    #[tokio::test]
    async fn test_no_memory_leak_from_function_env_cycle() {
        let mut interpreter = Interpreter::new();
        
        let source = r#"
        define action test_func():
            store x as 42
            return x
        end action
        
        store result as test_func()
        "#;
        
        let tokens = lex_wfl_with_positions(source);
        let mut parser = Parser::new(&tokens);
        let program = parser.parse().unwrap();
        
        let result = interpreter.interpret(&program).await;
        assert!(result.is_ok());
        
        let global_env = interpreter.global_env.clone();
        
        {
            let func_val = global_env.borrow().get("test_func").unwrap();
            
            if let Value::Function(func) = func_val {
                assert!(func.env.upgrade().is_some());
                
                let strong_count = Rc::strong_count(&global_env);
                assert!(strong_count >= 1);
            }
        }
        
        drop(interpreter);
        
        assert_eq!(Rc::strong_count(&global_env), 1);
    }
}
