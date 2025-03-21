use crate::ast::operator::*;
use std::fmt::Display;

use super::statement::Statement;

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum Expression {
    Int(i64),
    Identifier(String),
    String(String),
    Prefix {
        rhs: Box<Expression>,
        operator: PrefixOperator,
    },
    Bool(bool),
    Infix {
        lhs: Box<Expression>,
        operator: InfixOperator,
        rhs: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    Fn {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Array(Vec<Expression>),
    Index {
        lhs: Box<Expression>,
        index: Box<Expression>,
    },
    Hash(Vec<(Expression, Expression)>),
}

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        Self::Identifier(value.to_string())
    }
}

impl From<bool> for Expression {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Bool(value) => write!(f, "{}", value),
            Expression::Int(value) => write!(f, "{}", value),
            Expression::Identifier(value) => write!(f, "{}", value),
            Expression::Prefix { rhs, operator } => write!(f, "({operator}{rhs})"),
            Expression::Infix { lhs, operator, rhs } => write!(f, "({lhs} {operator} {rhs})"),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if {condition} {{")?;

                for statement in consequence {
                    write!(f, "{statement}")?;
                }

                write!(f, "}}")?;

                if let Some(statements) = alternative {
                    for statement in statements {
                        write!(f, "{statement}")?
                    }
                }
                write!(f, "}}")
            }
            Expression::Fn {
                name,
                parameters,
                body,
            } => {
                write!(f, "fn {} ({}) {{", name, parameters.join(", "))?;

                for statement in body {
                    write!(f, "{statement}")?;
                }

                write!(f, "}}")
            }
            Expression::Call {
                function,
                arguments,
            } => {
                write!(
                    f,
                    "{function}({})",
                    arguments
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Expression::String(string) => write!(f, "{}", string),
            Expression::Array(expressions) => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    expressions
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                write!(f, "]")
            }
            Expression::Index { lhs, index } => {
                write!(f, "({}[{}])", lhs, index)
            }
            Expression::Hash(hash) => {
                write!(f, "{{")?;
                for (k, v) in hash {
                    write!(f, "{k} => {v},")?;
                }
                write!(f, "}}")
            }
        }
    }
}
