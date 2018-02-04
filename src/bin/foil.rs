extern crate foil;
extern crate nom;
use nom::IResult;
use std::io::{self, Read};
use foil::parse_DOM_tree;

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
            println!("Error: {}", err);
        },
        IResult::Incomplete(needed) => {
            println!("Incomplete. Need {:?}", needed)
        }
    };
}
