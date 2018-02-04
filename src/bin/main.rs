extern crate foil;
extern crate nom;
use nom::{IResult, ErrorKind};
use nom::simple_errors::Err;
use std::error::Error;
use std::fmt;

fn main() {
    /*let input = b"-2015";
    let result = foil::positive_year(input);
    println!("input: {:?}", input);
    println!("result: {:?}", result);

    let input = b"2015";
    let result = foil::take_4_digits(input);
    println!("input: {:?}", input);
    println!("result: {:?}", result);*/

    let input = b"{ \"hej\" \"hejda\" }";
    let result = foil::take_delimited_children(input);

    match result {
        IResult::Done(_, node) => {
            println!("{:?}", node);
        },
        IResult::Error(ErrorKind::Custom(num)) => {
            println!("Custom error {}", num)
        },
        IResult::Error(err) => {
            print_recursive(&err);
        },
        IResult::Incomplete(needed) => {
            println!("Incomplete. Need {:?}", needed)
        }
    };
}

fn print_recursive(err: &Error) {
    println!("err: {}", err);
    if let Some(errchild) = err.cause() {
        print_recursive(errchild);
    }
}
