use std::collections::HashMap;

use super::value::Value;

#[derive(PartialEq, Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
