use std::collections::HashMap;
use compiler::parser::ast::{Ast, SetField, Id};
use compiler::tokens::{BinOp, Val};
use super::scope::Scope;
use super::output::Output;
use super::error::EvalError;
use super::binopevals::{
    eval_add, 
    eval_sub, 
    eval_mul, 
    eval_div,
    eval_mod,
    eval_pow,
    eval_equal,
};


pub struct Evaluator<'scope, 'ast: 'scope, 'text: 'ast> {
    expr: &'ast Ast<'text>,
    pub scope: &'scope Scope<'scope, 'ast, 'text>,
}
impl<'scope, 'ast: 'scope, 'text: 'ast> Evaluator<'scope, 'ast, 'text> {
    pub fn new(expr: &'ast Ast<'text>, scope: &'scope Scope<'scope, 'ast, 'text>) -> Self {
        Evaluator{expr: expr, scope: scope}
    }

    pub fn eval(&self) -> Result<Output, EvalError<'ast, 'text>> {
        match self.expr {
            &Ast::Let(ref field, ref child_expr) => self.eval_let(field, child_expr),
            &Ast::BinOp(ref binop, ref left, ref right) => 
                self.eval_binary_op(binop, left, right),
            &Ast::Val(ref val) => self.eval_val(val),
            &Ast::Id(ref id) => self.eval_id(id),
            _ => unimplemented!(),
        }
    }

    fn eval_val(&self, val: &Val<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        match val {
            &Val::Int(v) => Ok(Output::Int(v)),
            _ => unimplemented!(),
        }
    }

    fn eval_id(&self, id: &'ast Id<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let id_name: &str = id.1;
        if let Some(val) = self.scope.get_value(id_name) {
            val.eval()
        } else {
            Err(EvalError::IdNotFound(id))
        }
    }

    fn eval_binary_op(&self, binop: &'ast BinOp, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        match binop {
            &BinOp::Add => eval_add(self, left, right),
            &BinOp::Sub => eval_sub(self, left, right),
            &BinOp::Mul => eval_mul(self, left, right),
            &BinOp::Div => eval_div(self, left, right),
            &BinOp::Mod => eval_mod(self, left, right),
            &BinOp::Pow => eval_pow(self, left, right),
            &BinOp::Assign => panic!("Tell Stani that he is a 
            dumbass and that `=` is technically not a binary operator"),
            &BinOp::Equals => eval_equal(self, left, right),
        }
    }

    fn eval_let(&self, field: &'ast SetField<'text>, child_expr: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let mut map = HashMap::new();
        map.insert(field.name, Evaluator::new(&field.value, &self.scope));
        let child_scope = Scope{ parent: Some(self.scope), map: map};
        let eval = Evaluator::new(child_expr, &child_scope);
        eval.eval()
    }
}
