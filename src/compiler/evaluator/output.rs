use std::collections::HashMap;
use compiler::parser::ast::{Ast};
use super::scope::{OpenScope, ClosedScope, Scope};
use super::evaluator::{Evaluator, EvalResult};
use super::error::EvalError;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Output {
    Int(i64),
    Double(f64),
    Bool(bool),
    String(String),
    Fn(Function),
}

impl Output {
    pub fn to_string(self) -> Result<String, EvalError> {
        match self {
            Output::Int(x) => Ok(format!("{}", x)),
            Output::Bool(x) => Ok(format!("{}", x)),
            Output::String(x) => Ok(x),
            non_content => Err(EvalError::NotStringable(non_content)),
        }
    }

    pub fn is_stringable(&self) -> bool {
        match self {
            &Output::Int(_) | &Output::Double(_) | 
            &Output::Bool(_) | &Output::String(_) => true,
            &Output::Fn(_) => false,
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Function {
    param_name: String,
    expr: Ast,
    scope: ClosedScope,
}

impl Function {
    pub fn new(param_name: String, expr: Ast, scope: ClosedScope) -> Self {
        Function{
            param_name: param_name,
            scope: scope, 
            expr: expr}
    }

    pub fn eval(&self, param_value: &Ast) -> EvalResult {
        let mut map: HashMap<&str, _> = HashMap::new();
        let param_eval = Evaluator::new(param_value, Scope::Closed(&self.scope));
        map.insert(&self.param_name, param_eval);

        let child_scope = OpenScope{
                map: map, 
                parent: Some(Scope::Closed(&self.scope))
            };
        let eval = Evaluator::new(&self.expr, Scope::Open(&child_scope));
        eval.eval()
    }
}

