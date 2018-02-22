extern crate foil;
use foil::compiler::tokenizer::Tokenizer;
use foil::compiler::parser::Parser;
use foil::compiler::evaluator::{Evaluator, Scope, OpenScope, EvalResult, EvalError};

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{:?}", eval(line.unwrap()));
    }
}

fn eval(text: String) -> EvalResult {
    let mut tokenizer = Tokenizer::new(&text);
    let mut parser = Parser::new(&mut tokenizer);
    if let Some(parse_res) = parser.next() {
        match parse_res {
            Ok(ast) => {
                let scope = OpenScope::new();
                Evaluator::new(&ast,  Scope::Open(&scope)).eval()
            },
            Err(err) => Err(EvalError::Parser(err)),
        }
    } else {
        Err(EvalError::IOUnknown)
    }
}
