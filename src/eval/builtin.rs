use std::collections::HashMap;

use super::value::Value;
use super::EvalError;

pub fn new_builtins() -> HashMap<String, Value> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Value::Builtin(builtin_len));
    builtins.insert(String::from("first"), Value::Builtin(builtin_first));
    builtins.insert(String::from("last"), Value::Builtin(builtin_last));
    builtins.insert(String::from("rest"), Value::Builtin(builtin_rest));
    builtins.insert(String::from("push"), Value::Builtin(builtin_push));
    builtins.insert(String::from("puts"), Value::Builtin(builtin_puts));
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
        Value::Array(array) => Ok(Value::Int(array.len() as i64)),
        arg => Err(EvalError::new(format!(
            r#"argument to "len" not supported: got {}"#,
            arg.as_type()
        ))),
    }
}

fn builtin_first(args: Vec<Value>) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::new(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        )));
    }

    match &args[0] {
        Value::Array(array) => match array.first() {
            Some(value) => Ok(value.clone()),
            None => Err(EvalError::new("the array is empty")),
        },
        arg => Err(EvalError::new(format!(
            "argument to 'first' must be ARRAY, got {}",
            arg.as_type()
        ))),
    }
}

fn builtin_last(args: Vec<Value>) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::new(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        )));
    }

    match &args[0] {
        Value::Array(array) => match array.last() {
            Some(value) => Ok(value.clone()),
            None => Err(EvalError::new("the array is empty")),
        },
        arg => Err(EvalError::new(format!(
            "argument to 'last' must be ARRAY, got {}",
            arg.as_type()
        ))),
    }
}

fn builtin_rest(args: Vec<Value>) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::new(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        )));
    }

    match &args[0] {
        Value::Array(array) => Ok(Value::Array(array[1..].to_vec())),
        arg => Err(EvalError::new(format!(
            "argument to 'rest' must be ARRAY, got {}",
            arg.as_type()
        ))),
    }
}

fn builtin_push(args: Vec<Value>) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::new(format!(
            "wrong number of arguments, got={}, want=2",
            args.len()
        )));
    }

    match &args[0] {
        Value::Array(array) => {
            let mut new_array = array.clone();
            new_array.push(args[1].clone());
            Ok(Value::Array(new_array))
        }
        arg => Err(EvalError::new(format!(
            "arguments to 'rest' must be ARRAY, got: ({}, {})",
            arg.as_type(),
            &args[1].as_type()
        ))),
    }
}

fn builtin_puts(args: Vec<Value>) -> Result<Value, EvalError> {
    args.iter().for_each(|arg| println!("{arg}"));
    Ok(Value::Null)
}
