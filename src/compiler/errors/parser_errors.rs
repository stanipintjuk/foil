use compiler::models::{Token, Keyword};
use compiler::errors::TokenError;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum ParseError {
    Unexpected(Token),
    ExpectedExpression(usize),
    Lexer(TokenError),
    ExpectedGroupL(Token),
    ExpectedId(Token),
    UnexpectedEndOfCode(usize),
    ExpectedAssignment(Token),
    ExpectedComma(Token),
    UnexpectedKeyword(Keyword),
    ExpectedKeyword(Keyword, Token),
    ExpectedPath(Token),
    ExpectedColon(Token),
    ExpectedGroupR(Token),
    ExpectedString(Token),
}


