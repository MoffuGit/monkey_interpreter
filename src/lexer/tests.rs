use super::*;

#[test]
fn test_lexer1() {
    let input = "=+(){},;";
    let expected = vec![
        Token::Assign,
        Token::Plus,
        Token::Lparen,
        Token::Rparen,
        Token::Lbrace,
        Token::Rbrace,
        Token::Comma,
        Token::Semicolon,
        Token::Eof,
    ];
    let mut lexer = Lexer::new(input.chars().collect());
    for expect in expected {
        let token = lexer.next_token();
        assert_eq!(expect, token);
    }
}

#[test]
fn test_lexer2() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";
    let expected = vec![
        Token::Let,
        Token::Ident("five".into()),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".into()),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".into()),
        Token::Assign,
        Token::Function,
        Token::Lparen,
        Token::Ident("x".into()),
        Token::Comma,
        Token::Ident("y".into()),
        Token::Rparen,
        Token::Lbrace,
        Token::Ident("x".into()),
        Token::Plus,
        Token::Ident("y".into()),
        Token::Semicolon,
        Token::Rbrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".into()),
        Token::Assign,
        Token::Ident("add".into()),
        Token::Lparen,
        Token::Ident("five".into()),
        Token::Comma,
        Token::Ident("ten".into()),
        Token::Rparen,
        Token::Semicolon,
    ];

    let mut lexer = Lexer::new(input.chars().collect());
    for expect in expected {
        let token = lexer.next_token();
        assert_eq!(expect, token);
    }
}

#[test]
fn test_lexer3() {
    let input = "!-/*5;
5 < 10 > 5;
";
    let expected = vec![
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int(5),
        Token::Semicolon,
        Token::Int(5),
        Token::Lt,
        Token::Int(10),
        Token::Gt,
        Token::Int(5),
        Token::Semicolon,
        Token::Eof,
    ];
    let mut lexer = Lexer::new(input.chars().collect());
    for expect in expected {
        let token = lexer.next_token();
        assert_eq!(expect, token);
    }
}

#[test]
fn test_lexer4() {
    let input = "if (5 < 10) {
    return true;
} else {
    return false;
};
";
    let expected = vec![
        Token::If,
        Token::Lparen,
        Token::Int(5),
        Token::Lt,
        Token::Int(10),
        Token::Rparen,
        Token::Lbrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::Rbrace,
        Token::Else,
        Token::Lbrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::Rbrace,
        Token::Semicolon,
    ];
    let mut lexer = Lexer::new(input.chars().collect());
    for expect in expected {
        let token = lexer.next_token();
        assert_eq!(expect, token);
    }
}

#[test]
fn test_lexer5() {
    let input = r#"
10 == 10;
10 != 9;
"foobar";
"foo bar";
[1, 2];
"#;

    let expected = vec![
        Token::Int(10),
        Token::Eq,
        Token::Int(10),
        Token::Semicolon,
        Token::Int(10),
        Token::NotEq,
        Token::Int(9),
        Token::Semicolon,
        Token::String("foobar".into()),
        Token::Semicolon,
        Token::String("foo bar".into()),
        Token::Semicolon,
        Token::Lbracket,
        Token::Int(1),
        Token::Comma,
        Token::Int(2),
        Token::Rbracket,
        Token::Semicolon,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input.chars().collect());
    for expect in expected {
        let token = lexer.next_token();
        assert_eq!(expect, token);
    }
}
