use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::statement::Statement;

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
        body: Vec<Statement>,
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
            Value::Function {
                parameters, body, ..
            } => {
                write!(f, "fn ({}) {{", parameters.join(", "))?;

                for statement in body {
                    write!(f, "{statement}")?;
                }

                write!(f, "}}")
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
