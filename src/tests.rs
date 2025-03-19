extern crate test;

use std::cell::RefCell;
use std::rc::Rc;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::vm::Vm;
use test::Bencher;

use crate::eval::environment::Environment;
use crate::eval::Eval;
use crate::parser::Parser;
// const INPUT: &str = "let fibonacci = fn(x) { if (x == 0) { 0 } else { if (x == 1) { return 1; } else { fibonacci(x - 1) + fibonacci(x - 2); } } }; fibonacci(35);";
#[bench]
pub fn bench_inter(b: &mut Bencher) {
    b.iter(|| {
        let input = String::from( "let fibonacci = fn(x) { if (x == 0) { 0 } else { if (x == 1) { return 1; } else { fibonacci(x - 1) + fibonacci(x - 2); } } }; fibonacci(35);");
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Environment::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
        let res = eval.eval_program(program);
        println!("result: {:?}", res);
    })
}

#[bench]
pub fn bench_comp(b: &mut Bencher) {
    b.iter(|| {
        let input = String::from( "let fibonacci = fn(x) { if (x == 0) { 0 } else { if (x == 1) { return 1; } else { fibonacci(x - 1) + fibonacci(x - 2); } } }; fibonacci(35);");
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        let mut compiler = Compiler::new();
        let _ = compiler.compile_program(program);

        let mut machine = Vm::new(compiler.bytecode());
        let _ = machine.run();
        println!("result: {:?}", machine.last_popped_element);
    });
}
