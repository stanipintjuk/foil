use helpers::all_ok;

use compiler::models::{Ast, Id, Token, BinOp, Val, Keyword};
use compiler::tokenizer::{TokenIterator, TokenResult};

use compiler::errors::ParseError;
use super::parsers::{
    parse_keyword,
    parse_binop,
    parse_call,
};

pub type ParseResult = Result<Ast, ParseError>;

pub struct Parser<'i> {
    pub token_iter: &'i mut TokenIterator<'i>,
}
impl<'i> Parser<'i> {

    pub fn new(token_iter: &'i mut TokenIterator<'i>) -> Self {
        Parser{token_iter: token_iter}
    }

    fn parse_token(&mut self, token: TokenResult) -> Option<ParseResult> {
        match token {
            Ok(Token::Val(_, val)) => all_ok(Ast::Val(val)),
            Ok(Token::Id(pos, name)) => all_ok(Ast::Id(Id(pos, name))),
            Ok(Token::BinOp(pos, op)) => parse_binop(self, op, pos),
            Ok(Token::Keyword(pos, keyword)) => parse_keyword(self, keyword, pos),
            Ok(Token::GroupL(pos)) => parse_call(self, pos),
            Ok(t) => Some(Err(ParseError::Unexpected(t))),
            Err(err) => Some(Err(ParseError::Lexer(err))),
        }
    }
}
impl<'i> Iterator for Parser<'i> {
    type Item = Result<Ast, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.token_iter.next() {
            self.parse_token(token)
        } else {
            None
        }
    }
}

