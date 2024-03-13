use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::expression::Expression;

use super::environment::Environment;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Null,
    Let,
    Return(Box<Value>),
    Function {
        parameters: Vec<String>,
        body: Expression,
        env: Rc<RefCell<Environment>>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::Null => write!(f, "null"),
            Value::Return(value) => write!(f, "{value}"),
            Value::Let => write!(f, "let"),
            Value::Function { parameters, .. } => {
                write!(f, "fn ({}) {{ ... }}", parameters.join(", "))
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
        }
    }
}
