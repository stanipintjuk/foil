extern crate foil;
extern crate fs_extra;
extern crate tempdir;
use tempdir::TempDir;
use foil::builder::{build_dir, BuildError};
use foil::grammar::ParseError;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{Error as IOError};
use fs_extra::error::Error as FsError;
use fs_extra::dir;
use std::fs::{create_dir_all};
use foil::compiler::build_file;
use foil::compiler::evaluator::EvalError;

fn main() {
    let param = get_parameter();
    if None == param {
        print_usage();
        return;
    }
    let param = param.unwrap();

    let path = to_src_root_path(&param);
    if path == None {
        eprint!("`{}` is not a file", param);
        return;
    }
    let index_file = path.unwrap();
    let out_root = get_out_path();
    let tmp_out_dir = TempDir::new("out").unwrap();
    let tmp_out_dir = tmp_out_dir.path();

    let result = build_file(&index_file, &tmp_out_dir);
    match result {
        Ok(()) => {
            println!("Copying to output path...");
            create_dir_all(&out_root);
            let opts = dir::CopyOptions { 
                overwrite: true, 
                buffer_size: 64000,
                skip_exist: false,
                copy_inside: true,
                depth: 0
            };
            dir::copy(tmp_out_dir, out_root, &opts).unwrap();
            print_build_success();
        },
        Err(err) => println!("{:?}", err),
    }

}

fn print_build_success() {
    println!("Build successfull!");
}

fn print_usage() {
    eprintln!("Usage: foil [path/to/index.foil]");
}

fn get_out_path() -> PathBuf {
    env::current_dir().unwrap().join(Path::new("./"))
}

fn to_src_root_path(s: &str) -> Option<PathBuf> {
    let mut p = Path::new(s).to_owned();
    if p.is_relative() {
        p = env::current_dir().unwrap().join(p.clone())
    }

    if !p.exists() || !p.is_file() {
        None
    } else {
        Some(p)
    }
}

fn get_parameter() -> Option<String> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        None
    } else {
        Some(args[1].to_string())
    }
}

fn print_invalid_paths(paths: &Vec<(String, usize)>) {
    eprintln!("Found invalid paths:");
    for &(ref path, ref position) in paths {
        eprintln!("`{}` on position {}", path, position)
    }
}

fn print_html_parse_error(err: &EvalError) {
    eprint!("Error: {:?}", err);
}
