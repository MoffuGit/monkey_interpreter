use crate::code::Instructions;
use crate::eval::value::Value;

#[derive(Clone, Debug)]
pub struct Frame {
    pub cl: Value,
    pub ip: usize,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(cl: Value, base_pointer: usize) -> Self {
        Frame {
            cl,
            ip: 0,
            base_pointer,
        }
    }

    pub fn instructions(&mut self) -> Instructions {
        if let Value::Closure { fun, .. } = &self.cl {
            if let Value::CompiledFunction { instructions, .. } = *fun.clone() {
                instructions
            } else {
                panic!("this shoudl be a CompiledFunction")
            }
        } else {
            panic!("this shoudl be a Closure")
        }
    }
}
