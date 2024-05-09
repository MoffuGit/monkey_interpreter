use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::lexer::Lexer;
use crate::parser::Parser;

use super::builtin::new_builtins;
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
fn test_string_expression() {
    let tests_cases = [(r#""foobar""#, "foobar"), (r#""foo bar""#, "foo bar")];
    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::String(string)) => assert_eq!(string, expected.to_string()),
            value => panic!("evaluated expected {expected}, got {value:?}"),
        }
    })
}

#[test]
fn test_string_concatenation() {
    let tests_cases = [(r#""Hello" + " " + "World!""#, "Hello World!")];
    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::String(string)) => assert_eq!(string, expected.to_string()),
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
        // (
        //     r#"len(1)"#,
        //     r#"argument to "len" not supported: got INTEGER"#,
        // ),
        // (
        //     r#"len("one", "two")"#,
        //     "wrong number of arguments, got=2, want=1",
        // ),
        // ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        // ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        // ("-true", "unknown operator: -BOOLEAN"),
        // ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        // ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        // (
        //     "if (10 > 1) { true + false; }",
        //     "unknown operator: BOOLEAN + BOOLEAN",
        // ),
        // (
        //     "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
        //     "unknown operator: BOOLEAN + BOOLEAN",
        // ),
        // ("foobar", "identifier not found: foobar"),
        // (r#""Hello" - "World""#, "unknown operator: STRING - STRING"),
        // ("[1, 2, 3][3]", "index out of bounds"),
        ("[1, 2, 3][-1]", "index out of bounds"),
        (
            r#"{"name": "Monkey"}[fn(x) { x }];"#,
            "unusable as hash key: FUNCTION",
        ),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::from(new_builtins());
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Err(err) => assert_eq!(err.msg, *expected),
            Ok(Value::Array(_)) => panic!("this not should happen"),
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

#[test]
fn test_eval_function() {
    let input = "fn(x) { x + 2; };";
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let env = Environment::new();
    let mut eval = Eval::new(Rc::new(RefCell::new(env)));
    parser.check_errors();

    match eval.eval_program(program) {
        Ok(Value::Function {
            parameters, body, ..
        }) => {
            assert_eq!(parameters, vec!["x"]);
            assert_eq!(body[0].to_string(), "(x + 2)");
        }
        Ok(value) => panic!("expected Value::Function, got: {value:?}"),
        Err(err) => panic!("got an error: {err:?}"),
    }
}

#[test]
fn test_eval_function_call() {
    let tests_cases = [
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
        (
            "let newAdder = fn(x) { fn(y) { x + y }; }; let addTwo = newAdder(2); addTwo(2);",
            4,
        ),
        ("let counter = fn(x) { if (x > 90) { return x; } else { let foobar = 9999; counter(x + 1); } }; counter(0);", 91)
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

#[test]
fn test_builtin_functions() {
    let tests_cases = [
        (r#"len("")"#, 0),
        (r#"len("four")"#, 4),
        (r#"len("hello world")"#, 11),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::from(new_builtins());
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::Int(value)) => assert_eq!(value, *expected),
            unexpected => panic!("got an error: {unexpected:?}"),
        }
    });
}

#[test]
fn test_eval_array() {
    let input = "[1, 2 * 2, 3 + 3]";
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let env = Environment::new();
    let mut eval = Eval::new(Rc::new(RefCell::new(env)));
    parser.check_errors();

    match eval.eval_program(program) {
        Ok(Value::Array(values)) => {
            assert_eq!(Value::Int(1), values[0]);
            assert_eq!(Value::Int(4), values[1]);
            assert_eq!(Value::Int(6), values[2]);
        }
        Ok(value) => panic!("expected Value::Function, got: {value:?}"),
        Err(err) => panic!("got an error: {err:?}"),
    }
}

#[test]
fn test_eval_arrray_index_expression() {
    let tests_cases = [
        ("[1, 2, 3][0]", 1),
        ("[1, 2, 3][1]", 2),
        ("[1, 2, 3][2]", 3),
        ("let i = 0; [1][i];", 1),
        ("[1, 2, 3][1 + 1];", 3),
        ("let myArray = [1, 2, 3]; myArray[2];", 3),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            6,
        ),
        ("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2),
    ];

    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::from(new_builtins());
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(Value::Int(value)) => assert_eq!(value, *expected),
            unexpected => panic!("got an error: {unexpected:?}"),
        }
    });
}

#[test]
fn test_hash_literals() {
    let input = r#"let two = "two";
{"one": 10 - 9, two: 1 + 1, "thr" + "ee": 6 / 2, 4: 4, true: 5, false: 6}"#;

    let mut expected = HashMap::new();
    expected.insert(Value::String("one".into()), Value::Int(1));
    expected.insert(Value::String("two".into()), Value::Int(2));
    expected.insert(Value::String("three".into()), Value::Int(3));
    expected.insert(Value::Int(4), Value::Int(4));
    expected.insert(Value::Bool(true), Value::Int(5));
    expected.insert(Value::Bool(false), Value::Int(6));

    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let env = Environment::from(new_builtins());
    let mut eval = Eval::new(Rc::new(RefCell::new(env)));
    match eval.eval_program(program) {
        Ok(Value::Hash(value)) => assert_eq!(value, expected),
        unexpected => panic!("got an error: {unexpected:?}"),
    }
}

#[test]
fn test_hash_index_expression() {
    let tests_cases = [
        (r#"{"foo": 5}["foo"]"#, Value::Int(5)),
        (r#"{"foo": 5}["bar"]"#, Value::Null),
        (r#"let key = "foo"; {"foo": 5}[key]"#, Value::Int(5)),
        (r#"{}["foo"]"#, Value::Null),
        (r#"{5: 5}[5]"#, Value::Int(5)),
        (r#"{true: 5}[true]"#, Value::Int(5)),
        (r#"{false: 5}[false]"#, Value::Int(5)),
    ];
    tests_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::from(new_builtins());
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        match eval.eval_program(program) {
            Ok(value) => assert_eq!(value, *expected),
            unexpected => panic!("got an error: {unexpected:?}"),
        }
    });
}
