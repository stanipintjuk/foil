use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::Path;

use super::evaluator::{Evaluator, Output, EvalError, EvalResult, Scope, OpenScope};
use super::tokenizer::Tokenizer;
use super::parser::Parser;

pub fn evaluate_string(text: &str, file_path: &Path, out_dir: &Path) -> EvalResult {
    let mut tokenizer = Tokenizer::new(&text);
    let mut parser = Parser::new(&mut tokenizer);
    if let Some(parse_res) = parser.next() {
        match parse_res {
            Ok(ast) => {
                let scope = OpenScope::new();
                Evaluator::new(&ast, Scope::Open(&scope), file_path.to_owned(), out_dir.to_owned()).eval()
            },
            Err(err) => Err(EvalError::Parser(err)),
        }
    } else {
        Err(EvalError::FileDoesNotContainExpression(file_path.to_path_buf()))
    }
}

/// Reads the file `file_path` and evaluates it's contents.
/// Returns `EvalError::NotFile` if the file could not be opened.
pub fn evaluate_file(file_path: &Path, out_dir: &Path) -> EvalResult  {
    let mut f = match File::open(&file_path) {
        Ok(f) => f,
        Err(_err) => {
            return Err(EvalError::NotFile(file_path.to_str().unwrap_or("None").to_string()));
        }
    };
    let mut contents = String::new();

    let read_res = f.read_to_string(&mut contents);
    if let Err(err) = read_res {
        return Err(EvalError::IO(err));
    }

    evaluate_string(&contents, &file_path, &out_dir)
}

pub fn build_file(file_path: &Path, out_dir: &Path) -> Result<(), EvalError>  {
    let res = evaluate_file(file_path, out_dir);
    match res {
        Ok(output) => {
            match output.to_string() {
                Ok(output) => {
                    let mut out_index_file = out_dir.join(file_path.file_stem().unwrap());   
                    out_index_file.set_extension("html");
                    let outstr = format!("{}", output);
                    write_to_file(&outstr, &out_index_file);
                    Ok(())
                },
                Err(err) => {
                    return Err(err);
                }
            }
        },
        Err(err) => Err(err),
    }
}

pub fn write_to_file(text: &str, path: &Path) -> EvalResult {
    let mut f = match File::create(&path) {
        Ok(f) => f,
        Err(err) => { return Err(EvalError::IO(err)) },
    };

    match f.write_all(text.as_bytes()) {
        Err(err) => { return Err(EvalError::IO(err))},
        _ => {},
    }
    match f.sync_all() {
        Ok(_) => Ok(Output::String(path.to_str().unwrap().to_string())),
        Err(err) => Err(EvalError::IO(err)),
    }
}

pub fn copy_file(from: &Path, to: &Path) -> EvalResult {
    let res = fs::copy(from, to);
    match res {
        Ok(_) => Ok(Output::String(from.to_str().unwrap().to_string())),
        Err(err) => Err(EvalError::IO(err)),
    }
}
