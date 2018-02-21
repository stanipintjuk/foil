use std::path::{Path, PathBuf};
use std::collections::HashMap;
use compiler::parser::ast::{Ast, SetField, Id};
use compiler::tokens::{BinOp, Val};
use super::scope::{Scope, OpenScope};
use super::output::{Output, Function};
use super::error::EvalError;
use compiler::lexer::Tokenizer;
use std::fs::File;
use compiler::parser::Parser;
use std::io::Read;
use super::binopevals::{
    eval_add, 
    eval_sub, 
    eval_mul, 
    eval_div,
    eval_mod,
    eval_pow,
    eval_equal,
};


pub type EvalResult =
Result<Output, EvalError>;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Evaluator<'scope, 'ast: 'scope> {
    expr: &'ast Ast,
    pub scope: Scope<'scope, 'ast>,
    base_dir: PathBuf,
}
impl<'scope, 'ast: 'scope> Evaluator<'scope, 'ast> {
    pub fn new(expr: &'ast Ast, scope: Scope<'scope, 'ast>) -> Self {
        Evaluator{expr: expr, scope: scope, base_dir: Path::new("./").to_owned()}
    }

    pub fn with_path(expr: &'ast Ast, scope: Scope<'scope, 'ast>, base_dir: PathBuf) -> Self {
        Evaluator{expr: expr, scope: scope, base_dir: base_dir}
    }

    pub fn eval(&self) -> EvalResult {
        match self.expr {
            &Ast::Let(ref field, ref child_expr) => self.eval_let(field, child_expr),
            &Ast::BinOp(ref binop, ref left, ref right) => 
                self.eval_binary_op(binop, left, right),
            &Ast::Val(ref val) => self.eval_val(val),
            &Ast::Id(ref id) => self.eval_id(id),
            &Ast::Fn(ref param, ref expr) => self.eval_fn(param, expr),
            &Ast::Call(ref func, ref input) => self.eval_call(func, input),
            &Ast::Import(_, ref file_name) => self.eval_file(file_name),
            _ => unimplemented!(),
        }
    }

    fn eval_file(&self, file_name: &str) -> EvalResult {
        let mut f = File::open(self.base_dir.join(file_name)).unwrap();
        let mut contents = String::new();
        
        let read_res = f.read_to_string(&mut contents);
        if let Err(err) = read_res {
            return Err(EvalError::IO(err));
        }

        let mut tokenizer = Tokenizer::new(&contents);
        let mut parser = Parser::new(&mut tokenizer);
        if let Some(parse_res) = parser.next() {
            match parse_res {
                Ok(ast) => {
                    let scope = OpenScope::new();
                    Evaluator::new(&ast,  Scope::Open(&scope)).eval()
                },
                Err(err) => Err(EvalError::Parser(err)),
            }
        } else {
            Err(EvalError::FileDoesNotContainExpression(file_name.to_string()))
        }
    }

    fn eval_fn(&self, param: &str, expr: &Ast) -> EvalResult {
        Ok(Output::Fn(Function::new(param.to_string(), Clone::clone(expr), self.scope.to_closed())))
    }

    fn eval_call(&self, func: &'ast Ast,  input: &'ast Ast) -> EvalResult {
        let func = Evaluator::new(func, self.scope.clone()).eval();
        if let Ok(Output::Fn(func)) = func {
            func.eval(input)
        } else if let Ok(not_func) = func {
            Err(EvalError::NotAFunction(not_func))
        } else {
            func
        }

    }

    fn eval_val(&self, val: &Val) -> EvalResult {
        match val {
            &Val::Int(v) => Ok(Output::Int(v)),
            _ => unimplemented!(),
        }
    }

    fn eval_id(&self, id: &Id) -> EvalResult {
        let id_name: &str = &id.1;
        if let Some(val) = self.scope.get_value(id_name) {
            val
        } else {
            Err(EvalError::IdNotFound(Clone::clone(id)))
        }
    }

    fn eval_binary_op(&self, binop: &'ast BinOp, left: &'ast Ast, right: &'ast Ast) -> EvalResult {
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

    fn eval_let(&self, field: &SetField, child_expr: &Ast) -> EvalResult {
        let mut map: HashMap<&str, _> = HashMap::new();
        map.insert(&field.name, Evaluator::new(&field.value, self.scope.clone()));
        let child_scope = OpenScope{ parent: Some(self.scope.clone()), map: map};
        let eval = Evaluator::new(child_expr, Scope::Open(&child_scope));
        eval.eval()
    }
}
