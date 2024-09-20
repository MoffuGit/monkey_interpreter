pub mod symbol_table;
#[cfg(test)]
mod symbol_table_test;
#[cfg(test)]
mod tests;

use crate::ast::expression::Expression;
use crate::ast::operator::InfixOperator;
use crate::ast::program::Program;
use crate::ast::statement::Statement;
use crate::code::{Instructions, OpCode};
use crate::eval::value::Value;
use crate::{code, eval::value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use self::symbol_table::SymbolTable;

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
    constants: Rc<RefCell<Vec<value::Value>>>,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
    symbol_table: Rc<RefCell<SymbolTable>>,
}

pub struct ByteCode {
    pub instructions: code::Instructions,
    pub constants: Vec<value::Value>,
}

#[derive(Clone)]
pub struct EmittedInstruction {
    op: OpCode,
    position: i64,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Instructions(vec![]),
            constants: Rc::new(RefCell::new(vec![])),
            last_instruction: None,
            previous_instruction: None,
            symbol_table: Rc::new(RefCell::new(SymbolTable::new())),
        }
    }

    pub fn new_with_state(
        symbol_table: Rc<RefCell<SymbolTable>>,
        constatns: Rc<RefCell<Vec<Value>>>,
    ) -> Self {
        let mut compiler = Compiler::new();
        compiler.symbol_table = symbol_table;
        compiler.constants = constatns;
        compiler
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
            }
            Statement::Let { name, value } => {
                self.compile_expression(value)?;

                let symbol = self.symbol_table.borrow_mut().define(name);
                self.emit(OpCode::OpSetGlobal, &[symbol.index]);
            }
            Statement::Return(_) => todo!(),
            Statement::Block(statements) => {
                for statement in statements {
                    self.compile_statement(statement)?;
                }
            }
        };
        Ok(())
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<(), CompilerError> {
        match expression {
            Expression::Int(value) => {
                let int = Value::Int(value);
                let operands = vec![self.add_constant(int)];
                self.emit(OpCode::OpConstant, &operands);
            }
            Expression::Identifier(name) => {
                let symbol = self.symbol_table.borrow_mut().resolve(&name);
                if symbol.is_none() {
                    return Err(CompilerError::new(format!("undefined variable: {}", name)));
                }
                let symbol = symbol.unwrap();
                self.emit(OpCode::OpGetGlobal, &[symbol.index]);
            }
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
            } => {
                self.compile_expression(*condition)?;
                let jump_not_truthy_pos = self.emit(OpCode::OpJumpNotTruthy, &[9999]);
                self.compile_statement(Statement::Block(consequence))?;
                if self.last_instruction_is_pop() {
                    self.remove_last_pop();
                }

                let jump_pos = self.emit(OpCode::OpJump, &[9999]);
                let after_consequence_pos = self.instructions.len();
                self.change_operand(jump_not_truthy_pos, &[after_consequence_pos as i64]);

                if let Some(alternative) = alternative {
                    self.compile_statement(Statement::Block(alternative))?;
                    if self.last_instruction_is_pop() {
                        self.remove_last_pop();
                    }
                } else {
                    self.emit(OpCode::OpNull, &[]);
                }
                let after_aternative_pos = self.instructions.len();
                self.change_operand(jump_pos, &[after_aternative_pos as i64]);
            }
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

    fn replace_instruction(&mut self, position: i64, new_instruction: Instructions) {
        let mut idx = 0;
        while idx < new_instruction.0.len() {
            self.instructions.0[position as usize + idx] = new_instruction.0[idx];
            idx += 1;
        }
    }

    fn change_operand(&mut self, op_position: i64, operand: &[i64]) {
        let op = OpCode::try_from(self.instructions[op_position as usize]);
        if op.is_err() {
            panic!("your instruction become invalid")
        }
        let new_instruction = code::make(op.unwrap(), operand);

        self.replace_instruction(op_position, new_instruction);
    }

    fn last_instruction_is_pop(&mut self) -> bool {
        if let Some(last) = &self.last_instruction {
            last.op == OpCode::OpPop
        } else {
            false
        }
    }

    fn remove_last_pop(&mut self) {
        self.instructions = Instructions(
            self.instructions.0[..self.last_instruction.clone().unwrap().position as usize]
                .to_vec(),
        );
        self.last_instruction = self.previous_instruction.clone();
    }

    fn add_constant(&mut self, value: Value) -> i64 {
        self.constants.borrow_mut().push(value);
        self.constants.borrow_mut().len() as i64 - 1
    }

    fn emit(&mut self, op: OpCode, operands: &[i64]) -> i64 {
        let instruction = code::make(op, operands);
        let position = self.add_instruction(instruction);
        self.set_last_instruction(op, position);
        position
    }

    fn set_last_instruction(&mut self, op: OpCode, position: i64) {
        let previous = self.last_instruction.clone();
        let last = Some(EmittedInstruction { op, position });
        self.previous_instruction = previous;
        self.last_instruction = last;
    }

    fn add_instruction(&mut self, instructions: Instructions) -> i64 {
        let position_new_instruction = self.instructions.0.len() as i64;
        self.instructions.0.extend(instructions.0.iter());
        position_new_instruction
    }

    pub fn bytecode(&self) -> ByteCode {
        ByteCode {
            instructions: self.instructions.clone(),
            constants: self.constants.borrow_mut().clone(),
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
