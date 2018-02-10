extern crate foil;
use foil::grammar::html;
use foil::interpret::into_html;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_to_string(&mut buffer);

    let result = html::node(&buffer);
    match result {
        Ok(node) => println!("{}", into_html(&node)),
        Err(error) => print_parse_error(error),
    }
}

fn print_parse_error(err: html::ParseError) {
    eprint!("Error on line {}. Expected one of {:?} on position {}", 
            err.line,
            err.expected,
            err.column) 
}
