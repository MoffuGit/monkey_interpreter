use core::panic;
use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::Lexer;
use crate::parser::Parser;

use super::environment::Environment;
use super::value::Value;
use super::Eval;

#[test]
fn test_eval_int_expression() {
    let tests_cases = [
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::Int(value)) => assert_eq!(&value, expected),
            value => panic!("evaluated expected {expected}, got {value:?}"),
        }
    });
}

#[test]
fn test_eval_bool_expression() {
    let tests_cases = [
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::Bool(value)) => assert_eq!(&value, expected),
            value => panic!("evaluated expected {expected}, got {value:?}"),
        }
    })
}

#[test]
fn test_eval_bang_operator() {
    let tests_cases = [
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];
    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::Bool(value)) => assert_eq!(&value, expected),
            value => panic!("evaluated expected {expected}, got {value:?}"),
        }
    })
}

#[test]
fn test_eval_if_else_expression() {
    let tests_cases = [
        ("if (true) { 10 }", Value::Int(10)),
        ("if (false) { 10 }", Value::Null),
        ("if (1) { 10 }", Value::Int(10)),
        ("if (1 < 2) { 10 }", Value::Int(10)),
        ("if (1 > 2) { 10 }", Value::Null),
        ("if (1 > 2) { 10 } else { 20 }", Value::Int(20)),
        ("if (1 < 2) { 10 } else { 20 }", Value::Int(10)),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(value) => assert_eq!(&value, expected),
            Err(err) => panic!("got an error: {err}"),
        }
    })
}

#[test]
fn test_eval_return_statement() {
    let tests_cases = [
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            "if (10 > 1) { true; false; if (10 > 1) {8; 9; return 10; } return 1; } return 3;",
            10,
        ),
        (
            "if (10 > 1) { if (10 < 1) { 8; 9; return 10; } 1; 2; return 3; } return 4;",
            3,
        ),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::Int(value)) => assert_eq!(value, *expected),
            err => panic!("got an error: {err:?}"),
        }
    });
}

#[test]
fn test_eval_error_handling() {
    let tests_cases = [
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        (
            "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        ("foobar", "identifier not found: foobar"),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Err(err) => assert_eq!(err.msg, *expected),
            unexpected => panic!("got an error: {unexpected:?}"),
        }
    });
}

#[test]
fn test_eval_let_statement() {
    let tests_cases = [
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];
    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::Int(value)) => assert_eq!(value, *expected),
            unexpected => panic!("got an error: {unexpected:?}"),
        }
    });
}
