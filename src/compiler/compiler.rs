use std::io::Read;
use std::fs::File;
use std::path::Path;

use super::evaluator::{Evaluator, EvalError, EvalResult, Scope, OpenScope};
use super::tokenizer::Tokenizer;
use super::parser::Parser;

pub fn evaluate_string(text: &str, file_path: &Path) -> EvalResult {
    let mut tokenizer = Tokenizer::new(&text);
    let mut parser = Parser::new(&mut tokenizer);
    if let Some(parse_res) = parser.next() {
        match parse_res {
            Ok(ast) => {
                let scope = OpenScope::new();
                Evaluator::with_file(&ast, Scope::Open(&scope), file_path.to_owned()).eval()
            },
            Err(err) => Err(EvalError::Parser(err)),
        }
    } else {
        Err(EvalError::FileDoesNotContainExpression(file_path.to_path_buf()))
    }
}

pub fn evaluate_file(file_path: &Path) -> EvalResult  {
    let mut f = File::open(&file_path).unwrap();
    let mut contents = String::new();

    let read_res = f.read_to_string(&mut contents);
    if let Err(err) = read_res {
        return Err(EvalError::IO(err));
    }

    evaluate_string(&contents, &file_path)
}
