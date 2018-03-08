use compiler::models::{Ast, Output};
use compiler::evaluator::{Evaluator, EvalResult, EvalError};

/// Evaluates a function call
/// # Arguments
/// `func` - the expression that returns a function
/// `input` - the expression that returns the parameter for the function
pub fn evaluate_call<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, func: &'ast Ast,  input: &'ast Ast) -> EvalResult {
    let func = eval.copy_for_expr(func).eval();
    if let Ok(Output::Fn(func)) = func {
        func.eval(input)
    } else if let Ok(not_func) = func {
        Err(EvalError::NotAFunction(not_func))
    } else {
        func
    }

}
