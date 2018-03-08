use compiler::evaluator::{Evaluator, EvalResult};
use compiler::{evaluate_file};
use std::path::{PathBuf, Path};

pub fn evaluate_import<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, file_name: &str) -> EvalResult {
    let fallback_dir = PathBuf::from("./");

    let import_file = eval
        .get_working_dir()
        .unwrap_or(&fallback_dir)
        .join(file_name);

    let out_dir = eval.out_path
        .as_ref()
        .unwrap_or(&fallback_dir);

    evaluate_file(&import_file, out_dir)
}
