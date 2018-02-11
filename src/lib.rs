#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[cfg(test)]
extern crate tempdir;

extern crate htmlescape;
pub mod grammar;
pub mod interpret;
pub mod validate;
pub mod parser;
