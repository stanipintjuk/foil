#[macro_use] mod macros;
mod parser;
pub use self::parser::{Parser, ParseResult};
mod html_parser;
pub mod ast;
mod error;
pub use self::error::{ParseError};

#[cfg(test)] mod tests;
