use compiler::evaluator::{Evaluator, EvalError, EvalResult, Output};
use compiler::{evaluate_file, write_to_file, copy_file};
use std::path::PathBuf;

pub fn evaluate_import<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, file_name: &str) -> EvalResult {
    let file = if let Some(ref file_path) = eval.file_path {
        let mut file = file_path.clone();
        file.pop();
        file
    } else {
        let mut file = PathBuf::from("./");
        file
    };

    let fall_back_out_dir = PathBuf::from("./");
    let out_dir = eval.out_path
        .as_ref()
        .unwrap_or(&fall_back_out_dir);

    let file = file.join(file_name);
    evaluate_file(&file, out_dir)
}
