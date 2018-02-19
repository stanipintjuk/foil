use compiler::parser::ast::{Ast};
use compiler::tokens::{BinOp};
use super::evaluator::Evaluator;
use super::error::EvalError;
use super::output::Output;

pub fn eval_add<'scope, 'ast: 'scope, 'text: 'ast>(eval: &Evaluator<'scope, 'ast, 'text>, 
                                                   left: &'ast Ast<'text>, 
                                                   right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
    let left = Evaluator::new(left, eval.scope).eval();
    let right = Evaluator::new(right, eval.scope).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
            Ok(Output::Int(left + right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left + right as f64)),
                (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                    Ok(Output::Double(left as f64 + right)),
                    (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Add, l, r)),
                    (Err(err), _) => Err(err),
                    (_, Err(err)) => Err(err),
    }
}

pub fn eval_sub<'scope, 'ast: 'scope, 'text: 'ast>(eval: &Evaluator<'scope, 'ast, 'text>, 
                                   left: &'ast Ast<'text>, 
                                   right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
    let left = Evaluator::new(left, eval.scope).eval();
    let right = Evaluator::new(right, eval.scope).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
            Ok(Output::Int(left - right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left - right as f64)),
                (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                    Ok(Output::Double(left as f64 - right)),
                    (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Sub, l, r)),
                    (Err(err), _) => Err(err),
                    (_, Err(err)) => Err(err),
    }
}

pub fn eval_mul<'scope, 'ast: 'scope, 'text: 'ast>(eval: &Evaluator<'scope, 'ast, 'text>, 
                               left: &'ast Ast<'text>, 
                               right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
    let left = Evaluator::new(left, eval.scope).eval();
    let right = Evaluator::new(right, eval.scope).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
            Ok(Output::Int(left * right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left * right as f64)),
                (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                    Ok(Output::Double(left as f64 * right)),
                    (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mul, l, r)),
                    (Err(err), _) => Err(err),
                    (_, Err(err)) => Err(err),
    }
}

pub fn eval_div<'scope, 'ast: 'scope, 'text: 'ast>(eval: &Evaluator<'scope, 'ast, 'text>, 
                               left: &'ast Ast<'text>, 
                               right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
    let left = Evaluator::new(left, eval.scope).eval();
    let right = Evaluator::new(right, eval.scope).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
            Ok(Output::Int(left / right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left / right as f64)),
                (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                    Ok(Output::Double(left as f64 / right)),
                    (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Div, l, r)),
                    (Err(err), _) => Err(err),
                    (_, Err(err)) => Err(err),
    }
}

pub fn eval_mod<'scope, 'ast: 'scope,  'text: 'ast>(eval: &Evaluator<'scope, 'ast, 'text>,
                                    left: &'ast 
                                    Ast<'text>, 
                                    right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
    let left = Evaluator::new(left, eval.scope).eval();
    let right = Evaluator::new(right, eval.scope).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
            Ok(Output::Int(left % right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left % right as f64)),
                (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                    Ok(Output::Double(left as f64 % right)),
                    (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mod, l, r)),
                    (Err(err), _) => Err(err),
                    (_, Err(err)) => Err(err),
    }
}

pub fn eval_pow<'scope, 'ast: 'scope, 'text: 'ast>(eval: &Evaluator<'scope, 'ast, 'text>, 
                                                   _left: &'ast Ast<'text>, 
                                                   _right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
    panic!("Powers not yet supported");
}

pub fn eval_equal<'scope, 'ast: 'scope, 'text: 'ast>(eval: &Evaluator<'scope, 'ast, 'text>, 
                                                     left: &'ast Ast<'text>, 
                                                     right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
    let left = Evaluator::new(left, eval.scope).eval();
    let right = Evaluator::new(right, eval.scope).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
            Ok(Output::Bool(left == right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Bool(left == right as f64)),
                (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                    Ok(Output::Bool(left as f64 == right)),
                    (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Equals, l, r)),
                    (Err(err), _) => Err(err),
                    (_, Err(err)) => Err(err),
    }
}


