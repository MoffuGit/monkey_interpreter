use crate::ast::program::Program;
use crate::compiler::Compiler;
use crate::eval::value::Value;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::Vm;
use core::panic;
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
                };

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

#[test]
fn test_calling_functions_with_arguments_and_bindings() {
    let tests = vec![
        VmTestCase::new(
            r#"let identity = fn(a) { a };
    identity(4)
    "#,
            4,
        ),
        VmTestCase::new(
            r#"let sum = fn(a, b) { a + b; };
            sum(1, 2);
            "#,
            3,
        ),
        VmTestCase::new(
            r#"let sum = fn(a, b) { let c = a + b; c; };
            sum(1, 2);
                        "#,
            3,
        ),
        VmTestCase::new(
            r#"let sum = fn(a, b) { let c = a + b; c; };
            sum(1, 2) + sum(3, 4);
                        "#,
            10,
        ),
        VmTestCase::new(
            r#"let sum = fn(a, b) { let c = a + b; c; };
                        let outer = fn() { sum(1, 2) + sum(3, 4); };
                        outer();
                        "#,
            10,
        ),
        VmTestCase::new(
            r#"
                        let globalNum = 10;
                        let sum = fn(a, b) { let c = a + b; c + globalNum; };
                        let outer = fn() { sum(1, 2) + sum(3, 4) + globalNum; };
                        outer() + globalNum;
                        "#,
            50,
        ),
    ];

    run_vm_test(tests);
}

#[test]
fn test_calling_functions_with_wrong_arguments() {
    let tests = vec![
        ("fn() { 1; }(1)", "wrong number of arguments: want=0, got=1"),
        ("fn(a) { a; }()", "wrong number of arguments: want=1, got=0"),
        (
            "fn(a,b) { a + b; }(1)",
            "wrong number of arguments: want=2, got=1",
        ),
    ];

    for (input, expected) in tests {
        let program = parse(input.to_string());
        let mut compiler = Compiler::new();

        if let Err(err) = compiler.compile_program(program) {
            panic!("compiler error: {err}");
        }

        let mut vm = Vm::new(compiler.bytecode());

        if let Err(err) = vm.run() {
            assert_eq!(err.msg, expected);
        } else {
            panic!("expected a Vm error")
        };
    }
}

#[test]
fn test_builtin_functions() {
    let tests = vec![
        VmTestCase::new(r#"len("")"#, 0),
        VmTestCase::new(r#"len("four")"#, 4),
        VmTestCase::new(r#"len("hello world")"#, 11),
        VmTestCase::new("len([1, 2, 3])", 3),
        VmTestCase::new("len([])", 0),
        VmTestCase::new(r#"puts("hello", "world!")"#, Value::Null),
        VmTestCase::new("first([1, 2, 3])", 1),
        VmTestCase::new("first([])", Value::Null),
        VmTestCase::new("last([1, 2, 3])", 3),
        VmTestCase::new("last([])", Value::Null),
        VmTestCase::new("rest([1, 2, 3])", vec![2, 3]),
        VmTestCase::new("rest([])", Value::Null),
        VmTestCase::new("push([], 1)", vec![1]),
        VmTestCase::new(r#"first(rest(push([1,2,3], 4)))"#, 2),
    ];
    run_vm_test(tests);
}

#[test]
fn test_builtin_functions_with_wrong_arguments() {
    let tests = vec![
        ("len(1)", "argument to \"len\" not supported: got INTEGER"),
        (
            r#"len("one", "two")"#,
            "wrong number of arguments, got=2, want=1",
        ),
        ("first(1)", "argument to 'first' must be ARRAY, got INTEGER"),
        (
            "push(1, 1)",
            "argument to 'push' must be ARRAY, got: INTEGER",
        ),
        ("last(1)", "argument to 'last' must be ARRAY, got INTEGER"),
    ];
    for (input, expected) in tests {
        let program = parse(input.to_string());
        let mut compiler = Compiler::new();

        if let Err(err) = compiler.compile_program(program) {
            panic!("compiler error: {err}");
        }

        let mut vm = Vm::new(compiler.bytecode());

        if let Err(err) = vm.run() {
            assert_eq!(err.msg, expected);
        } else {
            panic!("expected a Vm error")
        };
    }
}

#[test]
fn test_closures() {
    let tests = vec![VmTestCase::new(
       "let newClosure = fn(a) { fn() { a; }; }; let closure = newClosure(99); closure();",
        99,
    ), VmTestCase::new("let newAdder = fn(a, b) { fn(c) { a + b + c }; }; let adder = newAdder(1, 2); adder(8);", 11),
    VmTestCase::new("let newAdder = fn(a, b) { let c = a + b; fn(d) { c + d }; }; let adder = newAdder(1, 2); adder(8);", 11),
    VmTestCase::new("let newAdderOuter = fn(a, b) { let c = a + b; fn(d) { let e = d + c; fn(f) { e + f; }; }; }; let newAdderInner = newAdderOuter(1, 2) let adder = newAdderInner(3); adder(8);", 14),
        VmTestCase::new("let a = 1; let newAdderOuter = fn(b) { fn(c) { fn(d) { a + b + c + d }; }; }; let newAdderInner = newAdderOuter(2) let adder = newAdderInner(3); adder(8);", 14),
        VmTestCase::new("let newClosure = fn(a, b) { let one = fn() { a; }; let two = fn() { b; }; fn() { one() + two(); }; }; let closure = newClosure(9, 90); closure();", 99),
        VmTestCase::new("let countDown = fn(x) { if (x == 0) { return 0; } else { countDown(x - 1); } }; countDown(1);", 0),
        VmTestCase::new("let countDown = fn(x) { if (x == 0) { return 0; } else { countDown(x - 1); } }; let wrapper = fn() { countDown(1); }; wrapper();", 0),
        VmTestCase::new("let wrapper = fn() { let countDown = fn(x) { if (x == 0) { return 0; } else { countDown(x - 1); } }; countDown(1); }; wrapper();", 0),
    ];

    run_vm_test(tests);
}

#[test]
fn test_recursive_fibonacci() {
    let tests = vec![
        VmTestCase::new("let fibonacci = fn(x) { if (x == 0) { return 0; } else { if (x == 1) { return 1; } else { fibonacci(x - 1) + fibonacci(x - 2); } } }; fibonacci(15);", 610)
    ];
    run_vm_test(tests);
}
