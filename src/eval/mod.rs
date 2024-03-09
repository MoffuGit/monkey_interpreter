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

pub trait Eval {
    fn eval(self) -> Result<Value, EvalError>;
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Eval for Program {
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

impl Eval for Statement {
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

impl Eval for Expression {
    fn eval(self) -> Result<Value, EvalError> {
        match self {
            Expression::Int(value) => Ok(Value::Int(value)),
            Expression::Bool(value) => Ok(Value::Bool(value)),
            Expression::Prefix { rhs, operator } => {
                let rhs = rhs.eval()?;
                Value::eval_prefix_expression(operator, rhs)
            }
            Expression::Infix { lhs, operator, rhs } => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                Value::eval_infix_expression(operator, lhs, rhs)
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
