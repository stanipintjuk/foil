use compiler::tokens::{BinOp};
use compiler::parser::ast::{Id};
use compiler::parser::ParseError;
use super::output::Output;
use std::io::{Error as IOError};

#[derive(Debug)]
pub enum EvalError {
    IdNotFound(Id),
    InvalidBinOp(BinOp, Output, Output),
    NotAFunction(Output),
    Parser(ParseError),
    FileDoesNotContainExpression(String),
    IO(IOError),
    IOUnknown,
}
impl PartialEq for EvalError {
    fn eq(&self, other: &EvalError) -> bool {
        match (self, other) {
            (&EvalError::IdNotFound(ref l), &EvalError::IdNotFound(ref r)) => l == r,
            (&EvalError::InvalidBinOp(ref lop, ref lo1, ref lo2), 
             &EvalError::InvalidBinOp(ref rop, ref ro1, ref ro2)) => lop == rop && lo1 == ro1 && lo2 == ro2,
            (&EvalError::NotAFunction(ref l), &EvalError::NotAFunction(ref r)) => l == r,
            (&EvalError::Parser(ref l), &EvalError::Parser(ref r)) => l == r,
            (&EvalError::FileDoesNotContainExpression(ref l),
             &EvalError::FileDoesNotContainExpression(ref r)) => l == r,
            (&EvalError::IO(_), &EvalError::IO(_)) => true,
            (&EvalError::IOUnknown, &EvalError::IOUnknown) => true,
            (_, _) => false,
        }
    }
}

impl Clone for EvalError {
    fn clone(&self) -> EvalError {
        match self {
            &EvalError::IdNotFound(ref x) => EvalError::IdNotFound(x.clone()),
            &EvalError::InvalidBinOp(ref x, ref y, ref z) => 
                EvalError::InvalidBinOp(x.clone(), y.clone(), z.clone()),
            &EvalError::NotAFunction(ref x) => EvalError::NotAFunction(x.clone()),
            &EvalError::Parser(ref x) => EvalError::Parser(x.clone()),
            &EvalError::FileDoesNotContainExpression(ref x) =>
                EvalError::FileDoesNotContainExpression(x.clone()),

            // Losing information about the IO error because IOError doesn't implement clone
            &EvalError::IO(_) => EvalError::IOUnknown,

            &EvalError::IOUnknown => EvalError::IOUnknown,
        }
    }
}
