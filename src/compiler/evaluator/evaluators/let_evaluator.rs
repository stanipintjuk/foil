use compiler::parser::ast::{Ast, SetField};
use compiler::evaluator::{Evaluator, EvalResult, Scope, OpenScope};
use std::collections::HashMap;

pub fn evaluate_let<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, field: &SetField, child_expr: &Ast) -> EvalResult {
    let mut map: HashMap<&str, _> = HashMap::new();
    map.insert(&field.name, Evaluator{
        scope: eval.scope.clone(),
        expr: &field.value,
        out_path: eval.out_path.clone(),
        file_path: eval.file_path.clone()
    });
    let child_scope = OpenScope{ parent: Some(eval.scope.clone()), map: map};
    let eval = Evaluator{
        scope: Scope::Open(&child_scope),
        expr: child_expr,
        file_path: eval.file_path.clone(),
        out_path: eval.out_path.clone(),
    };
    eval.eval()
}
