use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

use crate::ast::statement::Statement;

use super::environment::Environment;
use super::EvalError;

type BuiltinFuncion = fn(Vec<Value>) -> Result<Value, EvalError>;

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
            Value::Hash(_) => todo!(),
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
        }
    }
}
