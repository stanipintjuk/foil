use compiler::tokens::{BinOp};
use compiler::parser::ast::{Id};
use super::output::Output;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum EvalError<'a, 's: 'a> {
    IdNotFound(&'a Id<'s>),
    InvalidBinOp(BinOp, Output, Output),
}


