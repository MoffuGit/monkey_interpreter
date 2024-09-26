use crate::ast::program::Program;
use crate::code::{self, concat_instructions, OpCode};
use crate::eval::value::Value;
use crate::lexer::Lexer;
use crate::parser::Parser;

use super::Compiler;

struct CompilerTestCase {
    input: String,
    expected_constants: Vec<Value>,
    expected_instructions: Vec<code::Instructions>,
}

impl CompilerTestCase {
    pub fn new<S: Into<String>, V: Into<Value> + Clone>(
        input: S,
        expected_constants: &[V],
        expected_instructions: &[(OpCode, &[i64])],
    ) -> Self {
        CompilerTestCase {
            input: input.into(),
            expected_constants: expected_constants
                .iter()
                .map(|value| std::convert::Into::<Value>::into(value.clone()))
                .collect::<Vec<Value>>(),
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

        assert_eq!(test.expected_constants, byte_code.constants)
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
    let empty: Vec<&str> = Vec::new();
    let tests = &[
        CompilerTestCase::new(
            "true",
            &empty,
            &[(OpCode::OpTrue, &[]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            "false",
            &empty,
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
            &empty,
            &[
                (OpCode::OpTrue, &[]),
                (OpCode::OpFalse, &[]),
                (OpCode::OpEqual, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "true != false",
            &empty,
            &[
                (OpCode::OpTrue, &[]),
                (OpCode::OpFalse, &[]),
                (OpCode::OpNotEqual, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "!true",
            &empty,
            &[
                (OpCode::OpTrue, &[]),
                (OpCode::OpBang, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];
    run_compiler_test(tests);
}

#[test]
pub fn test_conditional() {
    let tests = &[
        CompilerTestCase::new(
            "if (true) { 10 }; 3333;",
            &[10, 3333],
            &[
                // 0000
                (OpCode::OpTrue, &[]),
                // 0001
                (OpCode::OpJumpNotTruthy, &[10]),
                // 0004
                (OpCode::OpConstant, &[0]),
                // 0007
                (OpCode::OpJump, &[11]),
                // 0010
                (OpCode::OpNull, &[]),
                // 0011
                (OpCode::OpPop, &[]),
                // 0012
                (OpCode::OpConstant, &[1]),
                // 0015
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "if (true) { 10 } else { 20 }; 3333;",
            &[10, 20, 3333],
            &[
                // 0000
                (OpCode::OpTrue, &[]),
                // 0001
                (OpCode::OpJumpNotTruthy, &[10]),
                // 0004
                (OpCode::OpConstant, &[0]),
                // 0007
                (OpCode::OpJump, &[13]),
                // 0010
                (OpCode::OpConstant, &[1]),
                // 0013
                (OpCode::OpPop, &[]),
                // 0014
                (OpCode::OpConstant, &[2]),
                // 0017
                (OpCode::OpPop, &[]),
            ],
        ),
    ];
    run_compiler_test(tests);
}

#[test]
fn test_global_let_statements() {
    let tests = &[
        CompilerTestCase::new(
            " let one = 1; let two = 2; ",
            &[1, 2],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpSetGlobal, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpSetGlobal, &[1]),
            ],
        ),
        CompilerTestCase::new(
            " let one = 1; one; ",
            &[1],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpSetGlobal, &[0]),
                (OpCode::OpGetGlobal, &[0]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            " let one = 1; let two = one; two;",
            &[1],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpSetGlobal, &[0]),
                (OpCode::OpGetGlobal, &[0]),
                (OpCode::OpSetGlobal, &[1]),
                (OpCode::OpGetGlobal, &[1]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];
    run_compiler_test(tests)
}

#[test]
fn test_string_expressions() {
    let tests = &[
        CompilerTestCase::new(
            r#""monkey""#,
            &["monkey"],
            &[(OpCode::OpConstant, &[0]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            r#""mon" + "key""#,
            &["mon", "key"],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpAdd, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];

    run_compiler_test(tests);
}

#[test]
fn test_array_literals() {
    let empty: Vec<&str> = Vec::new();
    let tests = &[
        CompilerTestCase::new(
            "[]",
            &empty,
            &[(OpCode::OpArray, &[0]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            "[[1,2,3]]",
            &[1, 2, 3],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpArray, &[3]),
                (OpCode::OpArray, &[1]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "[1,2,3]",
            &[1, 2, 3],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpArray, &[3]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "[1 + 2, 3 - 4, 5 * 6]",
            &[1, 2, 3, 4, 5, 6],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpAdd, &[]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpConstant, &[3]),
                (OpCode::OpSub, &[]),
                (OpCode::OpConstant, &[4]),
                (OpCode::OpConstant, &[5]),
                (OpCode::OpMul, &[]),
                (OpCode::OpArray, &[3]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];

    run_compiler_test(tests);
}

#[test]
fn test_hash_literals() {
    let empty: Vec<&str> = Vec::new();
    let tests = &[
        CompilerTestCase::new(
            "{}",
            &empty,
            &[(OpCode::OpHash, &[0]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            "{1:2, 3:4, 5:6}",
            &[1, 2, 3, 4, 5, 6],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpConstant, &[3]),
                (OpCode::OpConstant, &[4]),
                (OpCode::OpConstant, &[5]),
                (OpCode::OpHash, &[6]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "{1:2+3, 4:5*6}",
            &[1, 2, 3, 4, 5, 6],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpAdd, &[]),
                (OpCode::OpConstant, &[3]),
                (OpCode::OpConstant, &[4]),
                (OpCode::OpConstant, &[5]),
                (OpCode::OpMul, &[]),
                (OpCode::OpHash, &[4]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];

    run_compiler_test(tests);
}

#[test]
fn test_index_expression() {
    let tests = &[
        CompilerTestCase::new(
            "[1,2,3][1+1]",
            &[1, 2, 3, 1, 1],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpArray, &[3]),
                (OpCode::OpConstant, &[3]),
                (OpCode::OpConstant, &[4]),
                (OpCode::OpAdd, &[]),
                (OpCode::OpIndex, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            "{1: 2}[2 - 1]",
            &[1, 2, 2, 1],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpHash, &[2]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpConstant, &[3]),
                (OpCode::OpSub, &[]),
                (OpCode::OpIndex, &[]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];

    run_compiler_test(tests);
}
