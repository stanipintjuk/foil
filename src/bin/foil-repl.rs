extern crate foil;
use foil::compiler::tokenizer::Tokenizer;
use foil::compiler::parser::Parser;
use foil::compiler::evaluator::{Evaluator, Scope, OpenScope, EvalResult};
use foil::compiler::errors::{EvalError, ParseError};

use std::io::{self, BufRead};
use std::io::prelude::*;

const FOIL_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const FOIL_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const FOIL_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

fn main() {
    println!("Foil version {} {}", FOIL_VERSION, FOIL_HOMEPAGE);
    println!("Contact authors: {}", FOIL_AUTHORS);
    let stdin = io::stdin();
    let mut cmd = "".to_string();

    print!(">>> ");
    io::stdout().flush();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        cmd = format!("{}\n{}", cmd, line);

        let mut should_run = true;
        match eval(&cmd) {
            Err(EvalError::Parser(ParseError::UnexpectedEndOfCode(_))) => {
                should_run = false;
            },
            Err(err) => {
                println!("{:?}", err);
                cmd = "".to_string();
            },
            Ok(output) => {
                println!("{}", output);
                cmd = "".to_string();
            }
        }
        if should_run {
            print!(">>> ");
        } else {
            print!("... ");
        }
        io::stdout().flush();
    }
}

fn eval(text: &str) -> EvalResult {
    let mut tokenizer = Tokenizer::new(text);
    let mut parser = Parser::new(&mut tokenizer);
    if let Some(parse_res) = parser.next() {
        match parse_res {
            Ok(ast) => {
                let scope = OpenScope::new();
                Evaluator::without_files(&ast,  Scope::Open(&scope)).eval()
            },
            Err(err) => Err(EvalError::Parser(err)),
        }
    } else {
        Err(EvalError::IOUnknown)
    }
}
