use crate::compiler::{self, Compiler};
use crate::eval::builtin::new_builtins;
use crate::vm::{self, Vm};
use std::cell::RefCell;
use std::io::{self, stdin, stdout, Write};
use std::rc::Rc;

use crate::eval::environment::Environment;
use crate::eval::value::Value;
use crate::eval::Eval;
use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">>";

pub fn start_interpreter() -> io::Result<()> {
    let env = Environment::from(new_builtins());
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

pub fn start_compiler() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        print!("{PROMPT} ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        let lexer = Lexer::new(buffer.chars().collect());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        parser.check_errors();
        let mut compiler = Compiler::new();
        if let Err(err) = compiler.compile_program(program) {
            println!("Compiler error: {err}");
            continue;
        }

        let mut machine = Vm::new(compiler.bytecode());

        if let Err(err) = machine.run() {
            println!("Executing bytecode error: {err}");
            continue;
        }

        if let Some(top) = machine.last_popped_element {
            println!("{}", top);
        }
    }
}
