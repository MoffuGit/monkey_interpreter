#[cfg(test)]
mod tests;

use crate::ast::expression::Expression;
use crate::ast::operator::InfixOperator;
use crate::ast::program::Program;
use crate::ast::statement::Statement;
use crate::code::{Instructions, OpCode};
use crate::eval::value::Value;
use crate::{code, eval::value};
use std::fmt::Display;

#[derive(Debug)]
pub struct CompilerError {
    msg: String,
}

impl CompilerError {
    pub fn new(msg: impl Into<String>) -> Self {
        CompilerError { msg: msg.into() }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct Compiler {
    instructions: code::Instructions,
    constants: Vec<value::Value>,
}

pub struct ByteCode {
    pub instructions: code::Instructions,
    pub constants: Vec<value::Value>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Instructions(vec![]),
            constants: vec![],
        }
    }

    pub fn compile_program(&mut self, program: Program) -> Result<(), CompilerError> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<(), CompilerError> {
        match statement {
            Statement::Expression(expression) => {
                self.compile_expression(expression)?;
                self.emit(OpCode::OpPop, &[]);
                Ok(())
            }
            Statement::Let { name, value } => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Block(_) => todo!(),
        }
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<(), CompilerError> {
        match expression {
            Expression::Int(value) => {
                let int = Value::Int(value);
                let operands = vec![self.add_constant(int)];
                self.emit(OpCode::OpConstant, &operands);
            }
            Expression::Identifier(_) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Prefix { rhs, operator } => {
                self.compile_expression(*rhs)?;

                match operator {
                    crate::ast::operator::PrefixOperator::Not => self.emit(OpCode::OpBang, &[]),
                    crate::ast::operator::PrefixOperator::Negative => {
                        self.emit(OpCode::OpMinus, &[])
                    }
                };
            }
            Expression::Bool(bool) => {
                match bool {
                    true => self.emit(OpCode::OpTrue, &[]),
                    false => self.emit(OpCode::OpFalse, &[]),
                };
            }
            Expression::Infix { lhs, rhs, operator } => {
                if operator == InfixOperator::LessThan {
                    self.compile_expression(*rhs)?;
                    self.compile_expression(*lhs)?;
                    self.emit(OpCode::OpGreatherThan, &[]);
                    return Ok(());
                }
                self.compile_expression(*lhs)?;
                self.compile_expression(*rhs)?;

                match operator {
                    InfixOperator::Add => self.emit(OpCode::OpAdd, &[]),
                    InfixOperator::Sub => self.emit(OpCode::OpSub, &[]),
                    InfixOperator::Mul => self.emit(OpCode::OpMul, &[]),
                    InfixOperator::Div => self.emit(OpCode::OpDiv, &[]),
                    InfixOperator::Equal => self.emit(OpCode::OpEqual, &[]),
                    InfixOperator::NotEqual => self.emit(OpCode::OpNotEqual, &[]),
                    InfixOperator::GreaterThan => self.emit(OpCode::OpGreatherThan, &[]),
                    _ => unreachable!(),
                };
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => todo!(),
            Expression::Fn { parameters, body } => todo!(),
            Expression::Call {
                function,
                arguments,
            } => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Index { lhs, index } => todo!(),
            Expression::Hash(_) => todo!(),
        };
        Ok(())
    }

    fn add_constant(&mut self, value: Value) -> u64 {
        self.constants.push(value);
        self.constants.len() as u64 - 1
    }

    fn emit(&mut self, op: OpCode, operands: &[u64]) -> u64 {
        let instruction = code::make(op, operands);
        self.add_instruction(instruction)
    }

    fn add_instruction(&mut self, instructions: Instructions) -> u64 {
        let position_new_instruction = self.instructions.0.len() as u64;
        self.instructions.0.extend(instructions.0.iter());
        position_new_instruction
    }

    pub fn bytecode(&self) -> ByteCode {
        ByteCode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
