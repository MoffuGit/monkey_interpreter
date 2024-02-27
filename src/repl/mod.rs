use std::io::{self, stdin, stdout, Write};

use crate::lexer::{token::Token, Lexer};

const PROMPT: &str = ">>";

pub fn start() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        print!("{PROMPT} ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        let mut lexer = Lexer::new(buffer.chars().collect());
        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                break;
            }
            println!("{:?}", token);
        }
    }
}
