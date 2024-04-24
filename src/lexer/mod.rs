pub mod token;

use token::Token;

#[cfg(test)]
mod tests;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    column: usize,
    line: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            column: 0,
            line: 1,
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
        if let '\n' = self.ch {
            self.column = 0;
            self.line += 1;
        } else {
            self.column += 1;
        }
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

    pub fn read_string(&mut self) -> Token {
        self.read_char();
        let position = self.position;

        loop {
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
            self.read_char();
        }

        self.read_char();
        Token::String(self.input[position..self.position - 1].iter().collect())
    }

    pub fn next_token(&mut self) -> (Token, (usize, usize)) {
        self.skip_withespace();
        let token = (
            match self.ch {
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
                '[' => Token::Lbracket,
                ']' => Token::Rbracket,
                '\0' => Token::Eof,
                '"' => return (self.read_string(), (self.line, self.column)),
                _ if self.is_digit() => {
                    return (self.read_digit(), (self.line, self.column));
                }
                _ if self.is_letter() => {
                    return (self.read_identifier(), (self.line, self.column));
                }
                _ => Token::Illegal,
            },
            (self.line, self.column),
        );
        self.read_char();
        token
    }
}
