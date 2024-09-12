pub mod ast;
pub mod code;
pub mod compiler;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod relp;
pub mod vm;

use self::relp::start_compiler;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    start_compiler().expect("the repl dont fail");
}
