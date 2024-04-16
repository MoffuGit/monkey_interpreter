use std::collections::HashMap;

use super::value::Value;
use super::EvalError;

pub fn new_builtins() -> HashMap<String, Value> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Value::Builtin(builtin_len));
    builtins
}

fn builtin_len(args: Vec<Value>) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::new(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        )));
    }

    match &args[0] {
        Value::String(arg) => Ok(Value::Int(arg.len() as i64)),
        arg => Err(EvalError::new(format!(
            r#"argument to "len" not supported: got {}"#,
            arg.as_type()
        ))),
    }
}
