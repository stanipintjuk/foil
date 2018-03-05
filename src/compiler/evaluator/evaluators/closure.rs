use compiler::evaluator::{Evaluator, EvalResult, Output, Closure};
use compiler::parser::ast::Ast;

pub fn evaluate_closure<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, param: &str, expr: &Ast) -> EvalResult {
    Ok(Output::Fn(Closure::new(param.to_string(), 
                               Clone::clone(expr), 
                               eval.scope.to_closed())))
}
