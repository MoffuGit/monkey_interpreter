use std::collections::HashMap;

use super::value::{get_builtin_by_name, Value};

pub fn new_builtins() -> HashMap<String, Value> {
    let mut builtins = HashMap::new();
    builtins.insert(
        String::from("len"),
        get_builtin_by_name("len".to_string()).unwrap(),
    );
    builtins.insert(
        String::from("first"),
        get_builtin_by_name("first".to_string()).unwrap(),
    );
    builtins.insert(
        String::from("last"),
        get_builtin_by_name("last".to_string()).unwrap(),
    );
    builtins.insert(
        String::from("rest"),
        get_builtin_by_name("rest".to_string()).unwrap(),
    );
    builtins.insert(
        String::from("push"),
        get_builtin_by_name("push".to_string()).unwrap(),
    );
    builtins.insert(
        String::from("puts"),
        get_builtin_by_name("puts".to_string()).unwrap(),
    );
    builtins
}
