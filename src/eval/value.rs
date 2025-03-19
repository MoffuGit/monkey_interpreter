use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

use crate::ast::statement::Statement;
use crate::code::Instructions;

use super::environment::Environment;

pub type BuiltinFuncion = fn(Vec<Value>) -> Result<Value, String>;

pub enum Builtin {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
}

impl TryFrom<u8> for Builtin {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Builtin::Len),
            1 => Ok(Builtin::First),
            2 => Ok(Builtin::Last),
            3 => Ok(Builtin::Rest),
            4 => Ok(Builtin::Push),
            5 => Ok(Builtin::Puts),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for Builtin {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "len" => Ok(Builtin::Len),
            "first" => Ok(Builtin::First),
            "last" => Ok(Builtin::Last),
            "rest" => Ok(Builtin::Rest),
            "push" => Ok(Builtin::Push),
            "puts" => Ok(Builtin::Puts),
            _ => Err(()),
        }
    }
}

impl Builtin {
    pub fn get_builtin_fn(builtin: Builtin) -> BuiltinFuncion {
        match builtin {
            Builtin::Len => builtin_len,
            Builtin::First => builtin_first,
            Builtin::Last => builtin_last,
            Builtin::Rest => builtin_rest,
            Builtin::Push => builtin_push,
            Builtin::Puts => builtin_puts,
        }
    }
}

pub fn get_builtin_by_name(name: String) -> Result<Value, String> {
    Ok(Value::Builtin(match name.as_str() {
        "len" => builtin_len,
        "first" => builtin_first,
        "last" => builtin_last,
        "rest" => builtin_rest,
        "push" => builtin_push,
        "puts" => builtin_puts,
        _ => return Err("Invalid builtin name".into()),
    }))
}

fn builtin_len(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Value::String(arg) => Ok(Value::Int(arg.len() as i64)),
        Value::Array(array) => Ok(Value::Int(array.len() as i64)),
        arg => Err(format!(
            r#"argument to "len" not supported: got {}"#,
            arg.as_type()
        )),
    }
}

fn builtin_first(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Value::Array(array) => match array.first() {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Null),
        },
        arg => Err(format!(
            "argument to 'first' must be ARRAY, got {}",
            arg.as_type()
        )),
    }
}

fn builtin_last(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Value::Array(array) => match array.last() {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Null),
        },
        arg => Err(format!(
            "argument to 'last' must be ARRAY, got {}",
            arg.as_type()
        )),
    }
}

fn builtin_rest(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "wrong number of arguments, got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Value::Array(array) => {
            if array.is_empty() {
                return Ok(Value::Null);
            }
            Ok(Value::Array(array[1..].to_vec()))
        }
        arg => Err(format!(
            "argument to 'rest' must be ARRAY, got {}",
            arg.as_type()
        )),
    }
}

fn builtin_push(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "wrong number of arguments, got={}, want=2",
            args.len()
        ));
    }

    match &args[0] {
        Value::Array(array) => {
            let mut new_array = array.clone();
            new_array.push(args[1].clone());
            Ok(Value::Array(new_array))
        }
        arg => Err(format!(
            "argument to 'push' must be ARRAY, got: {}",
            arg.as_type(),
        )),
    }
}

fn builtin_puts(args: Vec<Value>) -> Result<Value, String> {
    args.iter().for_each(|arg| println!("{arg}"));
    Ok(Value::Null)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(String),
    Null,
    Let,
    Return(Box<Value>),
    Array(Vec<Value>),
    Function {
        parameters: Vec<String>,
        body: Vec<Statement>,
        env: Rc<RefCell<Environment>>,
    },
    Builtin(BuiltinFuncion),
    Hash(HashMap<Value, Value>),
    CompiledFunction {
        instructions: Instructions,
        num_locals: usize,
        num_parameters: usize,
    },
    Closure {
        fun: Box<Value>,
        free: Vec<Value>,
    },
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<HashMap<Value, Value>> for Value {
    fn from(value: HashMap<Value, Value>) -> Self {
        Value::Hash(value)
    }
}

impl<T: Into<Value> + Clone> From<Vec<T>> for Value {
    fn from(values: Vec<T>) -> Self {
        Value::Array(
            values
                .iter()
                .map(|value| std::convert::Into::<Value>::into(value.clone()))
                .collect::<Vec<Value>>(),
        )
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match *self {
            Value::Int(ref int) => int.hash(state),
            Value::Bool(ref bool) => bool.hash(state),
            Value::String(ref string) => string.hash(state),
            _ => "".hash(state),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::Null => write!(f, "null"),
            Value::Return(value) => write!(f, "{value}"),
            Value::Let => write!(f, "let"),
            Value::Function {
                parameters, body, ..
            } => {
                write!(f, "fn ({}) {{", parameters.join(", "))?;

                for statement in body {
                    write!(f, "{statement}")?;
                }

                write!(f, "}}")
            }
            Value::String(string) => write!(f, r#""{}""#, string),
            Value::Builtin(_) => write!(f, "[builtin function]"),
            Value::Array(values) => {
                write!(
                    f,
                    "[{}]",
                    values
                        .iter()
                        .map(|value| value.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Value::Hash(hash) => {
                write!(
                    f,
                    "{{{}}}",
                    hash.iter()
                        .map(|(key, value)| format!("{}:{}", key, value))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Value::CompiledFunction { instructions, .. } => {
                write!(f, "CompiledFunction[{}]", instructions)
            }
            Value::Closure { fun, .. } => {
                write!(f, "Closure[{fun}]")
            }
        }
    }
}

impl Value {
    pub fn as_type(&self) -> String {
        match self {
            Value::Int(_) => "INTEGER".into(),
            Value::Bool(_) => "BOOLEAN".into(),
            Value::Null => "NULL".into(),
            Value::Return(_) => "RETURN".into(),
            Value::Let => "LET".into(),
            Value::Function { .. } => "FUNCTION".into(),
            Value::String(_) => "STRING".into(),
            Value::Builtin(_) => "BUILTIN".into(),
            Value::Array(_) => "ARRAY".into(),
            Value::Hash(_) => "HASH".into(),
            Value::CompiledFunction { .. } => "COMPILED_FUNCTION_OBJ".into(),
            Value::Closure { .. } => "CLOSURE".into(),
        }
    }
}
