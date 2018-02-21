use compiler::tokens::{Token, Keyword};
use compiler::lexer::{LexError};
use super::ast::Ast;

pub type TokenIterator<'i> = Iterator<Item=Result<Token, LexError>> + 'i;

pub type ParseResult = Result<Ast, ParseError>;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum ParseError {
    Unexpected(Token),
    ExpectedExpression(usize),
    Lexer(LexError),
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
}


