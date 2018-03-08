use compiler::models::{Ast, BinOp, Output};
use compiler::evaluator::{Evaluator, EvalResult, EvalError};

pub fn evaluate_binop<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, binop: &'ast BinOp, left: &'ast Ast, right: &'ast Ast) -> EvalResult {
    match binop {
        &BinOp::Add => eval_add(eval, left, right),
        &BinOp::Sub => eval_sub(eval, left, right),
        &BinOp::Mul => eval_mul(eval, left, right),
        &BinOp::Div => eval_div(eval, left, right),
        &BinOp::Mod => eval_mod(eval, left, right),
        &BinOp::Pow => eval_pow(eval, left, right),
        &BinOp::Equals => eval_equal(eval, left, right),
    }
}

fn eval_add<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = eval.copy_for_expr(left).eval();
    let right = eval.copy_for_expr(right).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left + right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left + right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 + right)),
        (Ok(Output::Int(left)), Ok(Output::String(right))) => Ok(Output::String(format!("{}{}", left, right))),
        (Ok(Output::String(left)), Ok(Output::Int(right))) => Ok(Output::String(format!("{}{}", left, right))),
        (Ok(Output::Double(left)), Ok(Output::String(right))) => Ok(Output::String(format!("{}{}", left, right))),
        (Ok(Output::String(left)), Ok(Output::Double(right))) => Ok(Output::String(format!("{}{}", left, right))),
        (Ok(Output::String(left)), Ok(Output::String(right))) => Ok(Output::String(format!("{}{}", left, right))),
        (Ok(Output::Bool(left)), Ok(Output::String(right))) => Ok(Output::String(format!("{}{}", left, right))),
        (Ok(Output::String(left)), Ok(Output::Bool(right))) => Ok(Output::String(format!("{}{}", left, right))),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Add, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn eval_sub<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = eval.copy_for_expr(left).eval();
    let right = eval.copy_for_expr(right).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left - right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left - right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 - right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Sub, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn eval_mul<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = eval.copy_for_expr(left).eval();
    let right = eval.copy_for_expr(right).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left * right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left * right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 * right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mul, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn eval_div<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = eval.copy_for_expr(left).eval();
    let right = eval.copy_for_expr(right).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left / right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left / right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 / right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Div, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn eval_mod<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = eval.copy_for_expr(left).eval();
    let right = eval.copy_for_expr(right).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Int(left % right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Double(left % right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Double(left as f64 % right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mod, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn eval_pow<'scope, 'ast: 'scope>(_eval: &Evaluator<'scope, 'ast>, _left: &Ast, _right: &Ast) -> EvalResult {
    panic!("Powers not yet supported");
}

fn eval_equal<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, left: &Ast, right: &Ast) -> EvalResult {
    let left = eval.copy_for_expr(left).eval();
    let right = eval.copy_for_expr(right).eval();

    match (left, right) {
        (Ok(Output::Int(left)), Ok(Output::Int(right))) => Ok(Output::Bool(left == right)),
        (Ok(Output::Double(left)), Ok(Output::Int(right))) => Ok(Output::Bool(left == right as f64)),
        (Ok(Output::Int(left)), Ok(Output::Double(right))) => Ok(Output::Bool(left as f64 == right)),
        (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Equals, l, r)),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}
