use crate::ast::program::Program;
use crate::code::{self, concat_instructions, Instructions, OpCode};
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

        if let Err(err) = compiler.compile_program(program) {
            panic!("Compile program fail: {:?}", err);
        };

        let byte_code = compiler.bytecode();

        assert_eq!(
            byte_code.instructions.to_string(),
            concat_instructions(&test.expected_instructions).to_string()
        );
        assert_eq!(&byte_code.constants.len(), &test.expected_constants.len());

        test.expected_constants
            .iter()
            .zip(byte_code.constants.iter())
            .for_each(|(expected, value)| {
                assert_eq!(expected, value, "\nexpected: {}\ngot: {}", expected, value)
            });
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

#[test]
fn test_compiler_scopes() {
    let mut compiler = Compiler::new();
    assert_eq!(0, compiler.scope_idx);
    let global_symbl_table = compiler.symbol_table.clone();

    compiler.emit(OpCode::OpMul, &[]);
    compiler.enter_scope();
    assert_eq!(1, compiler.scope_idx);
    compiler.emit(OpCode::OpDiv, &[]);
    assert!(compiler
        .scopes
        .get(compiler.scope_idx)
        .is_some_and(|scope| scope.instructions.len() == 1));
    assert!(compiler
        .scopes
        .get(compiler.scope_idx)
        .is_some_and(|scope| scope
            .last_instruction
            .as_ref()
            .is_some_and(|last_instruction| last_instruction.op == OpCode::OpDiv)));
    assert!(compiler
        .symbol_table
        .borrow()
        .outer
        .clone()
        .is_some_and(|table| table == global_symbl_table));
    compiler.leave_scope();
    assert_eq!(0, compiler.scope_idx);
    assert!(compiler.symbol_table.clone() == global_symbl_table);
    assert!(compiler.symbol_table.borrow().outer.clone().is_none());
    compiler.emit(OpCode::OpAdd, &[]);
    assert!(compiler
        .scopes
        .get(compiler.scope_idx)
        .is_some_and(|scope| scope.instructions.len() == 2));
    assert!(compiler
        .scopes
        .get(compiler.scope_idx)
        .is_some_and(|scope| scope
            .last_instruction
            .as_ref()
            .is_some_and(|last_instruction| last_instruction.op == OpCode::OpAdd)));
    assert!(compiler
        .scopes
        .get(compiler.scope_idx)
        .is_some_and(|scope| scope
            .previous_instruction
            .as_ref()
            .is_some_and(|previous_instruction| previous_instruction.op == OpCode::OpMul)));
}

#[test]
fn test_functions() {
    let tests = &[
        CompilerTestCase::new(
            "fn() {return 5 + 10}",
            &[
                Value::Int(5),
                Value::Int(10),
                Value::CompiledFunction {
                    instructions: Instructions::from(Vec::from([
                        (OpCode::OpConstant, vec![0]),
                        (OpCode::OpConstant, vec![1]),
                        (OpCode::OpAdd, vec![]),
                        (OpCode::OpReturnValue, vec![]),
                    ])),
                    num_locals: 0,
                    num_parameters: 0,
                },
            ],
            &[(OpCode::OpConstant, &[2]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            "fn() { 5 + 10}",
            &[
                Value::Int(5),
                Value::Int(10),
                Value::CompiledFunction {
                    instructions: Instructions::from(Vec::from([
                        (OpCode::OpConstant, vec![0]),
                        (OpCode::OpConstant, vec![1]),
                        (OpCode::OpAdd, vec![]),
                        (OpCode::OpReturnValue, vec![]),
                    ])),
                    num_locals: 0,
                    num_parameters: 0,
                },
            ],
            &[(OpCode::OpConstant, &[2]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            "fn() { 1; 2 }",
            &[
                Value::Int(1),
                Value::Int(2),
                Value::CompiledFunction {
                    instructions: Instructions::from(Vec::from([
                        (OpCode::OpConstant, vec![0]),
                        (OpCode::OpPop, vec![]),
                        (OpCode::OpConstant, vec![1]),
                        (OpCode::OpReturnValue, vec![]),
                    ])),
                    num_locals: 0,
                    num_parameters: 0,
                },
            ],
            &[(OpCode::OpConstant, &[2]), (OpCode::OpPop, &[])],
        ),
    ];

    run_compiler_test(tests);
}

#[test]
fn test_functions_without_return_value() {
    let tests = &[CompilerTestCase::new(
        "fn() {}",
        &[Value::CompiledFunction {
            instructions: Instructions::from(Vec::from([(OpCode::OpReturn, vec![])])),
            num_locals: 0,
            num_parameters: 0,
        }],
        &[(OpCode::OpConstant, &[0]), (OpCode::OpPop, &[])],
    )];

    run_compiler_test(tests);
}

#[test]
fn test_functions_calls() {
    let tests = &[
        CompilerTestCase::new(
            "fn() { 24 }();",
            &[
                Value::Int(24),
                Value::CompiledFunction {
                    instructions: Instructions::from(Vec::from([
                        (OpCode::OpConstant, vec![0]),
                        (OpCode::OpReturnValue, vec![]),
                    ])),
                    num_locals: 0,
                    num_parameters: 0,
                },
            ],
            &[
                (OpCode::OpConstant, &[1]),
                (OpCode::OpCall, &[0]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            r#"let noArg = fn() { 24 };
noArg();"#,
            &[
                Value::Int(24),
                Value::CompiledFunction {
                    instructions: Instructions::from(Vec::from([
                        (OpCode::OpConstant, vec![0]),
                        (OpCode::OpReturnValue, vec![]),
                    ])),
                    num_locals: 0,
                    num_parameters: 0,
                },
            ],
            &[
                (OpCode::OpConstant, &[1]),
                (OpCode::OpSetGlobal, &[0]),
                (OpCode::OpGetGlobal, &[0]),
                (OpCode::OpCall, &[0]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            r#"let oneArg = fn(a) { a };
oneArg(24);
            "#,
            &[
                Value::CompiledFunction {
                    instructions: Instructions::from(vec![
                        (OpCode::OpGetLocal, vec![0]),
                        (OpCode::OpReturnValue, vec![]),
                    ]),
                    num_locals: 1,
                    num_parameters: 1,
                },
                Value::Int(24),
            ],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpSetGlobal, &[0]),
                (OpCode::OpGetGlobal, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpCall, &[1]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            r#"let manyArg = fn(a, b, c) { a; b; c;};
manyArg(24, 25, 26);
"#,
            &[
                Value::CompiledFunction {
                    instructions: Instructions::from(vec![
                        (OpCode::OpGetLocal, vec![0]),
                        (OpCode::OpPop, vec![]),
                        (OpCode::OpGetLocal, vec![1]),
                        (OpCode::OpPop, vec![]),
                        (OpCode::OpGetLocal, vec![2]),
                        (OpCode::OpReturnValue, vec![]),
                    ]),
                    num_locals: 3,
                    num_parameters: 3,
                },
                Value::Int(24),
                Value::Int(25),
                Value::Int(26),
            ],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpSetGlobal, &[0]),
                (OpCode::OpGetGlobal, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpConstant, &[2]),
                (OpCode::OpConstant, &[3]),
                (OpCode::OpCall, &[3]),
                (OpCode::OpPop, &[]),
            ],
        ),
    ];
    run_compiler_test(tests);
}

#[test]
fn test_let_statements_scopes() {
    let tests = &[
        CompilerTestCase::new(
            r#"let num = 55;
fn() { num }
"#,
            &[
                Value::Int(55),
                Value::CompiledFunction {
                    instructions: Instructions::from(vec![
                        (OpCode::OpGetGlobal, vec![0]),
                        (OpCode::OpReturnValue, vec![]),
                    ]),
                    num_locals: 0,
                    num_parameters: 0,
                },
            ],
            &[
                (OpCode::OpConstant, &[0]),
                (OpCode::OpSetGlobal, &[0]),
                (OpCode::OpConstant, &[1]),
                (OpCode::OpPop, &[]),
            ],
        ),
        CompilerTestCase::new(
            r#"fn() {
let num = 55;
num
}"#,
            &[
                Value::Int(55),
                Value::CompiledFunction {
                    instructions: Instructions::from(vec![
                        (OpCode::OpConstant, vec![0]),
                        (OpCode::OpSetLocal, vec![0]),
                        (OpCode::OpGetLocal, vec![0]),
                        (OpCode::OpReturnValue, vec![]),
                    ]),
                    num_locals: 1,
                    num_parameters: 0,
                },
            ],
            &[(OpCode::OpConstant, &[1]), (OpCode::OpPop, &[])],
        ),
        CompilerTestCase::new(
            r#"fn() {
let a = 55;
let b = 77;
a + b
}"#,
            &[
                Value::Int(55),
                Value::Int(77),
                Value::CompiledFunction {
                    instructions: Instructions::from(vec![
                        (OpCode::OpConstant, vec![0]),
                        (OpCode::OpSetLocal, vec![0]),
                        (OpCode::OpConstant, vec![1]),
                        (OpCode::OpSetLocal, vec![1]),
                        (OpCode::OpGetLocal, vec![0]),
                        (OpCode::OpGetLocal, vec![1]),
                        (OpCode::OpAdd, vec![]),
                        (OpCode::OpReturnValue, vec![]),
                    ]),
                    num_locals: 2,
                    num_parameters: 0,
                },
            ],
            &[(OpCode::OpConstant, &[2]), (OpCode::OpPop, &[])],
        ),
    ];

    run_compiler_test(tests);
}
