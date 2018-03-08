use helpers::all_ok;
use compiler::models::{Ast, SetField, Token, Val, Keyword};
use compiler::parser::{ParseResult, Parser};
use compiler::errors::ParseError;

pub fn parse_import(parser: &mut Parser, pos: usize) -> Option<ParseResult> {
    let token = next_token!(parser.token_iter, pos);
    match token {
        Token::Val(pos, Val::String(path)) => {
            Some(Ok(Ast::Import(pos, path)))
        },
        token => {
            Some(Err(ParseError::ExpectedString(token)))
        }
    }
}
