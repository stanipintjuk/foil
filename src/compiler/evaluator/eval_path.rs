use compiler::evaluator::error::EvalError;
use compiler::evaluator::{EvalResult, Output};
use compiler::{evaluate_file, write_to_file, copy_file};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/// # Arguments
/// `file` - relative path to the file to be processesed.
/// `src_path` - current working directory.
/// `out_path` - the directory to  which the processed file would be copied.
/// 
/// # Errors
/// `EvalError::NotFile(String)` will be returned if `file` is not a file or doesn't exist
/// relative to `src_path`.
///
/// `EvalError::OutputPathNotSpecified` if `out_path` is None.
/// error variant would be returned.
/// 
pub fn process_path(file: &str, src_path: &Path, out_path: &Option<&Path>) -> EvalResult {
    if out_path == &None {
        return Err(EvalError::OutputPathNotSpecified)
    }
    let out_path = out_path.unwrap();
    build_path(file, src_path, out_path)
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
        let full_path = src_path.join(file);
        let full_path = full_path.to_str().unwrap();
        return Err(EvalError::NotFile(full_path.to_string()));
    }

    // if extension is "foil" then build the file
    // and change the extension to html.
    if Some(OsStr::new("foil")) == file_path.extension() {
        let out_file_path = out_file_path.with_extension("html");
        evaluate_file(&file_path, &out_file_path)
            .and_then(Output::to_string)
            .and_then(|text| { 
                write_to_file(&text, &out_file_path) 
            })
    } else {
        copy_file(&file_path, &out_file_path)
    }
}
