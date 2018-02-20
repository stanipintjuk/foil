use std::collections::HashMap;
use compiler::parser::ast::{Ast};
use super::scope::{OpenScope, ClosedScope, Scope};
use super::evaluator::{Evaluator, EvalResult};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Output<'ast, 'text: 'ast> {
    Int(i64),
    Double(f64),
    Bool(bool),
    Fn(Function<'ast, 'text>),
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Function<'ast, 'text: 'ast> {
    param_name: &'text str,
    expr: &'ast Ast<'text>,
    scope: ClosedScope<'ast, 'text>,
}

impl<'ast, 'text: 'ast> Function<'ast, 'text> {
    pub fn new(param_name: &'text str, expr: &'ast Ast<'text>, scope: ClosedScope<'ast, 'text>) -> Self {
        Function{
            param_name: param_name,
            scope: scope, 
            expr: expr}
    }

    pub fn eval(&self, param_value: &'ast Ast<'text>) -> EvalResult<'ast, 'text> {
        let mut map = HashMap::new();
        let param_eval = Evaluator::new(param_value, 
                                        Scope::Closed(&self.scope));
        map.insert(self.param_name, param_eval);

        let child_scope = OpenScope{
                map: map, 
                parent: Some(Scope::Closed(&self.scope))
            };
        let eval = Evaluator::new(self.expr, Scope::Open(&child_scope));
        eval.eval()
    }
}

