use std::fmt::Display;
use std::ops::{Index, Range, RangeFrom};

#[cfg(test)]
mod tests;
#[derive(Debug, Clone, PartialEq)]
pub struct Instructions(pub Vec<u8>);

impl Instructions {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for Instructions {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<RangeFrom<usize>> for Instructions {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Range<usize>> for Instructions {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_intruction(definition: Definition, operands: &[i64]) -> String {
            match operands.len() {
                0 => definition.name.to_string(),
                1 => format!("{} {}", definition.name, operands[0]),
                _ => unreachable!(),
            }
        }
        let mut idx = 0;
        while idx < self.0.len() {
            let definition: Definition = match std::convert::TryInto::<OpCode>::try_into(self[idx])
            {
                Ok(op) => op.into(),
                Err(_) => {
                    write!(f, "Error: {} is not a valid OpCode", self[idx])?;
                    idx += 1;
                    continue;
                }
            };
            let (operands, read) = read_operands(&definition, self[idx + 1..].to_vec());
            writeln!(f, "{:04} {}", idx, fmt_intruction(definition, &operands))?;
            idx += 1 + read as usize;
        }
        Ok(())
    }
}

pub fn concat_instructions(instructions: &[Instructions]) -> Instructions {
    Instructions(
        instructions
            .iter()
            .map(|instruction| instruction.0.clone())
            .collect::<Vec<Vec<u8>>>()
            .concat(),
    )
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    OpConstant = 0,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPop,
    OpTrue,
    OpFalse,
    OpEqual,
    OpNotEqual,
    OpGreatherThan,
    OpMinus,
    OpBang,
    OpJumpNotTruthy,
    OpJump,
    OpNull,
    OpGetGlobal,
    OpSetGlobal,
    OpArray,
    OpHash,
    OpIndex,
}

#[derive(Debug)]
pub struct Definition {
    name: String,
    operand_widths: Vec<u8>,
}

impl Definition {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Definition {
            name: name.into(),
            operand_widths: vec![],
        }
    }

    pub fn width(self, width: Vec<u8>) -> Self {
        Definition {
            name: self.name,
            operand_widths: width,
        }
    }
}

impl From<OpCode> for Definition {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::OpConstant => Definition::new("OpConstant").width(vec![2]),
            OpCode::OpAdd => Definition::new("OpAdd"),
            OpCode::OpPop => Definition::new("OpPop"),
            OpCode::OpSub => Definition::new("OpSub"),
            OpCode::OpMul => Definition::new("OpMul"),
            OpCode::OpDiv => Definition::new("OpDiv"),
            OpCode::OpTrue => Definition::new("OpTrue"),
            OpCode::OpFalse => Definition::new("OpFalse"),
            OpCode::OpEqual => Definition::new("OpEqual"),
            OpCode::OpNotEqual => Definition::new("OpNotEqual"),
            OpCode::OpGreatherThan => Definition::new("OpGreatherThan"),
            OpCode::OpMinus => Definition::new("OpMinus"),
            OpCode::OpBang => Definition::new("OpBang"),
            OpCode::OpJumpNotTruthy => Definition::new("OpJumpNotTruthy").width(vec![2]),
            OpCode::OpJump => Definition::new("OpJump").width(vec![2]),
            OpCode::OpNull => Definition::new("OpNull"),
            OpCode::OpGetGlobal => Definition::new("OpGetGlobal").width(vec![2]),
            OpCode::OpSetGlobal => Definition::new("OpSetGlobal").width(vec![2]),
            OpCode::OpArray => Definition::new("OpArray").width(vec![2]),
            OpCode::OpHash => Definition::new("OpHash").width(vec![2]),
            OpCode::OpIndex => Definition::new("OpIndex"),
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => OpCode::OpConstant,
            1 => OpCode::OpAdd,
            2 => OpCode::OpSub,
            3 => OpCode::OpMul,
            4 => OpCode::OpDiv,
            5 => OpCode::OpPop,
            6 => OpCode::OpTrue,
            7 => OpCode::OpFalse,
            8 => OpCode::OpEqual,
            9 => OpCode::OpNotEqual,
            10 => OpCode::OpGreatherThan,
            11 => OpCode::OpMinus,
            12 => OpCode::OpBang,
            13 => OpCode::OpJumpNotTruthy,
            14 => OpCode::OpJump,
            15 => OpCode::OpNull,
            16 => OpCode::OpGetGlobal,
            17 => OpCode::OpSetGlobal,
            18 => OpCode::OpArray,
            19 => OpCode::OpHash,
            20 => OpCode::OpIndex,
            _ => return Err(()),
        })
    }
}

pub fn read_operands(definition: &Definition, instruction: Vec<u8>) -> (Vec<i64>, u8) {
    let mut operands = vec![];
    let mut offset = 0_u8;

    for width in &definition.operand_widths {
        match width {
            2 => {
                let instruction = instruction[offset as usize..(offset + 2) as usize]
                    .try_into()
                    .unwrap();
                operands.push(u16::from_be_bytes(instruction) as i64);
                offset += 2
            }
            _ => unreachable!(),
        }
    }
    (operands.to_vec(), offset)
}

pub fn make(op: OpCode, operands: &[i64]) -> Instructions {
    let definition: Definition = op.into();
    let mut instruction = vec![];
    instruction.push(op as u8);
    for (idx, operand) in operands.iter().enumerate() {
        let width = definition.operand_widths[idx];
        match width {
            2 => {
                let bytes = (*operand as u16).to_be_bytes();
                instruction.extend(bytes.iter());
            }
            _ => unreachable!(),
        }
    }
    Instructions(instruction)
}
