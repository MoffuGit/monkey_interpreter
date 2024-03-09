use std::fmt::Display;

use crate::ast::statement::Statement;

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self
            .statements
            .iter()
            .map(|statement| statement.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{statements}")
    }
}
