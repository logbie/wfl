#[cfg(test)]
mod memory_tests {
    use crate::interpreter::{Interpreter, Value};
    use crate::config::WflConfig;
    use std::rc::Rc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_memory_tracking() {
        let interpreter = Interpreter::new();
        
        assert_eq!(*interpreter.bytes_allocated.borrow(), 0);
        
        let items = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        let list = Value::new_list(items, &interpreter).unwrap();
        
        assert!(*interpreter.bytes_allocated.borrow() > 0);
        
        let current = *interpreter.bytes_allocated.borrow();
        interpreter.track_deallocation(100);
        assert_eq!(*interpreter.bytes_allocated.borrow(), current.saturating_sub(100));
    }
    
    #[test]
    fn test_memory_limit() {
        let mut config = WflConfig::default();
        config.max_memory_mb = 1;
        let interpreter = Interpreter::with_config(&config);
        
        let result = interpreter.track_allocation(1024 * 1024 * 2); // 2MB
        
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind, crate::interpreter::error::ErrorKind::OutOfMemory);
        }
    }
    
    #[test]
    fn env_cycle() {
        let interpreter = Interpreter::new();
        let global_env = Rc::clone(interpreter.global_env());
        
        drop(interpreter);
        
        thread::sleep(Duration::from_millis(10));
        
        assert_eq!(Rc::strong_count(&global_env), 1);
    }
}
