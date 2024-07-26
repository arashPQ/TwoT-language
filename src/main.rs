use std::io;
use crate::repl::start;

pub mod ast;
//pub mod builtins;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    println!("\n Hello, You are using the TwoT language!! \n");
    start(io::stdin(), io::stdout());
}
