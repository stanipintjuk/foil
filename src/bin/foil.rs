extern crate foil;
extern crate nom;
use nom::{IResult, ErrorKind};
use std::io::{self, Read};
use foil::parse_DOM_tree;
use std::error::Error;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer);

    let result = parse_DOM_tree(buffer.as_bytes());
    match result {
        IResult::Done(_, node) => {
            println!("{}", node.to_html());
        },
        IResult::Error(err) => {
            print_error(&err);
        },
        IResult::Incomplete(needed) => {
            println!("Incomplete. Need {:?}", needed)
        }
    };
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
        print_recursive_errors(err);
    }
}

fn print_recursive_errors(err: &Error) {
    println!("err: {}", err);
    if let Some(errchild) = err.cause() {
        println!("Caused by:");
        print_recursive_errors(errchild);
    }
}
