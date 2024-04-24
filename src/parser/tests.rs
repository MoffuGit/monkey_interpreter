use core::panic;

use super::*;

#[test]
fn test_let_statement() {
    let input = "let x = 5;
    let y = 10;
    let foobar = 838383;";

    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(program.statements.len(), 3);
    let expected = vec![
        Statement::r#let("x", Expression::Int(5)),
        Statement::r#let("y", Expression::Int(10)),
        Statement::r#let("foobar", Expression::Int(838383)),
    ];

    program
        .statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| {
            assert_eq!(*statement, expected[idx]);
        });
}

#[test]
fn test_return_statement() {
    let input = "return 5; return 10; return 993322;";

    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(program.statements.len(), 3);

    let expected = vec![
        Statement::Return(Expression::Int(5)),
        Statement::Return(Expression::Int(10)),
        Statement::Return(Expression::Int(993322)),
    ];

    program
        .statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| {
            assert_eq!(*statement, expected[idx]);
        })
}

#[test]
fn test_identifier() {
    let input = "foobar;";

    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(program.statements.len(), 1);

    let statement = &program.statements[0];
    match statement {
        Statement::Expression(value) => {
            if value != &Expression::from("foobar") {
                panic!("Statement::Expression not foobar, got: {:?}", value)
            }
        }
        _ => panic!("program.statement[0] isnot Statement::Expression"),
    }
}

#[test]
fn test_int_literal() {
    let input = "5;";

    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(program.statements.len(), 1);

    let statement = &program.statements[0];
    match statement {
        Statement::Expression(value) => {
            if value != &Expression::from(5) {
                panic!("Statement::Expression not 5, got: {:?}", value)
            }
        }
        _ => panic!("program.statement[0] is not Statement::Expression"),
    }
}

#[test]
fn test_prefix_expression() {
    let input = "!5;
-15;";

    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    let statements = &program.statements;

    assert_eq!(statements.len(), 2);
    statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| match statement {
            Statement::Expression(value) => match value {
                Expression::Prefix { .. } => (),
                value => {
                    panic!("Statement::Expression is not a Expression::Prefix, got: {value}")
                }
            },
            statement => {
                panic!("program.statement[{idx}] is not Statement::Expression, got: {statement}")
            }
        });
}

#[test]
fn test_infix_expression() {
    let input = "5 + 5;
5 - 5;
5 * 5;
5 / 5;
5 > 5;
5 < 5;
5 == 5;
5 != 5;
3 + 4 * 5 == 3 * 1 + 4 * 5;
3 <= 4;";

    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    let statements = &program.statements;

    assert_eq!(statements.len(), 10);
    statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| match statement {
            Statement::Expression(value) => match value {
                Expression::Infix { .. } => (),
                value => {
                    panic!("Statement::Expression is not a Expression::Prefix, got: {value:?}")
                }
            },
            statement => {
                panic!("program.statement[{idx}] is not Statement::Expression, got: {statement:?}")
            }
        });
}

#[test]
fn test_bool() {
    let input = "false;
true;";
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(program.statements.len(), 2);
    assert_eq!(
        program.statements[0],
        Statement::Expression(Expression::Bool(false))
    );
    assert_eq!(
        program.statements[1],
        Statement::Expression(Expression::Bool(true))
    )
}

#[test]
fn test_grouped_expression() {
    let input = " 1 + (2 + 3) + 4;
(5 + 5) * 2;
-(5 + 5);
!(true == true);
";
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    let statements = &program.statements;

    assert_eq!(statements[0].to_string(), "((1 + (2 + 3)) + 4)");
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }".chars().collect();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(
        program.statements.first(),
        Some(&Statement::Expression(Expression::If {
            condition: Box::new(Expression::Infix {
                lhs: Box::new(Expression::Identifier("x".into())),
                operator: InfixOperator::LessThan,
                rhs: Box::new(Expression::Identifier("y".into()))
            }),
            consequence: vec![Statement::Expression(Expression::Identifier("x".into()))],
            alternative: None
        }))
    )
}

#[test]
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }".chars().collect();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(
        program.statements.first(),
        Some(&Statement::Expression(Expression::If {
            condition: Box::new(Expression::Infix {
                lhs: Box::new(Expression::Identifier("x".into())),
                operator: InfixOperator::LessThan,
                rhs: Box::new(Expression::Identifier("y".into()))
            }),
            consequence: vec![Statement::Expression(Expression::Identifier("x".into()))],
            alternative: Some(vec![Statement::Expression(Expression::Identifier(
                "y".into()
            ))])
        }))
    )
}

#[test]
fn test_function_literal() {
    let input = "fn(x, y) { x + y;}".chars().collect();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();

    assert_eq!(
        program.statements.first(),
        Some(&Statement::Expression(Expression::Fn {
            parameters: vec!["x".into(), "y".into()],
            body: vec![Statement::Expression(Expression::Infix {
                lhs: Box::new(Expression::Identifier("x".into())),
                operator: InfixOperator::Add,
                rhs: Box::new(Expression::Identifier("y".into()))
            })]
        }))
    )
}

#[test]
fn test_function_parameters() {
    let test_cases = [
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x".to_string()]),
        (
            "fn(x, foo, bar) {};",
            vec!["x".to_string(), "foo".to_string(), "bar".to_string()],
        ),
    ];

    test_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        parser.check_errors();
        if let Some(Statement::Expression(Expression::Fn { parameters, .. })) =
            program.statements.first()
        {
            assert_eq!(parameters, expected)
        }
    })
}

#[test]
fn test_call_expression() {
    let input = "add(a,b)".chars().collect();
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();
    assert_eq!(
        program.statements.first(),
        Some(&Statement::Expression(Expression::Call {
            function: Box::new(Expression::Identifier("add".to_string())),
            arguments: vec![
                Expression::Identifier("a".to_string()),
                Expression::Identifier("b".to_string())
            ]
        }))
    );
}

#[test]
fn test_operator_precedence() {
    let test_cases = [
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        (
            "a * [1,2,3,4][b * c] * d",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        ),
        (
            "add(a * b[2], b[1], 2 * [1, 2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ),
    ];

    test_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        parser.check_errors();
        assert_eq!(program.statements[0].to_string(), expected.to_string());
    })
}

#[test]
fn test_string_expression() {
    let test_cases = [(r#""foobar""#, "foobar")];

    test_cases.iter().for_each(|(input, expected)| {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        parser.check_errors();
        assert_eq!(program.statements[0].to_string(), expected.to_string());
    })
}

#[test]
fn test_array() {
    let input = "[1, 2 * 2, 3 + 3]".chars().collect();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();
    match program.statements.first() {
        Some(Statement::Expression(Expression::Array(values))) => {
            assert_eq!(values[0], Expression::Int(1));
        }
        value => panic!("expected Array got: {:?}", value),
    }
}

#[test]
fn test_index_expression() {
    let input = "myArray[1 + 1]".chars().collect();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    parser.check_errors();
    match program.statements.first() {
        Some(Statement::Expression(Expression::Index { lhs, index })) => {
            assert_eq!(
                lhs,
                &Box::new(Expression::Identifier("myArray".to_string()))
            );
            assert_eq!(
                index,
                &Box::new(Expression::Infix {
                    lhs: Box::new(Expression::Int(1)),
                    operator: InfixOperator::Add,
                    rhs: Box::new(Expression::Int(1))
                })
            )
        }
        value => panic!("expected Array got: {:?}", value),
    }
}
