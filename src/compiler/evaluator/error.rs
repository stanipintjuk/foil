use compiler::tokens::{BinOp};
use compiler::parser::ast::{Id};
use super::output::Output;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum EvalError<'ast, 'text: 'ast> {
    IdNotFound(&'ast Id<'text>),
    InvalidBinOp(BinOp, Output<'ast, 'text>, Output<'ast, 'text>),
    NotAFunction(Output<'ast, 'text>),
}


