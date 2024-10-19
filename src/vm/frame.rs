use crate::code::Instructions;

#[derive(Clone, Debug)]
pub struct Frame {
    pub function: Instructions,
    pub ip: usize,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(function: Instructions, base_pointer: usize) -> Self {
        Frame {
            function,
            ip: 0,
            base_pointer,
        }
    }
}
