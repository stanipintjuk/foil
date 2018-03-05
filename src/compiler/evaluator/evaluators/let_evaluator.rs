use compiler::parser::ast::{Ast, SetField};
use compiler::evaluator::{Evaluator, EvalResult, Scope, OpenScope};
use std::collections::HashMap;

pub fn evaluate_let<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, field: &SetField, child_expr: &Ast) -> EvalResult {
    let mut map: HashMap<&str, _> = HashMap::new();
    map.insert(&field.name, eval.copy_for_expr(&field.value));
    let child_scope = OpenScope{ parent: Some(eval.scope.clone()), map: map};
    let eval = eval.copy_for_child_expr(child_expr, Scope::Open(&child_scope));
    eval.eval()
}
