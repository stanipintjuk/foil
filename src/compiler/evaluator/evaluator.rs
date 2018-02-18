use compiler::parser::ast::{Ast, SetField, Id};
use compiler::tokens::{BinOp, Val};
use std::collections::LinkedList;
use std::collections::HashMap;
use std::ops;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Output {
    Int(i64),
    Double(f64),
    Bool(bool),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum EvalError<'a, 's: 'a> {
    IdNotFound(&'a Id<'s>),
    InvalidBinOp(BinOp, Output, Output),
}

pub struct Scope<'parent, 'ast: 'parent, 'text: 'ast> {
    parent: Option<&'parent Scope<'parent, 'ast, 'text>>,
    map: HashMap<&'text str, Evaluator<'parent, 'ast, 'text>>,
}
impl<'parent, 'ast: 'parent, 'text: 'ast> Scope<'parent, 'ast, 'text> {
    pub fn new() -> Self {
        Scope{parent: None, map: HashMap::new()}
    }

    fn empty(parent: &'parent Scope<'parent, 'ast, 'text>) -> Self {
        Scope{parent: Some(parent), map: HashMap::new()}
    }

    fn get_value(&self, id_name: &'text str) -> Option<&Evaluator<'parent, 'ast, 'text>> {
        if let Some(eval) = self.map.get(id_name) {
            Some(eval)
        } else if let Some(parent) = self.parent {
            parent.get_value(id_name)
        } else {
            None
        }
    }
}

pub struct Evaluator<'scope, 'ast: 'scope, 'text: 'ast> {
    expr: &'ast Ast<'text>,
    scope: &'scope Scope<'scope, 'ast, 'text>,
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
            &BinOp::Add => self.eval_add(left, right),
            &BinOp::Sub => self.eval_sub(left, right),
            &BinOp::Mul => self.eval_mul(left, right),
            &BinOp::Div => self.eval_div(left, right),
            &BinOp::Mod => self.eval_mod(left, right),
            &BinOp::Pow => self.eval_pow(left, right),
            &BinOp::Assign => panic!("Tell Stani that he is a 
            dumbass and that `=` is technically not a binary operator"),
            &BinOp::Equals => self.eval_equal(left, right),
        }
    }

    fn eval_add(&self, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let left = Evaluator::new(left, self.scope).eval();
        let right = Evaluator::new(right, self.scope).eval();

        match (left, right) {
            (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
                Ok(Output::Int(left + right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left + right as f64)),
            (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                Ok(Output::Double(left as f64 + right)),
            (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Add, l, r)),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }

    fn eval_sub(&self, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let left = Evaluator::new(left, self.scope).eval();
        let right = Evaluator::new(right, self.scope).eval();

        match (left, right) {
            (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
                Ok(Output::Int(left - right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left - right as f64)),
            (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                Ok(Output::Double(left as f64 - right)),
            (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Sub, l, r)),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }

    fn eval_mul(&self, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let left = Evaluator::new(left, self.scope).eval();
        let right = Evaluator::new(right, self.scope).eval();

        match (left, right) {
            (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
                Ok(Output::Int(left * right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left * right as f64)),
            (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                Ok(Output::Double(left as f64 * right)),
            (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mul, l, r)),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }

    fn eval_div(&self, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let left = Evaluator::new(left, self.scope).eval();
        let right = Evaluator::new(right, self.scope).eval();

        match (left, right) {
            (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
                Ok(Output::Int(left / right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left / right as f64)),
            (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                Ok(Output::Double(left as f64 / right)),
            (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Div, l, r)),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }

    fn eval_mod(&self, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let left = Evaluator::new(left, self.scope).eval();
        let right = Evaluator::new(right, self.scope).eval();

        match (left, right) {
            (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
                Ok(Output::Int(left % right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Double(left % right as f64)),
            (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                Ok(Output::Double(left as f64 % right)),
            (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Mod, l, r)),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }

    fn eval_pow(&self, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        panic!("Powers not yet supported");
    }

    fn eval_equal(&self, left: &'ast Ast<'text>, right: &'ast Ast<'text>) -> Result<Output, EvalError<'ast, 'text>> {
        let left = Evaluator::new(left, self.scope).eval();
        let right = Evaluator::new(right, self.scope).eval();

        match (left, right) {
            (Ok(Output::Int(left)), Ok(Output::Int(right))) => 
                Ok(Output::Bool(left == right)),
            (Ok(Output::Double(left)), Ok(Output::Int(right))) => 
                Ok(Output::Bool(left == right as f64)),
            (Ok(Output::Int(left)), Ok(Output::Double(right))) => 
                Ok(Output::Bool(left as f64 == right)),
            (Ok(l), Ok(r)) => Err(EvalError::InvalidBinOp(BinOp::Equals, l, r)),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
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
