use std::cell::RefCell;
use std::io::{self, stdin, stdout, Write};
use std::rc::Rc;

use crate::eval::environment::Environment;
use crate::eval::value::Value;
use crate::eval::Eval;
use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">>";

pub fn start() -> io::Result<()> {
    let env = Environment::new();
    let mut eval = Eval::new(Rc::new(RefCell::new(env)));
    loop {
        let mut buffer = String::new();
        print!("{PROMPT} ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        let lexer = Lexer::new(buffer.chars().collect());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        parser.check_errors();
        match eval.eval_program(program) {
            Ok(Value::Let) => (),
            // Ok(Value::Function { .. }) => (),
            Ok(evaluated) => println!("{evaluated}"),
            Err(err) => println!("Err: {err}"),
        }
    }
}
