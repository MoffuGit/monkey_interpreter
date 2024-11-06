use crate::code::OpCode;
use crate::compiler::ByteCode;
use crate::eval::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use self::frame::Frame;

mod frame;
#[cfg(test)]
mod tests;

pub struct VmError {
    pub msg: String,
}

impl VmError {
    pub fn new(msg: impl Into<String>) -> Self {
        VmError { msg: msg.into() }
    }
}

impl Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

const STACK_SIZE: usize = 2048_usize;
const MAX_FRAMES: usize = 1024_usize;

#[derive(Debug)]
pub struct Vm {
    constans: Vec<Value>,
    stack: Vec<Value>,
    sp: usize,
    pub last_popped_element: Option<Value>,
    globals: Rc<RefCell<Vec<Value>>>,
    frames: Vec<Frame>,
}

impl Vm {
    fn current_frame(&mut self) -> Result<&mut Frame, VmError> {
        self.frames
            .last_mut()
            .ok_or(VmError::new("the frames are empty"))
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }

    pub fn new(byte_code: ByteCode) -> Self {
        let mut frames: Vec<Frame> = Vec::with_capacity(MAX_FRAMES);
        frames.push(Frame::new(byte_code.instructions.clone(), 0));
        Vm {
            constans: byte_code.constants.clone(),
            frames,
            stack: Vec::with_capacity(STACK_SIZE),
            last_popped_element: None,
            sp: 0,
            globals: Rc::new(RefCell::new(Vec::with_capacity(65536))),
        }
    }

    pub fn new_with_global_store(byte_code: ByteCode, storage: Rc<RefCell<Vec<Value>>>) -> Self {
        let mut vm = Vm::new(byte_code);
        vm.globals = storage;
        vm
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while self.current_frame()?.ip < self.current_frame()?.function.len() {
            let ip = self.current_frame()?.ip;
            let instructions = self.current_frame()?.function.clone();
            let op = match OpCode::try_from(instructions[ip]) {
                Ok(op) => op,
                Err(_) => return Err(VmError::new("the u8 isnt a valid OpCode")),
            };

            match op {
                OpCode::OpConstant => {
                    let const_idx =
                        u16::from_be_bytes(instructions[ip + 1..ip + 3].try_into().unwrap());
                    self.current_frame()?.ip += 2;
                    self.push(self.constans[const_idx as usize].clone())?;
                }
                OpCode::OpAdd | OpCode::OpSub | OpCode::OpMul | OpCode::OpDiv => {
                    self.execute_binary_operation(op)?;
                }
                OpCode::OpPop => {
                    self.pop()?;
                }
                OpCode::OpTrue => {
                    self.push(true)?;
                }
                OpCode::OpFalse => {
                    self.push(false)?;
                }
                OpCode::OpEqual | OpCode::OpNotEqual | OpCode::OpGreatherThan => {
                    self.execute_comparision(op)?;
                }
                OpCode::OpBang => {
                    self.execute_bang_operator()?;
                }
                OpCode::OpMinus => self.execute_minus_operator()?,
                OpCode::OpJump => {
                    let position =
                        u16::from_be_bytes(instructions[ip + 1..ip + 3].try_into().unwrap());
                    self.current_frame()?.ip = (position - 1) as usize;
                }
                OpCode::OpJumpNotTruthy => {
                    let position =
                        u16::from_be_bytes(instructions[ip + 1..ip + 3].try_into().unwrap());

                    self.current_frame()?.ip += 2;
                    let condition = self.pop()?;
                    if !self.is_truthy(condition) {
                        self.current_frame()?.ip = (position - 1) as usize;
                    }
                }
                OpCode::OpNull => {
                    self.push(Value::Null)?;
                }
                OpCode::OpSetGlobal => {
                    let global_idx =
                        u16::from_be_bytes(instructions[ip + 1..ip + 3].try_into().unwrap());
                    self.current_frame()?.ip += 2;
                    let value = self.pop()?;
                    if self.globals.borrow().len() == global_idx as usize {
                        self.globals.borrow_mut().push(value);
                    } else {
                        self.globals.borrow_mut()[global_idx as usize] = value;
                    }
                }
                OpCode::OpGetGlobal => {
                    let global_idx =
                        u16::from_be_bytes(instructions[ip + 1..ip + 3].try_into().unwrap());
                    self.current_frame()?.ip += 2;
                    let value = self.globals.borrow_mut()[global_idx as usize].clone();
                    self.push(value)?;
                }
                OpCode::OpArray => {
                    let len = u16::from_be_bytes(instructions[ip + 1..ip + 3].try_into().unwrap());
                    self.current_frame()?.ip += 2;
                    let start = self.sp - len as usize;
                    let array = self.build_array(start, self.sp);
                    for _ in start..self.sp {
                        self.pop()?;
                    }
                    self.push(array)?;
                }
                OpCode::OpHash => {
                    let len = u16::from_be_bytes(instructions[ip + 1..ip + 3].try_into().unwrap());

                    self.current_frame()?.ip += 2;

                    let hash = self.build_hash(self.sp - len as usize, self.sp);

                    for _ in self.sp - len as usize..self.sp {
                        self.pop()?;
                    }

                    self.push(hash)?;
                }
                OpCode::OpIndex => {
                    let idx = self.pop()?;
                    let lhs = self.pop()?;

                    let value = self.execute_index_expression(idx, lhs);
                    self.push(value)?;
                }
                OpCode::OpCall => {
                    let num_args =
                        u8::from_be_bytes(instructions[ip + 1..ip + 2].try_into().unwrap());
                    self.current_frame()?.ip += 1;
                    self.call_funtion(num_args as usize)?;
                    continue;
                }
                OpCode::OpReturn => {
                    if let Some(frame) = self.pop_frame() {
                        for _ in 0..self.sp - frame.base_pointer {
                            self.pop()?;
                        }
                    };
                    self.push(Value::Null)?;
                }
                OpCode::OpReturnValue => {
                    let return_value = self.pop()?;
                    if let Some(frame) = self.pop_frame() {
                        for _ in 0..self.sp - frame.base_pointer {
                            self.pop()?;
                        }
                    }
                    self.pop()?;
                    self.push(return_value)?;
                }
                OpCode::OpSetLocal => {
                    let local_idx =
                        u8::from_be_bytes(instructions[ip + 1..ip + 2].try_into().unwrap());
                    self.current_frame()?.ip += 1;
                    let base_pointer = self.current_frame()?.base_pointer;
                    let value = self.pop()?;
                    self.stack[base_pointer + local_idx as usize] = value;
                }
                OpCode::OpGetLocal => {
                    let local_idx =
                        u8::from_be_bytes(instructions[ip + 1..ip + 2].try_into().unwrap());
                    self.current_frame()?.ip += 1;
                    let base_pointer = self.current_frame()?.base_pointer;
                    if let Some(value) = self.stack.get(base_pointer + local_idx as usize).cloned()
                    {
                        self.push(value)?;
                    }
                }
            };
            self.current_frame()?.ip += 1;
        }

        Ok(())
    }

    fn call_funtion(&mut self, num_args: usize) -> Result<(), VmError> {
        if let Value::CompiledFunction {
            instructions: function,
            num_locals,
            num_parameters,
        } = &self.stack[self.sp - 1 - num_args].clone()
        {
            if num_parameters != &num_args {
                return Err(VmError::new(format!(
                    "wrong number of arguments: want={}, got={}",
                    num_parameters, num_args
                )));
            }
            let frame = Frame::new(function.clone(), self.sp - num_args);
            self.push_frame(frame);
            for _ in 0..*num_locals {
                self.stack.push(Value::Null);
            }
            self.sp += num_locals;
            Ok(())
        } else {
            Err(VmError::new(format!(
                "Calling non-function: {:?}",
                &self.stack[self.sp - 1].clone(),
            )))
        }
    }

    fn execute_index_expression(&mut self, idx: Value, lhs: Value) -> Value {
        match lhs {
            Value::Array(arr) => {
                if let Value::Int(idx) = idx {
                    arr.get(idx as usize).unwrap_or(&Value::Null).clone()
                } else {
                    Value::Null
                }
            }
            Value::Hash(hash) => hash.get(&idx).unwrap_or(&Value::Null).clone(),
            _ => Value::Null,
        }
    }

    #[allow(clippy::mutable_key_type)]
    fn build_hash(&mut self, start_idx: usize, end_idx: usize) -> Value {
        let mut hash = HashMap::new();
        let mut idx = start_idx;
        while idx < end_idx {
            let key = self.stack[idx].clone();
            let value = self.stack[idx + 1].clone();

            hash.insert(key, value);

            idx += 2;
        }
        Value::Hash(hash)
    }

    fn build_array(&mut self, start_idx: usize, end_idx: usize) -> Value {
        Value::Array(
            (start_idx..end_idx)
                .map(|idx| self.stack[idx].clone())
                .collect::<Vec<Value>>(),
        )
    }

    fn is_truthy(&mut self, value: Value) -> bool {
        match value {
            Value::Bool(bool) => bool,
            Value::Null => false,
            _ => true,
        }
    }

    fn execute_minus_operator(&mut self) -> Result<(), VmError> {
        let op = self.pop()?;

        if let Value::Int(value) = op {
            self.push(-value)
        } else {
            Err(VmError::new(format!("unsupported type for negation: {op}")))
        }
    }

    fn execute_bang_operator(&mut self) -> Result<(), VmError> {
        let op = self.pop()?;
        match op {
            Value::Bool(bool) => self.push(!bool),
            Value::Null => self.push(true),
            _ => self.push(false),
        }
    }

    fn execute_comparision(&mut self, op: OpCode) -> Result<(), VmError> {
        let right = self.pop()?;
        let left = self.pop()?;

        match (right, left) {
            (Value::Int(right), Value::Int(left)) => {
                self.execute_integer_comparision(op, left, right)
            }
            (right, left) => match op {
                OpCode::OpEqual => self.push(Value::from(right == left)),
                OpCode::OpNotEqual => self.push(Value::from(right != left)),
                op => Err(VmError::new(format!(
                    "Your are using a wrong operator: {op:?}"
                ))),
            },
        }
    }

    fn execute_integer_comparision(
        &mut self,
        op: OpCode,
        left: i64,
        right: i64,
    ) -> Result<(), VmError> {
        match op {
            OpCode::OpEqual => self.push(Value::from(right == left)),
            OpCode::OpNotEqual => self.push(Value::from(right != left)),
            OpCode::OpGreatherThan => self.push(Value::from(left > right)),
            _ => Err(VmError::new("You are using the wrong operator")),
        }
    }

    fn execute_binary_operation(&mut self, op: OpCode) -> Result<(), VmError> {
        let right = self.pop()?;
        let left = self.pop()?;

        match (right, left) {
            (Value::Int(right), Value::Int(left)) => {
                self.execute_binary_integer_operation(op, right, left)
            }
            (Value::String(right), Value::String(left)) => {
                self.execute_binary_str_operation(op, &right, &left)
            }
            (right, left) => Err(VmError::new(format!(
                "unsupported values for binary operation: {} {}",
                right, left,
            ))),
        }
    }

    fn execute_binary_str_operation(
        &mut self,
        op: OpCode,
        right: &str,
        left: &str,
    ) -> Result<(), VmError> {
        if op == OpCode::OpAdd {
            self.push(format!("{}{}", left, right))
        } else {
            Err(VmError::new(
                "You only can add string, any other operatio it's invalid",
            ))
        }
    }

    fn execute_binary_integer_operation(
        &mut self,
        op: OpCode,
        right: i64,
        left: i64,
    ) -> Result<(), VmError> {
        match op {
            OpCode::OpAdd => self.push(left + right)?,
            OpCode::OpSub => self.push(left - right)?,
            OpCode::OpMul => self.push(left * right)?,
            OpCode::OpDiv => self.push(left / right)?,
            _ => unreachable!(),
        };
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, VmError> {
        if let Some(element) = self.stack.pop() {
            self.sp -= 1;
            self.last_popped_element = Some(element.clone());
            Ok(element)
        } else {
            Err(VmError::new("You try to pop on an empty stack"))
        }
    }

    fn push<V: Into<Value>>(&mut self, value: V) -> Result<(), VmError> {
        if self.sp >= STACK_SIZE {
            return Err(VmError::new("Stack Overflow"));
        }

        self.stack.push(value.into());
        self.sp += 1;
        Ok(())
    }
}
