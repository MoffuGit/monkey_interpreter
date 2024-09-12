use crate::ast::program::Program;
use crate::code::{self, concat_instructions, OpCode};
use crate::lexer::Lexer;
use crate::parser::Parser;

use super::Compiler;

struct CompilerTestCase {
    input: String,
    expected_constants: Vec<u8>,
    expected_instructions: Vec<code::Instructions>,
}

impl CompilerTestCase {
    pub fn new<S: Into<String>>(
        input: S,
        expected_constants: &[u8],
        expected_instructions: &[(OpCode, &[u64])],
    ) -> Self {
        CompilerTestCase {
            input: input.into(),
            expected_constants: expected_constants.to_vec(),
            expected_instructions: expected_instructions
                .iter()
                .map(|(op, operands)| code::make(*op, operands))
                .collect(),
        }
    }
}

fn parse(input: String) -> Program {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}

fn run_compiler_test(tests: &[CompilerTestCase]) {
    for test in tests.iter() {
        let program = parse(test.input.clone());
        let mut compiler = Compiler::new();

        if compiler.compile_program(program).is_err() {
            panic!("Compile pgram fail");
        };

        let byte_code = compiler.bytecode();

        assert_eq!(
            byte_code.instructions.to_string(),
            concat_instructions(&test.expected_instructions).to_string()
        );
        assert_eq!(&byte_code.constants.len(), &test.expected_constants.len());

        for (idx, constant) in byte_code.constants.iter().enumerate() {
            match constant {
                crate::eval::value::Value::Int(value) => {
                    assert_eq!(*value as u8, test.expected_constants[idx])
                }
                _ => unreachable!(),
            }
        }
    }
}

#[test]
pub fn test_integer_arithmetic() {
    let tests = &[
        CompilerTestCase::new(
            "1 + 2",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpAdd, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "1; 2",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpPop, &[]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "1 - 2",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpSub, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "1 * 2",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpMul, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "2 / 1",
            &[2, 1],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpDiv, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "-1",
            &[1],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpMinus, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];
    run_compiler_test(tests);
}

#[test]
pub fn test_boolean_expression() {
    let tests = &[
        CompilerTestCase::new("true", &[], &[(OpCode::OpTrue, &[]), (OpCode::OpPop, &[])]),
        CompilerTestCase::new(
            "false",
            &[],
            &[(OpCode::OpFalse, &[]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            "1 > 2",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpGreatherThan, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "1 < 2",
            &[2, 1],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpGreatherThan, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "1 == 2",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpEqual, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "1 != 2",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpNotEqual, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "true == false",
            &[],
            &[
                (OpCode::OpTrue, &[]),
                (OpCode::OpFalse, &[]),
                (OpCode::OpEqual, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "true != false",
            &[],
            &[
                (OpCode::OpTrue, &[]),
                (OpCode::OpFalse, &[]),
                (OpCode::OpNotEqual, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "!true",
            &[],
            &[
                (OpCode::OpTrue, &[]),
                (OpCode::OpBang, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];
    run_compiler_test(tests);
}
