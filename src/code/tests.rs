use super::*;
#[test]
fn test_make() {
    struct Test {
        op: OpCode,
        operands: Vec<i64>,
        expected: Instructions,
    }
    let tests = vec![
        Test {
            op: OpCode::OpConstant,
            operands: vec![65534],
            expected: Instructions(vec![OpCode::OpConstant as u8, 255, 254]),
        },
        Test {
            op: OpCode::OpAdd,
            operands: vec![],
            expected: Instructions(vec![OpCode::OpAdd as u8]),
        },
        Test {
            op: OpCode::OpGetLocal,
            operands: vec![255],
            expected: Instructions(vec![OpCode::OpGetLocal as u8, 255]),
        },
    ];

    for test in tests {
        let instruction = make(test.op, &test.operands);
        assert_eq!(instruction, test.expected);
    }
}

#[test]
pub fn test_instruction_string() {
    let instructions = vec![
        make(OpCode::OpAdd, &[]),
        make(OpCode::OpGetLocal, &[1]),
        make(OpCode::OpConstant, &[2]),
        make(OpCode::OpConstant, &[65535]),
    ];

    let expected = r#"0000 OpAdd 0001 OpGetLocal 1 0003 OpConstant 2 0006 OpConstant 65535 "#;

    assert_eq!(concat_instructions(&instructions).to_string(), expected)
}

#[test]
pub fn test_read_operands() {
    struct Test {
        op: OpCode,
        operands: Vec<i64>,
        bytes_read: usize,
    }

    let tests = [
        Test {
            op: OpCode::OpConstant,
            operands: vec![65535],
            bytes_read: 2,
        },
        Test {
            op: OpCode::OpGetLocal,
            operands: vec![255],
            bytes_read: 1,
        },
    ];

    for test in tests.iter() {
        let intruction = make(test.op, &test.operands);
        let definition: Definition = test.op.into();
        let (operands_read, n) = read_operands(&definition, intruction.0[1..].to_vec());
        assert_eq!(test.bytes_read, n);
        assert_eq!(test.operands, operands_read);
    }
}
