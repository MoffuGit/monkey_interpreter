use crate::ast::program::Program;
use crate::compiler::Compiler;
use crate::eval::value::Value;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::Vm;
use std::collections::HashMap;

fn parse(input: String) -> Program {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}

struct VmTestCase {
    input: String,
    expected: Value,
}

impl VmTestCase {
    pub fn new<I: Into<String>, V: Into<Value>>(input: I, expected: V) -> Self {
        Self {
            input: input.into(),
            expected: expected.into(),
        }
    }
}

fn run_vm_test(tests: Vec<VmTestCase>) {
    for test in tests {
        let program = parse(test.input);

        let mut compiler = Compiler::new();
        if compiler.compile_program(program).is_err() {
            panic!("Compiler pogram fail");
        }

        let mut vm = Vm::new(compiler.bytecode());
        if let Err(err) = vm.run() {
            panic!("{}", err);
        }

        let stack_element = vm.last_popped_element;

        assert_eq!(stack_element, Some(test.expected))
    }
}

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        VmTestCase::new("1", 1),
        VmTestCase::new("2", 2),
        VmTestCase::new("1 + 2", 3),
        VmTestCase::new("1 - 2", -1),
        VmTestCase::new("1 * 2", 2),
        VmTestCase::new("4 / 2", 2),
        VmTestCase::new("50 / 2 * 2 + 10 - 5", 55),
        VmTestCase::new("5 + 5 + 5 + 5 - 10", 10),
        VmTestCase::new("2 * 2 * 2 * 2 * 2", 32),
        VmTestCase::new("5 * 2 + 10", 20),
        VmTestCase::new("5 + 2 * 10", 25),
        VmTestCase::new("5 * (2 + 10)", 60),
        VmTestCase::new("-5", -5),
        VmTestCase::new("-10", -10),
        VmTestCase::new("-50 + 100 + -50", 0),
        VmTestCase::new("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    run_vm_test(tests);
}

#[test]
fn test_bool_expression() {
    let tests = vec![
        VmTestCase::new("true", true),
        VmTestCase::new("false", false),
        VmTestCase::new("1 < 2", true),
        VmTestCase::new("1 > 2", false),
        VmTestCase::new("1 < 1", false),
        VmTestCase::new("1 > 1", false),
        VmTestCase::new("1 == 1", true),
        VmTestCase::new("1 != 1", false),
        VmTestCase::new("1 == 2", false),
        VmTestCase::new("1 != 2", true),
        VmTestCase::new("true == true", true),
        VmTestCase::new("false == false", true),
        VmTestCase::new("true == false", false),
        VmTestCase::new("true != false", true),
        VmTestCase::new("false != true", true),
        VmTestCase::new("(1 < 2) == true", true),
        VmTestCase::new("(1 < 2) == false", false),
        VmTestCase::new("(1 > 2) == true", false),
        VmTestCase::new("(1 > 2) == false", true),
        VmTestCase::new("!true", false),
        VmTestCase::new("!false", true),
        VmTestCase::new("!5", false),
        VmTestCase::new("!!true", true),
        VmTestCase::new("!!false", false),
        VmTestCase::new("!!5", true),
        VmTestCase::new("!(if (false) { 5; })", true),
    ];

    run_vm_test(tests);
}

#[test]
fn test_conditionals() {
    let tests = vec![
        VmTestCase::new("if (true) { 10 }", 10),
        VmTestCase::new("if (true) { 10 } else { 20 }", 10),
        VmTestCase::new("if (false) { 10 } else { 20 } ", 20),
        VmTestCase::new("if (1) { 10 }", 10),
        VmTestCase::new("if (1 < 2) { 10 }", 10),
        VmTestCase::new("if (1 < 2) { 10 } else { 20 }", 10),
        VmTestCase::new("if (1 > 2) { 10 } else { 20 }", 20),
        VmTestCase::new("if (1 > 2) { 10 }", Value::Null),
        VmTestCase::new("if (false) { 10 }", Value::Null),
        VmTestCase::new("if ((if (false) { 10 })) { 10 } else { 20 }", 20),
    ];

    run_vm_test(tests);
}

#[test]
fn test_global_let_statements() {
    let tests = vec![
        VmTestCase::new("let one = 1; one", 1),
        VmTestCase::new("let one = 1; let two = 2; one + two", 3),
        VmTestCase::new("let one = 1; let two = one + one; one + two", 3),
    ];

    run_vm_test(tests);
}

#[test]
fn test_string_expression() {
    let tests = vec![
        VmTestCase::new(r#""monkey""#, "monkey"),
        VmTestCase::new(r#""mon" + "key""#, "monkey"),
        VmTestCase::new(r#""mon" + "key" + "love" + "bananas""#, "monkeylovebananas"),
    ];

    run_vm_test(tests);
}

#[test]
fn test_array_literals() {
    let empty: Vec<Value> = vec![];
    let tests = vec![
        VmTestCase::new("[]", empty),
        VmTestCase::new("[1,2,3]", vec![1, 2, 3]),
        VmTestCase::new("[1 + 2,3*4,5+6]", vec![3, 12, 11]),
        VmTestCase::new("[[1,2,3]]", vec![vec![1, 2, 3]]),
    ];

    run_vm_test(tests);
}

#[test]
fn test_hash_literals() {
    let tests = vec![
        VmTestCase::new("{}", HashMap::new()),
        VmTestCase::new(
            "{1:2, 2:3}",
            HashMap::from([
                (Value::Int(1), Value::Int(2)),
                (Value::Int(2), Value::Int(3)),
            ]),
        ),
        VmTestCase::new(
            "{1+1:2*2, 3+3:4*4}",
            HashMap::from([
                (Value::Int(2), Value::Int(4)),
                (Value::Int(6), Value::Int(16)),
            ]),
        ),
    ];
    run_vm_test(tests)
}

#[test]
fn test_index_expression() {
    let tests = vec![
        VmTestCase::new("[1,2,3][1]", 2),
        VmTestCase::new("[1,2,3][0 + 2]", 3),
        VmTestCase::new("[[1,2,3]][0][0]", 1),
        VmTestCase::new("[][0]", Value::Null),
        VmTestCase::new("[1,2,3][99]", Value::Null),
        VmTestCase::new("[1,2,3][-1]", Value::Null),
        VmTestCase::new("{ 1:2,3:4 }[1]", 2),
        VmTestCase::new("{ 1:2,3:4 }[1 + 2]", 4),
        VmTestCase::new("{ 1: 2 }[0]", Value::Null),
        VmTestCase::new("{  }[0]", Value::Null),
    ];

    run_vm_test(tests);
}

#[test]
fn test_calling_functions_without_arguments() {
    let tests = vec![
        VmTestCase::new(
            r#"let fivePlusTen = fn() { 10 / 5; };
                fivePlusTen();
                "#,
            2,
        ),
        VmTestCase::new(
            r#"let one = fn() { 1; };
        let two = fn() { 2; };
        one() + two()"#,
            3,
        ),
        VmTestCase::new(
            r#"let a = fn() { 1 };
                let b = fn() { a() + 1 };
                let c = fn() { b() + 1 };
                c()"#,
            3,
        ),
        VmTestCase::new(
            r#"let earlyExit = fn() { return 99; 100; };
                earlyExit();"#,
            99,
        ),
        VmTestCase::new(
            r#"let earlyExit = fn() { return 99; return 100; };
                earlyExit()"#,
            99,
        ),
        VmTestCase::new(
            r#"let noReturn = fn() { };
        noReturn();"#,
            Value::Null,
        ),
        VmTestCase::new(
            r#"let noReturn = fn() { };
        let noReturnTwo = fn() { noReturn() };
        noReturn();
        noReturnTwo();"#,
            Value::Null,
        ),
        VmTestCase::new(
            r#"let returnsOne = fn() { 1; };
        let returnsOneReturner = fn() { returnsOne; };
        returnsOneReturner()();"#,
            1,
        ),
    ];
    run_vm_test(tests);
}

#[test]
fn test_calling_functions_with_bindings() {
    let tests = vec![
        VmTestCase::new(
            r#"let one = fn() { let one = 1; one; };
        one();
        "#,
            1,
        ),
        VmTestCase::new(
            r#"let oneAndTwo = fn() { let one = 1; let two = 2; one + two };
                oneAndTwo();
                "#,
            3,
        ),
        VmTestCase::new(
            r#"let oneAndTwo = fn() { let one = 1; let two = 2; one + two };
                let threeAndFour = fn() {
                    let three = 3;
                    let four = 4;
                    three + four
                }

                oneAndTwo() + threeAndFour();
                "#,
            10,
        ),
        VmTestCase::new(
            r#"let firstFoobar = fn() {let foobar = 50; foobar;};
        let secondFoobar = fn() {let foobar = 100; foobar;};
        firstFoobar() + secondFoobar();
        "#,
            150,
        ),
        VmTestCase::new(
            r#"let globalSeed = 50;
        let minusOne = fn() { let num = 1; globalSeed - num; };
        let minusTwo = fn() { let num = 2; globalSeed - num; };
        minusOne() + minusTwo();
        "#,
            97,
        ),
    ];
    run_vm_test(tests);
}
