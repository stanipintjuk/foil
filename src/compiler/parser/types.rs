use compiler::tokens::{Token, Keyword};
use compiler::lexer::{LexError};
use super::ast::Ast;

pub type TokenIterator<'i, 's: 'i> = Iterator<Item=Result<Token<'s>, LexError<'s>>> + 'i;

pub type ParseResult<'s> = Result<Ast<'s>, ParseError<'s>>;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ParseError<'s> {
    Unexpected(Token<'s>),
    ExpectedExpression(usize),
    Lexer(LexError<'s>),
    ExpectedGroupL(Token<'s>),
    ExpectedId(Token<'s>),
    UnexpectedEndOfCode(usize),
    ExpectedAssignment(Token<'s>),
    ExpectedComma(Token<'s>),
    UnexpectedKeyword(Keyword),
    ExpectedKeyword(Keyword, Token<'s>),
    ExpectedPath(Token<'s>),
    ExpectedColon(Token<'s>),
    ExpectedGroupR(Token<'s>),
}


