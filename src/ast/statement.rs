use std::fmt::Display;

use super::expression::Expression;

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum Statement {
    Expression(Expression),
    Let { name: String, value: Expression },
    Return(Expression),
    Block(Vec<Statement>),
}

impl Statement {
    pub fn r#let(name: impl Into<String>, value: impl Into<Expression>) -> Self {
        Statement::Let {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(value) => write!(f, "{}", value),
            Statement::Let { name, value } => write!(f, "Let {name} = {value}"),
            Statement::Return(value) => write!(f, "Return {}", value),
            Statement::Block(statements) => {
                for statement in statements {
                    write!(f, "{statement}")?;
                }

                Ok(())
            }
        }
    }
}
