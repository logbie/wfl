use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use super::value::Value;

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    pub parent: Option<Weak<RefCell<Environment>>>,
}

impl Environment {
    pub fn new_global() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            parent: None,
        }))
    }
    
    pub fn new(parent: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            parent: Some(Rc::downgrade(parent)),
        }))
    }
    
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }
    
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                parent.borrow_mut().assign(name, value)
            } else {
                Err(format!("Parent environment no longer exists"))
            }
        } else {
            Err(format!("Undefined variable '{}'", name))
        }
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                parent.borrow().get(name)
            } else {
                None
            }
        } else {
            None
        }
    }
}
