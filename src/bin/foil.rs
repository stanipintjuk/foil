extern crate foil;
use foil::grammar::html;
use std::io::{self, Read};
use foil::validate::validate_paths;
use foil::interpret::into_html;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_to_string(&mut buffer);

    let result = html::node(&buffer);
    match result {
        Ok(html_tree) => {
            match validate_paths(&html_tree) {
                Ok(html_tree) => println!("{}", into_html(&html_tree)),
                Err(paths) => print_invalid_paths(&paths),
            }
        },
        Err(err) => print_html_parse_error(&err),
    }
}

fn print_invalid_paths<'a>(paths: &Vec<(&'a str, &'a usize)>) {
    eprintln!("Found invalid paths:");
    for &(path, position) in paths {
        eprintln!("`{}` on position {}", path, position)
    }
}

fn print_html_parse_error(err: &html::ParseError) {
    eprint!("Error on line {}. Expected one of {:?} on position {}", 
            err.line,
            err.expected,
            err.column) 
}
