use crate::repl::start;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    start().expect("the repl dont fail");
}
