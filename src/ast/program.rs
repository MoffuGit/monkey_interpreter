use crate::ast::statement::Statement;

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}
