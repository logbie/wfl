use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use wfl::debug::SafeDebug;
use wfl::interpreter::value::Value;

#[test]
fn test_list_truncation() {
    let mut items = Vec::with_capacity(100);
    for i in 0..100 {
        items.push(Value::Number(i as f64));
    }

    let list_value = Value::List(Rc::new(RefCell::new(items)));

    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    list_value.safe_fmt(&mut formatter).unwrap();

    assert!(
        output.contains("… (84 more)"),
        "SafeDebug didn't properly truncate the list: {}",
        output
    );
}

#[test]
fn test_string_truncation() {
    let long_string = "a".repeat(200);
    let text_value = Value::Text(Rc::from(long_string.as_str()));

    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    text_value.safe_fmt(&mut formatter).unwrap();

    assert!(
        output.contains("..."),
        "SafeDebug didn't properly truncate the string: {}",
        output
    );

    assert!(
        output.len() < long_string.len() + 5,
        "SafeDebug didn't truncate the string: output length = {}, original length = {}",
        output.len(),
        long_string.len()
    );
}

#[test]
fn test_object_truncation() {
    let mut map = HashMap::new();
    for i in 0..30 {
        map.insert(format!("key_{}", i), Value::Number(i as f64));
    }

    let object_value = Value::Object(Rc::new(RefCell::new(map)));

    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    object_value.safe_fmt(&mut formatter).unwrap();

    assert!(
        output.contains("… (14 more)"),
        "SafeDebug didn't properly truncate the object: {}",
        output
    );
}
