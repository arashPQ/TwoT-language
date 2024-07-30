use std::io;
use crate::repl::start;

pub mod ast;
pub mod builtins;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    println!("Hello, You are using the TwoT language!!");
    println!("version 0.0.1");
    println!("This language Developed by arash :) \n");
    start(io::stdin(), io::stdout());
}
