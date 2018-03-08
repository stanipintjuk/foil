use std::collections::HashMap;
use compiler::models::Ast;
use compiler::evaluator::{OpenScope, ClosedScope, Scope, Evaluator, EvalResult};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Closure {
    pub param_name: String,
    pub expr: Ast,
    scope: ClosedScope,
}

impl Closure {
    pub fn new(param_name: String, expr: Ast, scope: ClosedScope) -> Self {
        Closure{
            param_name: param_name,
            scope: scope, 
            expr: expr
        }
    }

    pub fn eval(&self, param_value: &Ast) -> EvalResult {
        let mut map: HashMap<&str, _> = HashMap::new();
        let param_eval = Evaluator::without_files(param_value, Scope::Closed(&self.scope));
        map.insert(&self.param_name, param_eval);

        let child_scope = OpenScope{
                map: map, 
                parent: Some(Scope::Closed(&self.scope))
            };
        let eval = Evaluator::without_files(&self.expr, Scope::Open(&child_scope));
        eval.eval()
    }
}
