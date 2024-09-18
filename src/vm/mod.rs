use crate::code;
use crate::code::OpCode;
use crate::compiler::ByteCode;
use crate::eval::value::Value;
use std::fmt::Display;

#[cfg(test)]
mod tests;

const NULL: Value = Value::Null;

pub struct VmError {
    msg: String,
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

pub struct Vm {
    constans: Vec<Value>,
    instructions: code::Instructions,
    stack: Vec<Value>,
    sp: usize,
    pub last_popped_element: Option<Value>,
}

impl Vm {
    pub fn new(byte_code: ByteCode) -> Self {
        Vm {
            constans: byte_code.constants.clone(),
            instructions: byte_code.instructions.clone(),
            stack: Vec::with_capacity(STACK_SIZE),
            last_popped_element: None,
            sp: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        let mut ip = 0;

        while ip < self.instructions.len() {
            let op = match OpCode::try_from(self.instructions[ip]) {
                Ok(op) => op,
                Err(_) => return Err(VmError::new("the u8 isnt a valid OpCode")),
            };

            match op {
                OpCode::OpConstant => {
                    let const_idx =
                        u16::from_be_bytes(self.instructions[ip + 1..ip + 3].try_into().unwrap());
                    ip += 2;
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
                        u16::from_be_bytes(self.instructions[ip + 1..ip + 3].try_into().unwrap());
                    ip = (position - 1) as usize;
                }
                OpCode::OpJumpNotTruthy => {
                    let position =
                        u16::from_be_bytes(self.instructions[ip + 1..ip + 3].try_into().unwrap());

                    ip += 2;
                    let condition = self.pop()?;
                    if !self.is_truthy(condition) {
                        ip = (position - 1) as usize;
                    }
                }
                OpCode::OpNull => {
                    self.push(Value::Null)?;
                }
                _ => (),
            };
            ip += 1;
        }

        Ok(())
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
            (right, left) => Err(VmError::new(format!(
                "unsupported values for binary operation: {} {}",
                right, left,
            ))),
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
