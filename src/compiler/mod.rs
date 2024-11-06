pub mod symbol_table;
#[cfg(test)]
mod symbol_table_test;
#[cfg(test)]
mod tests;

use crate::ast::expression::Expression;
use crate::ast::operator::InfixOperator;
use crate::ast::program::Program;
use crate::ast::statement::Statement;
use crate::code::{concat_instructions, make, Instructions, OpCode};
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
    constants: Rc<RefCell<Vec<value::Value>>>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    scopes: Vec<CompilationScope>,
    scope_idx: usize,
}

#[derive(Default)]
pub struct CompilationScope {
    instructions: Instructions,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
}

pub struct ByteCode {
    pub instructions: code::Instructions,
    pub constants: Vec<value::Value>,
}

#[derive(Clone, Debug)]
pub struct EmittedInstruction {
    op: OpCode,
    position: usize,
}

impl Compiler {
    pub fn current_scope(&mut self) -> &mut CompilationScope {
        &mut self.scopes[self.scope_idx]
    }
    pub fn leave_scope(&mut self) -> Instructions {
        let instructions = self.current_instructions();
        let outer_symbol_table = self
            .symbol_table
            .borrow()
            .outer
            .clone()
            .expect("should exist an outer symbol table");
        self.symbol_table = outer_symbol_table;
        self.scopes.pop();
        self.scope_idx -= 1;
        instructions
    }
    pub fn enter_scope(&mut self) {
        let scope = CompilationScope::default();
        let enclosed_symbol_table = SymbolTable::new_with_enclosed(self.symbol_table.clone());
        self.symbol_table = Rc::new(RefCell::new(enclosed_symbol_table));
        self.scopes.push(scope);
        self.scope_idx += 1;
    }
    pub fn new() -> Self {
        Compiler {
            constants: Rc::new(RefCell::new(vec![])),
            symbol_table: Rc::new(RefCell::new(SymbolTable::new())),
            scope_idx: 0,
            scopes: vec![CompilationScope::default()],
        }
    }

    pub fn current_instructions(&mut self) -> Instructions {
        self.current_scope().instructions.clone()
    }

    pub fn add_instruction(&mut self, instructions: Instructions) -> usize {
        let position_new_instruction = self.current_instructions().len();
        let updated_instuctions = concat_instructions(&[self.current_instructions(), instructions]);

        self.current_scope().instructions = updated_instuctions;
        position_new_instruction
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
                let scope = match symbol.scope {
                    symbol_table::SymbolScope::GlobalScope => OpCode::OpSetGlobal,
                    symbol_table::SymbolScope::LocalScope => OpCode::OpSetLocal,
                };
                self.emit(scope, &[symbol.index as i64]);
            }
            Statement::Return(expression) => {
                self.compile_expression(expression)?;
                self.emit(OpCode::OpReturnValue, &[]);
            }
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
                if let Some(symbol) = symbol {
                    let scope = match symbol.scope {
                        symbol_table::SymbolScope::GlobalScope => OpCode::OpGetGlobal,
                        symbol_table::SymbolScope::LocalScope => OpCode::OpGetLocal,
                    };
                    self.emit(scope, &[symbol.index as i64]);
                } else {
                    return Err(CompilerError::new(format!("undefined variable: {}", name)));
                };
            }
            Expression::String(value) => {
                let string = Value::String(value);
                let operands = vec![self.add_constant(string)];
                self.emit(OpCode::OpConstant, &operands);
            }
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
                if self.last_instruction_is(OpCode::OpPop) {
                    self.remove_last_pop();
                }

                let jump_pos = self.emit(OpCode::OpJump, &[9999]);
                let after_consequence_pos = self.current_scope().instructions.len();
                self.change_operand(jump_not_truthy_pos, &[after_consequence_pos as i64]);

                if let Some(alternative) = alternative {
                    self.compile_statement(Statement::Block(alternative))?;
                    if self.last_instruction_is(OpCode::OpPop) {
                        self.remove_last_pop();
                    }
                } else {
                    self.emit(OpCode::OpNull, &[]);
                }
                let after_aternative_pos = self.current_scope().instructions.len();
                self.change_operand(jump_pos, &[after_aternative_pos as i64]);
            }
            Expression::Fn { parameters, body } => {
                self.enter_scope();
                let num_parameters = parameters.len();

                for parameter in parameters {
                    self.symbol_table.borrow_mut().define(parameter);
                }

                self.compile_statement(Statement::Block(body))?;
                if self.last_instruction_is(OpCode::OpPop) {
                    self.replace_last_pop_with_return();
                }
                if !self.last_instruction_is(OpCode::OpReturnValue) {
                    self.emit(OpCode::OpReturn, &[]);
                }
                let num_locals = self.symbol_table.borrow().num_definitions;
                let instructions = self.leave_scope();
                let compiled_fn = Value::CompiledFunction {
                    instructions,
                    num_locals,
                    num_parameters,
                };
                let operands = self.add_constant(compiled_fn);
                self.emit(OpCode::OpConstant, &[operands]);
            }
            Expression::Call {
                function,
                arguments,
            } => {
                self.compile_expression(*function)?;

                let arguments_len = arguments.len();
                for argument in arguments {
                    self.compile_expression(argument)?;
                }
                self.emit(OpCode::OpCall, &[arguments_len as i64]);
            }
            Expression::Array(values) => {
                let len = values.len();
                for value in values {
                    self.compile_expression(value)?;
                }

                self.emit(OpCode::OpArray, &[len.try_into().unwrap()]);
            }
            Expression::Index { lhs, index } => {
                self.compile_expression(*lhs)?;
                self.compile_expression(*index)?;
                self.emit(OpCode::OpIndex, &[]);
            }
            Expression::Hash(values) => {
                let mut len = 0;
                for (key, value) in values {
                    self.compile_expression(key)?;
                    self.compile_expression(value)?;
                    len += 2;
                }
                self.emit(OpCode::OpHash, &[len.into()]);
            }
        };
        Ok(())
    }

    fn replace_last_pop_with_return(&mut self) {
        if let Some(last_instruction) = &mut self.current_scope().last_instruction {
            let last_position = last_instruction.position;
            last_instruction.op = OpCode::OpReturnValue;
            self.replace_instruction(last_position, make(OpCode::OpReturnValue, &[]));
        }
    }

    fn replace_instruction(&mut self, position: usize, new_instruction: Instructions) {
        let mut idx = 0;
        while idx < new_instruction.0.len() {
            self.current_scope().instructions.0[position + idx] = new_instruction.0[idx];
            idx += 1;
        }
    }

    fn change_operand(&mut self, op_position: usize, operand: &[i64]) {
        let op = OpCode::try_from(self.current_scope().instructions[op_position]);
        if op.is_err() {
            panic!("your instruction become invalid")
        }
        let new_instruction = code::make(op.unwrap(), operand);

        self.replace_instruction(op_position, new_instruction);
    }

    fn last_instruction_is(&mut self, op: OpCode) -> bool {
        if self.current_instructions().is_empty() {
            return false;
        }

        self.current_scope()
            .last_instruction
            .as_ref()
            .is_some_and(|last_instruction| last_instruction.op == op)
    }

    fn remove_last_pop(&mut self) {
        let position = self
            .current_scope()
            .last_instruction
            .clone()
            .unwrap()
            .position;
        let instructions = self.current_scope().instructions[..position].to_vec();
        self.current_scope().instructions = Instructions(instructions);
        self.current_scope().last_instruction = self.current_scope().previous_instruction.clone();
    }

    fn add_constant(&mut self, value: Value) -> i64 {
        self.constants.borrow_mut().push(value);
        self.constants.borrow().len() as i64 - 1
    }

    fn emit(&mut self, op: OpCode, operands: &[i64]) -> usize {
        let instruction = code::make(op, operands);
        let position = self.add_instruction(instruction);
        self.set_last_instruction(op, position);
        position
    }

    fn set_last_instruction(&mut self, op: OpCode, position: usize) {
        let previous = &self.current_scope().last_instruction;
        let last = Some(EmittedInstruction { op, position });
        self.current_scope().previous_instruction = previous.clone();
        self.current_scope().last_instruction = last;
    }

    pub fn bytecode(&mut self) -> ByteCode {
        ByteCode {
            instructions: self.current_scope().instructions.clone(),
            constants: self.constants.borrow().clone(),
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
