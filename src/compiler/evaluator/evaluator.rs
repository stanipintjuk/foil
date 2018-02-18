use compiler::parser::ast::{Ast};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Output {
    Int(i64),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum EvalError {

}

pub trait Evaluatable {
    fn evaluate(&self) -> Result<Output, EvalError>;
}

impl<'s> Evaluatable for Ast<'s> {
    fn evaluate(&self) -> Result<Output, EvalError> {
        Ok(Output::Int(0))
    }
}
