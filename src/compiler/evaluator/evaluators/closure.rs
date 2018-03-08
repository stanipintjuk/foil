use compiler::models::{Ast, Closure, Output};
use compiler::evaluator::{Evaluator, EvalResult};

pub fn evaluate_closure<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, param: &str, expr: &Ast) -> EvalResult {
    let closure = Closure::new(
        param.to_string(), 
        expr.clone(), 
        eval.scope.to_closed()
    );
    Ok(Output::Fn(closure))
}
