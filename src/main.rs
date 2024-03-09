pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod relp;

use crate::relp::start;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    start().expect("the repl dont fail");
}
