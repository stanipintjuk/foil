use std::path::{PathBuf, Path};
use compiler::models::{Ast, Output};

use super::scope::Scope;
use compiler::errors::EvalError;

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
    /// Scope of the evaluation. Used for looking up function and variable references.
    pub scope: Scope<'scope, 'ast>,

    /// The directory to which processed files are written.
    /// If it is `None` then evaluation of paths will return `EvalError::OutputPathNotSpecified`.
    pub out_path: Option<PathBuf>,

    expr: &'ast Ast,
    file_path: Option<PathBuf>,
}
impl<'scope, 'ast: 'scope> Evaluator<'scope, 'ast> {
    /// # Arguments
    /// `expr` - The AST (Abstract Syntax Tree) to be evaluated.
    /// `scope` - The scope of the evaluation.
    /// `file_path` - The path to the file for which the AST has been evaluated.
    pub fn new(expr: &'ast Ast, scope: Scope<'scope, 'ast>, file_path: PathBuf, out_path: PathBuf) -> Self {
        Evaluator{expr: expr, scope: scope, file_path: Some(file_path), out_path: Some(out_path)}
    }

    /// Creates a new evaluator with no input file or output directory specified.
    ///
    /// # Arguments
    /// `expr` - The AST (Abstract Syntax Tree) to be evaluated.
    /// `scope` - The scope of the evaluation.
    pub fn without_files(expr: &'ast Ast, scope: Scope<'scope, 'ast>) -> Self {
        Evaluator{expr: expr, scope: scope, file_path: None, out_path: None}
    }

    /// Creates a new `Evaluator` with the same input file, out directory and scope for the given
    /// expression.
    pub fn copy_for_expr(&self, expr: &'ast Ast) -> Evaluator<'scope, 'ast> {
        Evaluator {
            scope: self.scope.clone(),
            expr: expr,
            file_path: self.file_path.clone(),
            out_path: self.out_path.clone()
        }
    }

    /// Create a new `Evaluator` with the same input and out directory but with a new scope.
    pub fn copy_for_child_expr(&self, expr: &'ast Ast, scope: Scope<'scope, 'ast>) -> Evaluator<'scope, 'ast> {
        Evaluator {
            scope: scope,
            expr: expr,
            file_path: self.file_path.clone(),
            out_path: self.out_path.clone(),
        }
    }

    pub fn get_working_dir(&self) -> Option<&Path> {
        self.file_path
            .as_ref()
            .map(PathBuf::as_path)
            .and_then(Path::parent)
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
