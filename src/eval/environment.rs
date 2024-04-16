use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::value::Value;

#[derive(PartialEq, Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Value>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_with_outer(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&mut self, name: impl AsRef<str>) -> Option<Value> {
        match self.store.get(name.as_ref()) {
            Some(value) => Some(value.clone()),
            None => match self.outer {
                Some(ref outer) => outer.borrow_mut().get(name),
                None => None,
            },
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, value: Value) {
        self.store.insert(name.into(), value);
    }
}

impl From<HashMap<String, Value>> for Environment {
    fn from(value: HashMap<String, Value>) -> Self {
        Environment {
            store: value,
            outer: None,
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
