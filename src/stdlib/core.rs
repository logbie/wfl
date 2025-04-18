use crate::interpreter::error::RuntimeError;
use crate::interpreter::environment::Environment;
use crate::interpreter::value::Value;
use std::rc::Rc;

pub fn native_print(args: Vec<Value>) -> Result<Value, RuntimeError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    Ok(Value::Null)
}

pub fn native_typeof(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::new(
            format!("typeof expects 1 argument, got {}", args.len()),
            0, 0
        ));
    }
    
    let type_name = args[0].type_name();
    Ok(Value::Text(Rc::from(type_name)))
}

pub fn native_isnothing(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::new(
            format!("isnothing expects 1 argument, got {}", args.len()),
            0, 0
        ));
    }
    
    match &args[0] {
        Value::Null => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
    }
}

pub fn register_core(env: &mut Environment) {
    env.define("print", Value::NativeFunction(native_print));
    
    env.define("typeof", Value::NativeFunction(native_typeof));
    env.define("isnothing", Value::NativeFunction(native_isnothing));
    
    env.define("type_of", Value::NativeFunction(native_typeof));
    env.define("is_nothing", Value::NativeFunction(native_isnothing));
}
