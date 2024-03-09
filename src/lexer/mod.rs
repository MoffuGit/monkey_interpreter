pub mod token;
use token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        self.ch = match self.input.get(self.read_position) {
            None => '\0',
            Some(char) => *char,
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peak_char(&self) -> char {
        match self.input.get(self.read_position) {
            None => '\0',
            Some(char) => *char,
        }
    }

    fn is_digit(&self) -> bool {
        self.ch.is_ascii_digit()
    }

    fn is_letter(&self) -> bool {
        self.ch.is_alphabetic() || self.ch == '_'
    }

    fn read_digit(&mut self) -> Token {
        let position = self.position;
        while self.is_digit() {
            self.read_char();
        }
        let int = String::from_iter(&self.input[position..self.position])
            .parse::<i64>()
            .expect("parse the string to i64");
        Token::Int(int)
    }

    fn skip_withespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while self.is_letter() {
            self.read_char()
        }

        let ident = String::from_iter(&self.input[position..self.position]);
        match ident.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(ident),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_withespace();
        let token = match self.ch {
            '=' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '%' => Token::Percent,
            '>' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    Token::GtorEq
                } else {
                    Token::Gt
                }
            }
            '<' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    Token::LtorEq
                } else {
                    Token::Lt
                }
            }
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '\0' => Token::Eof,
            _ if self.is_digit() => {
                return self.read_digit();
            }
            _ if self.is_letter() => {
                return self.read_identifier();
            }
            _ => Token::Illegal,
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
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
        let input = "10 == 10;
10 != 9;";

        let expected = vec![
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input.chars().collect());
        for expect in expected {
            let token = lexer.next_token();
            assert_eq!(expect, token);
        }
    }
}
