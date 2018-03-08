use compiler::models::{Val, Output};
use compiler::evaluator::{Evaluator, EvalResult};
use std::path::{Path, PathBuf};
use super::evaluate_path;

pub fn evaluate_val<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, val: &Val) -> EvalResult {
    let fall_back_dir = PathBuf::from("./");
    let working_dir: &Path = 
        eval.get_working_dir().unwrap_or(&fall_back_dir);

    let out_path: &Option<&Path> = &eval.out_path
        .as_ref()
        .map(PathBuf::as_path);

    match val {
        &Val::Int(v) => Ok(Output::Int(v)),
        &Val::Double(v) => Ok(Output::Double(v)),
        &Val::String(ref v) => Ok(Output::String(v.to_string())),
        &Val::Path(ref v) => evaluate_path(v, working_dir, out_path),
        &Val::Bool(ref b) => Ok(Output::Bool(*b)),
    }
}
