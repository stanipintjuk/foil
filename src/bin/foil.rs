extern crate foil;
extern crate nom;
use nom::{IResult, ErrorKind, Err};
use std::io::{self, Read};
use foil::parsers::foil_parser;
use foil::parsers::errors::*;
use std::error::Error;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_to_string(&mut buffer);

    let result = foil_parser(buffer.as_bytes());
    match result {
        Ok(dom_tree) => {
            println!("{}", dom_tree.to_html());
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    };
}
/*
fn print_err(err: Err<u32>) {
    match err {
        Err::Code(err) => print_error(err),
        Err::Node(err, vec) => {
            print_error(err);
            for err in vec {
                print_err(err);
            }
        }
        Err::Position(err, pos) => {
            print_err(err);
            println!("position: {}", pos);
        }
        Err::NodePosition(err, pos, vec) {
            print_err(err);
            for err in vec {
                print_err(err);
            }
            println!("position: {}", pos);
        }
    }
}

fn print_error(err: &ErrorKind) {
    match err {
        &ErrorKind::Custom(ref err_code) => {
            print_custom_errs(*err_code, err);
        },
        err => {
            print_recursive_errors(err);
        },
    };
}

fn print_custom_errs(err_code: u32, err: &Error) {
    print!("err: ");
    match err_code {
        ERROR_INVALID_TAG_NAME => println!("Invalid tagname."),
        ERROR_INVALID_ATTRIBUTE_NAME => println!("Invalid attribute name."),
        ERROR_ATTRIBUTE_VALUE_NOT_STRING => println!("Attribute value is not a string."),
        ERROR_EXPECTED_WHITESPACE => println!("Expected whitespace"),
        ERROR_INVALID_EXPRESSION => println!("Invalid expression"),
        ERROR_DOM_NODE => println!("DOM node is invalid."),
        ERROR_INVALID_SELF_CLOSING_DOM_NODE => println!("Invalid self-closing DOM node."),
        ERROR_EXPECTING_DELIMITERS => println!("Expecting delimiters."),
        num => println!("Unknown {}", num),
    };

    if let Some(cause) = err.cause() {
        println!("Caused by:");
        print_recursive_errors(cause);
    }
}

fn print_recursive_errors(err: &Error) {
    println!("err: {}", err);
    if let Some(errchild) = err.cause() {
        println!("Caused by:");
        print_recursive_errors(errchild);
    }
}
*/
