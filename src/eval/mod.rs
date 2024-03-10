use crate::ast::operator::{InfixOperator, PrefixOperator};
use std::fmt::Display;

use crate::ast::expression::Expression;
use crate::ast::program::Program;
use crate::ast::statement::Statement;

use self::value::Value;

pub mod value;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct EvalError {
    msg: String,
}

impl EvalError {
    pub fn new(msg: impl Into<String>) -> Self {
        EvalError { msg: msg.into() }
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct Eval {}

impl Eval {
    pub fn eval_prefix_expression(
        operator: PrefixOperator,
        rhs: Value,
    ) -> Result<Value, EvalError> {
        Ok(match operator {
            PrefixOperator::Not => Eval::eval_bang(rhs)?,
            PrefixOperator::Negative => Eval::eval_minus(rhs)?,
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
                Ok(Eval::eval_int_infix_expression(operator, lhs, rhs))
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

pub trait EvalTrait {
    fn eval(self) -> Result<Value, EvalError>;
}
impl EvalTrait for Program {
    fn eval(self) -> Result<Value, EvalError> {
        let mut value = Value::Null;

        for statement in self.statements {
            value = statement.eval()?;

            if let Value::Return(value) = value {
                return Ok(*value);
            }
        }
        Ok(value)
    }
}

impl EvalTrait for Statement {
    fn eval(self) -> Result<Value, EvalError> {
        match self {
            Statement::Expression(expression) => expression.eval(),
            Statement::Let { name: _, value: _ } => todo!(),
            Statement::Return(expression) => Ok(Value::Return(Box::new(expression.eval()?))),
            Statement::Block(statements) => {
                let mut value = Value::Null;

                for statement in statements {
                    value = statement.eval()?;
                    if let Value::Return(_) = value {
                        return Ok(value);
                    }
                }
                Ok(value)
            }
        }
    }
}

impl EvalTrait for Expression {
    fn eval(self) -> Result<Value, EvalError> {
        match self {
            Expression::Int(value) => Ok(Value::Int(value)),
            Expression::Bool(value) => Ok(Value::Bool(value)),
            Expression::Prefix { rhs, operator } => {
                let rhs = rhs.eval()?;
                Eval::eval_prefix_expression(operator, rhs)
            }
            Expression::Infix { lhs, operator, rhs } => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                Eval::eval_infix_expression(operator, lhs, rhs)
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition = match condition.eval()? {
                    Value::Bool(value) => value,
                    Value::Int(value) => value != 0,
                    condition => {
                        return Err(EvalError::new(format!(
                            "expected bool condition, got: {condition}"
                        )))
                    }
                };

                if condition {
                    Statement::Block(consequence).eval()
                } else {
                    alternative.map_or(Ok(Value::Null), |statements| {
                        Statement::Block(statements).eval()
                    })
                }
            }
            _ => Ok(Value::Null),
        }
    }
}
