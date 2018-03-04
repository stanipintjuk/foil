extern crate foil;
extern crate fs_extra;
use foil::builder::{build_dir, BuildError};
use foil::grammar::ParseError;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{Error as IOError};
use fs_extra::error::Error as FsError;

fn main() {
    let param = get_parameter();
    if None == param {
        print_usage();
        return;
    }
    let param = param.unwrap();

    let path = to_src_root_path(&param);
    if path == None {
        eprint!("`{}` is not a directory", param);
        return;
    }
    let src_root = path.unwrap();

    let out_root = get_out_path();

    let result = build_dir(src_root, out_root);
    match result {
        Ok(()) => print_build_success(),
        Err(err) => print_build_error(&err),
    }

}

fn print_build_success() {
    println!("Build successfull!");
}

fn print_build_error(err: &BuildError) {
    match err {
        &BuildError::IO(ref err) => print_io_error(err),
        &BuildError::FsError(ref err) => print_fs_error(err),
        &BuildError::Parser(ref err) => print_html_parse_error(err),
        &BuildError::InvalidPaths(ref paths) => print_invalid_paths(paths)
    }
}

fn print_io_error(err: &IOError) {
    eprintln!("IOError: {}", err);
}

fn print_fs_error(err: &FsError) {
    eprintln!("FsError: {}", err);
}

fn print_usage() {
    eprintln!("Usage: foil [path/to/source/dir]");
}

fn get_out_path() -> PathBuf {
    env::current_dir().unwrap().join(Path::new("out"))
}

fn to_src_root_path(s: &str) -> Option<PathBuf> {
    let mut p = Path::new(s).to_owned();
    if p.is_relative() {
        p = env::current_dir().unwrap().join(p.clone())
    }

    if !p.exists() || !p.is_dir() {
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

fn print_html_parse_error(err: &ParseError) {
    eprint!("Error on line {}. Expected one of {:?} on position {}", 
            err.line,
            err.expected,
            err.column) 
}
