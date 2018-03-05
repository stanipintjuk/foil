use std::path::PathBuf;
use compiler::parser::ast::Ast;

use super::scope::Scope;
use super::output::{Output};
use super::error::EvalError;

use super::evaluators::{
    evaluate_binop, 
    evaluate_html, 
    evaluate_html_closed,
    evaluate_import,
    evaluate_closure,
    evaluate_call,
    evaluate_id,
    evaluate_val,
    evaluate_let,
};

pub type EvalResult = Result<Output, EvalError>;

/// A struct that holds all the relevant information to evaluate an AST (Abstract Syntax Tree)
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Evaluator<'scope, 'ast: 'scope> {
    /// Scope of the evaluation
    pub scope: Scope<'scope, 'ast>,

    pub expr: &'ast Ast,
    pub file_path: Option<PathBuf>,
    pub out_path: Option<PathBuf>,
}
impl<'scope, 'ast: 'scope> Evaluator<'scope, 'ast> {

    /// Creates a new evaluator with no input file or output directory specified.
    ///
    /// # Arguments
    /// `expr` - The AST (Abstract Syntax Tree) to be evaluated.
    /// `scope` - The scope of the evaluation.
    pub fn new(expr: &'ast Ast, scope: Scope<'scope, 'ast>) -> Self {
        Evaluator{expr: expr, scope: scope, file_path: None, out_path: None}
    }

    /// # Arguments
    /// `expr` - The AST (Abstract Syntax Tree) to be evaluated.
    /// `scope` - The scope of the evaluation.
    /// `file_path` - The path to the file for which the AST has been evaluated.
    pub fn with_file(expr: &'ast Ast, scope: Scope<'scope, 'ast>, file_path: PathBuf, out_path: PathBuf) -> Self {
        Evaluator{expr: expr, scope: scope, file_path: Some(file_path), out_path: Some(out_path)}
    }

    /// Evaluates the expression
    pub fn eval(&self) -> EvalResult {
        match self.expr {
            &Ast::BinOp(ref binop, ref left, ref right) => evaluate_binop(self, binop, left, right),
            &Ast::Val(ref val) => evaluate_val(self, val),
            &Ast::Set(_) => panic!("Evaluation of sets is not implemented"),
            &Ast::Let(ref field, ref child_expr) => evaluate_let(self, field, child_expr),
            &Ast::Fn(ref param, ref expr) => evaluate_closure(self, param, expr),
            &Ast::Call(ref func, ref input) => evaluate_call(self, func, input),
            &Ast::Id(ref id) => evaluate_id(self, id),
            &Ast::Import(_, ref relative_path) => evaluate_import(self, relative_path),
            &Ast::Html{ref tag_name, ref attributes, ref children} => evaluate_html(self, tag_name, attributes, children),
            &Ast::HtmlClosed{ref tag_name, ref attributes} => evaluate_html_closed(self, tag_name, attributes),
        }
    }
}
