use crate::ast::program::Program;
use crate::compiler::Compiler;
use crate::eval::value::Value;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::Vm;

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
