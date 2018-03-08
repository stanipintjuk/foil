#[macro_use] mod macros;
mod parser;
pub use self::parser::{Parser, ParseResult};

mod parsers;

#[cfg(test)] mod tests;
