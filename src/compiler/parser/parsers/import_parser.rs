use helpers::all_ok;
use compiler::tokenizer::tokens::{Token, Keyword, Val};
use compiler::parser::ast::{Ast, SetField};
use compiler::parser::{ParseError, ParseResult, Parser};

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
