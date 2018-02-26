use compiler::evaluator::error::EvalError;
use compiler::evaluator::{EvalResult, Output};
use compiler::{evaluate_file, write_to_file, copy_file};
use std::path::Path;
use std::ffi::OsStr;

pub fn process_path(path: &str, src_path: &Path, out_path: &Path) -> EvalResult {
    build_path(path, src_path, out_path)
}

fn build_path(file: &str, src_path: &Path, out_path: &Path) -> EvalResult {
    // Allow only relative paths
    let file_path = Path::new(file);
    if file_path.is_absolute() {
        return Err(EvalError::PathNotRelative(file.to_string()));
    }

    let out_file_path = out_path.join(&file_path);
    let file_path = src_path.join(&file_path);

    if !file_path.is_file() {
        return Err(EvalError::NotFile(file.to_string()));
    }

    // if extension is "foil" then build the file
    // and change the extension to html.
    if Some(OsStr::new("foil")) == file_path.extension() {
        let out_file_path = out_file_path.with_extension("html");
        evaluate_file(&file_path, &out_file_path)
            .and_then(Output::to_content)
            .and_then(|text| { 
                write_to_file(&text, &out_file_path) 
            })
    } else {
        copy_file(&file_path, &out_file_path)
    }
}
