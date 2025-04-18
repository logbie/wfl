use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use crate::parser::ast::{Statement};
use super::environment::Environment;
use super::error::RuntimeError;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Text(Rc<str>),
    Bool(bool),
    List(Rc<RefCell<Vec<Value>>>),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Function(Rc<FunctionValue>),
    NativeFunction(NativeFunction),
    Null,
}

pub type NativeFunction = fn(Vec<Value>) -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub struct FunctionValue {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
    pub env: Rc<RefCell<Environment>>,
    pub line: usize,
    pub column: usize,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::Text(_) => "Text",
            Value::Bool(_) => "Boolean",
            Value::List(_) => "List",
            Value::Object(_) => "Object",
            Value::Function(_) => "Function",
            Value::NativeFunction(_) => "NativeFunction",
            Value::Null => "Null",
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::List(list) => !list.borrow().is_empty(),
            Value::Object(obj) => !obj.borrow().is_empty(),
            Value::Function(_) | Value::NativeFunction(_) => true,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "\"{}\"", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::List(l) => {
                let values = l.borrow();
                write!(f, "[")?;
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(o) => {
                let map = o.borrow();
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {:?}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Function(func) => {
                write!(f, "Function({})", func.name.as_ref().unwrap_or(&"anonymous".to_string()))
            }
            Value::NativeFunction(_) => write!(f, "NativeFunction"),
            Value::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", if *b { "yes" } else { "no" }),
            Value::List(_) => write!(f, "[List]"),
            Value::Object(_) => write!(f, "[Object]"),
            Value::Function(func) => {
                write!(f, "action {}", func.name.as_ref().unwrap_or(&"anonymous".to_string()))
            }
            Value::NativeFunction(_) => write!(f, "[NativeFunction]"),
            Value::Null => write!(f, "nothing"),
        }
    }
}
