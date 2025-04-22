use super::value::Value;
use crate::Ident;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap<Ident, Value>,
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

    #[inline]
    pub fn new_child_env(parent: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            parent: Some(Rc::downgrade(parent)),
        }))
    }

    pub fn define<N: Into<Ident>>(&mut self, name: N, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn assign<N: Into<Ident>>(&mut self, name: N, value: Value) -> Result<(), String> {
        let name_ident = name.into();
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.values.entry(name_ident.clone()) {
            e.insert(value);
            Ok(())
        } else if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                parent.borrow_mut().assign(name_ident, value)
            } else {
                Err("Parent environment no longer exists".to_string())
            }
        } else {
            Err(format!("Undefined variable '{}'", name_ident))
        }
    }

    pub fn get<N: AsRef<str>>(&self, name: N) -> Option<Value> {
        let name_ref = name.as_ref();
        if let Some(value) = self.values.get(&crate::common::ident::intern(name_ref)) {
            Some(value.clone())
        } else if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                parent.borrow().get(name_ref)
            } else {
                None
            }
        } else {
            None
        }
    }
}
