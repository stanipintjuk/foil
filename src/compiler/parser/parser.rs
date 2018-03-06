use helpers::all_ok;

use compiler::tokenizer::tokens::{Token, BinOp, Val, Keyword};
use compiler::tokenizer::{TokenIterator, TokenResult};

use super::ast::{Ast, Id, SetField};
use super::error::ParseError;
use super::parsers::{
    parse_html,
    parse_keyword,
};

pub type ParseResult = Result<Ast, ParseError>;

pub struct Parser<'i> {
    pub token_iter: &'i mut TokenIterator<'i>,
}
impl<'i> Parser<'i> {

    pub fn new(token_iter: &'i mut TokenIterator<'i>) -> Self {
        Parser{token_iter: token_iter}
    }

    fn parse_bin_op(&mut self, op: BinOp, pos: usize) -> Option<ParseResult> {
        // Get the left expression
        let left = expect_expression!(self, pos);

        // Get the right expression
        let right = expect_expression!(self, pos);

        Some(Ok(Ast::BinOp(op, 
                           Box::new(left), 
                           Box::new(right))))
    }


    fn parse_fn_call(&mut self, pos: usize) -> Option<ParseResult> {
        let func = expect_expression!(self, pos);
        let param = expect_expression!(self, pos);
        expect_group_r!(self.token_iter, pos);
        all_ok(Ast::Call(
                Box::new(func),
                Box::new(param)))
    }

    fn parse_token(&mut self, token: TokenResult) -> Option<ParseResult> {
        match token {
            Ok(Token::BinOp(pos, op)) => self.parse_bin_op(op, pos),
            Ok(Token::Val(_, val)) => all_ok(Ast::Val(val)),
            Ok(Token::Keyword(pos, keyword)) => parse_keyword(self, keyword, pos),
            Ok(Token::Id(pos, name)) => all_ok(Ast::Id(Id(pos, name))),
            Ok(Token::GroupL(pos)) => self.parse_fn_call(pos),
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

