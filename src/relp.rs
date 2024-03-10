use std::io::{self, stdin, stdout, Write};

use crate::eval::EvalTrait;
use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">>";

pub fn start() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        print!("{PROMPT} ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        let lexer = Lexer::new(buffer.chars().collect());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        parser.check_errors();
        if let Ok(evaluated) = program.eval() {
            println!("{evaluated}");
        }
    }
}
