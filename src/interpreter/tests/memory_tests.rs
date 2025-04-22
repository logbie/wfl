use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::config::WflConfig;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

#[test]
fn test_memory_tracking() {
    let config = WflConfig::default();
    let interpreter = Interpreter::with_config(&config);
    
    // Test allocation tracking
    let bytes = 1024 * 1024; // 1MB
    assert!(interpreter.track_allocation(bytes).is_ok());
    
    // Verify bytes are tracked
    assert!(interpreter.bytes_allocated.borrow().clone() >= bytes);
    
    // Test deallocation
    interpreter.track_deallocation(bytes);
    assert!(interpreter.bytes_allocated.borrow().clone() < bytes);
    
    // Test memory limit
    let over_limit = config.max_memory_mb * 1024 * 1024 + 1;
    let result = interpreter.track_allocation(over_limit);
    assert!(result.is_err());
    
    // Test Value constructors
    let text = "x".repeat(1000);
    let text_val = Value::new_text(text, &interpreter).unwrap();
    assert!(matches!(text_val, Value::Text(_)));
    
    let mut items = Vec::with_capacity(100);
    for i in 0..100 {
        items.push(Value::Number(i as f64));
    }
    
    let list_val = Value::new_list(items, &interpreter).unwrap();
    assert!(matches!(list_val, Value::List(_)));
}

#[test]
fn test_env_cycle() {
    let config = WflConfig::default();
    let interpreter = Rc::new(Interpreter::with_config(&config));
    
    // Get a reference to the global environment
    let global_env = Rc::clone(interpreter.global_env());
    
    // Create a closure that captures the environment
    let _closure = Value::Function(Rc::new(crate::interpreter::value::FunctionValue::new(
        "test".to_string(),
        vec!["x".to_string()],
        Box::new(crate::parser::ast::Statement::Block(vec![])),
        Rc::clone(&global_env),
        0,
        0,
    )));
    
    // Drop the interpreter
    drop(interpreter);
    
    // Give any async drops a chance to run
    thread::sleep(Duration::from_millis(10));
    
    // Verify that only one strong reference to the environment remains
    assert_eq!(Rc::strong_count(&global_env), 1);
}
