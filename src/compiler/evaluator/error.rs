use compiler::tokens::{BinOp};
use compiler::parser::ast::{Id};
use super::output::Output;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum EvalError {
    IdNotFound(Id),
    InvalidBinOp(BinOp, Output, Output),
    NotAFunction(Output),
}


