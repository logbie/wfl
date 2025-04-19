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
    
    #[tokio::test]
    async fn test_closure_outlives_scope() {
        let mut interpreter = Interpreter::new();
        
        let source = r#"
        define action make_counter():
            store i as 0
            define action counter():
                change i to i plus 1
                return i
            end action
            return counter
        end action
        
        store c as make_counter()
        store a as c()   # 1
        store b as c()   # 2
        "#;
        
        let tokens = lex_wfl_with_positions(source);
        let mut parser = Parser::new(&tokens);
        let program = parser.parse().unwrap();
        
        let result = interpreter.interpret(&program).await;
        assert!(result.is_ok());
        
        let global_env = interpreter.global_env.clone();
        
        {
            let a_val = global_env.borrow().get("a").unwrap();
            let b_val = global_env.borrow().get("b").unwrap();
            
            if let (Value::Number(a_num), Value::Number(b_num)) = (&a_val, &b_val) {
                assert_eq!(*a_num, 1.0);
                assert_eq!(*b_num, 2.0);
            } else {
                panic!("Expected numbers, got {:?} and {:?}", a_val, b_val);
            }
        }
    }
}
