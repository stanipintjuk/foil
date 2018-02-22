use compiler::parser::ast::{Ast};
use compiler::tokenizer::tokens::{BinOp};
use super::evaluator::{Evaluator, EvalResult};
use super::error::EvalError;
use super::output::Output;

pub fn eval_add<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = Evaluator::new(left, eval.scope.clone()).eval();
    let right = Evaluator::new(right, eval.scope.clone()).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left + right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left + right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 + right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Add, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

pub fn eval_sub<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = Evaluator::new(left, eval.scope.clone()).eval();
    let right = Evaluator::new(right, eval.scope.clone()).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left - right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left - right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 - right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Sub, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

pub fn eval_mul<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = Evaluator::new(left, eval.scope.clone()).eval();
    let right = Evaluator::new(right, eval.scope.clone()).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left * right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left * right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 * right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mul, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

pub fn eval_div<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = Evaluator::new(left, eval.scope.clone()).eval();
    let right = Evaluator::new(right, eval.scope.clone()).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left / right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left / right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 / right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Div, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

pub fn eval_mod<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = Evaluator::new(left, eval.scope.clone()).eval();
    let right = Evaluator::new(right, eval.scope.clone()).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left % right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left % right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 % right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mod, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

pub fn eval_pow<'scope, 'ast: 'scope>(_eval: &Evaluator<'scope, 'ast>, _left: &Ast, _right: &Ast) -> EvalResult {
    panic!("Powers not yet supported");
}

pub fn eval_equal<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = Evaluator::new(left, eval.scope.clone()).eval();
    let right = Evaluator::new(right, eval.scope.clone()).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Bool(left == right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Bool(left == right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Bool(left as f64 == right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Equals, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}
