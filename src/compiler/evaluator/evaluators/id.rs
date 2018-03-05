use compiler::parser::ast::{Id};
use compiler::evaluator::{Evaluator, EvalResult, EvalError};

/// Evaluates an "Id" (a function or  a variable reference) by doing a lookup in the scope
pub fn evaluate_id<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, id: &Id) -> EvalResult {
    let id_name: &str = &id.1;
    if let Some(val) = eval.scope.get_value(id_name) {
        val
    } else {
        Err(EvalError::IdNotFound(Clone::clone(id)))
    }
}

