use std::fmt::Display;

use crate::ast::operator::{InfixOperator, PrefixOperator};

use super::EvalError;

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Null,
    Return(Box<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::Null => write!(f, "null"),
            Value::Return(value) => write!(f, "{value}"),
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
        }
    }
    pub fn eval_prefix_expression(
        operator: PrefixOperator,
        rhs: Value,
    ) -> Result<Value, EvalError> {
        Ok(match operator {
            PrefixOperator::Not => Value::eval_bang(rhs)?,
            PrefixOperator::Negative => Value::eval_minus(rhs)?,
        })
    }

    pub fn eval_bang(rhs: Value) -> Result<Value, EvalError> {
        Ok(Value::Bool(match rhs {
            Value::Int(value) => value == 0,
            Value::Bool(value) => !value,
            Value::Null => true,
            value => {
                return Err(EvalError::new(format!(
                    "unknown operator: !{}",
                    value.as_type()
                )))
            }
        }))
    }

    pub fn eval_minus(rhs: Value) -> Result<Value, EvalError> {
        Ok(match rhs {
            Value::Int(value) => Value::Int(-value),
            value => {
                return Err(EvalError::new(format!(
                    "unknown operator: -{}",
                    value.as_type()
                )))
            }
        })
    }

    pub fn eval_infix_expression(
        operator: InfixOperator,
        lhs: Value,
        rhs: Value,
    ) -> Result<Value, EvalError> {
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => {
                Ok(Value::eval_int_infix_expression(operator, lhs, rhs))
            }
            (Value::Bool(lhs), Value::Bool(rhs)) => match operator {
                InfixOperator::Equal => Ok(Value::Bool(lhs == rhs)),
                InfixOperator::NotEqual => Ok(Value::Bool(lhs != rhs)),
                operator => Err(EvalError::new(format!(
                    "unknown operator: BOOLEAN {operator} BOOLEAN"
                ))),
            },
            (lhs, rhs) => Err(EvalError::new(format!(
                "type mismatch: {} {operator} {}",
                lhs.as_type(),
                rhs.as_type()
            ))),
        }
    }

    pub fn eval_int_infix_expression(operator: InfixOperator, lhs: i64, rhs: i64) -> Value {
        match operator {
            InfixOperator::Add => Value::Int(lhs + rhs),
            InfixOperator::Sub => Value::Int(lhs - rhs),
            InfixOperator::Mul => Value::Int(lhs * rhs),
            InfixOperator::Div => Value::Int(lhs / rhs),
            InfixOperator::Equal => Value::Bool(lhs == rhs),
            InfixOperator::NotEqual => Value::Bool(lhs != rhs),
            InfixOperator::GreaterThan => Value::Bool(lhs > rhs),
            InfixOperator::LessThan => Value::Bool(lhs < rhs),
            InfixOperator::Modulo => Value::Int(lhs % rhs),
            InfixOperator::GreaterThanOrEqual => Value::Bool(lhs >= rhs),
            InfixOperator::LessThanOrEqual => Value::Bool(lhs <= rhs),
        }
    }
}
