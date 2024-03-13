use crate::ast::operator::{InfixOperator, PrefixOperator};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::expression::Expression;
use crate::ast::program::Program;
use crate::ast::statement::Statement;

use self::environment::Environment;
use self::value::Value;

pub mod environment;
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

#[derive()]
pub struct Eval {
    pub env: Rc<RefCell<Environment>>,
}

impl Eval {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Eval { env }
    }
    pub fn eval_program(&mut self, program: Program) -> Result<Value, EvalError> {
        let mut value = Value::Null;

        for statement in program.statements {
            value = self.eval_statement(statement)?;

            if let Value::Return(value) = value {
                return Ok(*value);
            }
        }
        Ok(value)
    }

    pub fn eval_statement(&mut self, statement: Statement) -> Result<Value, EvalError> {
        match statement {
            Statement::Expression(expression) => self.eval_expression(expression),
            Statement::Let { name, value } => {
                let value = self.eval_expression(value)?;
                self.env.borrow_mut().store.insert(name, value.clone());
                Ok(Value::Let)
            }
            Statement::Return(expression) => {
                Ok(Value::Return(Box::new(self.eval_expression(expression)?)))
            }
            Statement::Block(statements) => {
                let mut value = Value::Null;

                for statement in statements {
                    value = self.eval_statement(statement)?;
                    if let Value::Return(_) = value {
                        return Ok(value);
                    }
                }
                Ok(value)
            }
        }
    }

    pub fn eval_expression(&mut self, expression: Expression) -> Result<Value, EvalError> {
        match expression {
            Expression::Int(value) => Ok(Value::Int(value)),
            Expression::Bool(value) => Ok(Value::Bool(value)),
            Expression::Prefix { rhs, operator } => {
                let rhs = self.eval_expression(*rhs)?;
                self.eval_prefix_expression(operator, rhs)
            }
            Expression::Infix { lhs, operator, rhs } => {
                let lhs = self.eval_expression(*lhs)?;
                let rhs = self.eval_expression(*rhs)?;
                self.eval_infix_expression(operator, lhs, rhs)
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition = match self.eval_expression(*condition)? {
                    Value::Bool(value) => value,
                    Value::Int(value) => value != 0,
                    condition => {
                        return Err(EvalError::new(format!(
                            "expected bool condition, got: {condition}"
                        )))
                    }
                };

                if condition {
                    self.eval_statement(Statement::Block(consequence))
                } else {
                    alternative.map_or(Ok(Value::Null), |statements| {
                        self.eval_statement(Statement::Block(statements))
                    })
                }
            }
            Expression::Identifier(name) => match self.env.borrow_mut().store.get(&name) {
                Some(value) => Ok(value.clone()),
                None => Err(EvalError::new(format!("identifier not found: {}", name))),
            },
            _ => Ok(Value::Null),
        }
    }

    pub fn eval_prefix_expression(
        &self,
        operator: PrefixOperator,
        rhs: Value,
    ) -> Result<Value, EvalError> {
        Ok(match operator {
            PrefixOperator::Not => self.eval_bang(rhs)?,
            PrefixOperator::Negative => self.eval_minus(rhs)?,
        })
    }

    pub fn eval_bang(&self, rhs: Value) -> Result<Value, EvalError> {
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

    pub fn eval_minus(&self, rhs: Value) -> Result<Value, EvalError> {
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
        &self,
        operator: InfixOperator,
        lhs: Value,
        rhs: Value,
    ) -> Result<Value, EvalError> {
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => {
                Ok(self.eval_int_infix_expression(operator, lhs, rhs))
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

    pub fn eval_int_infix_expression(&self, operator: InfixOperator, lhs: i64, rhs: i64) -> Value {
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
