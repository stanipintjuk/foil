use compiler::models::Keyword;
use compiler::parser::{ParseError, ParseResult, Parser};
use super::{
    parse_let,
    parse_fn,
    parse_import,
    parse_set,
    parse_html,
};

pub fn parse_keyword(parser: &mut Parser, keyword: Keyword, pos: usize) -> Option<ParseResult> {
    match keyword {
        Keyword::Let => parse_let(parser, pos),
        Keyword::Fn => parse_fn(parser, pos),
        Keyword::Import => parse_import(parser, pos),
        Keyword::Set => parse_set(parser, pos),
        Keyword::In => Some(Err(ParseError::UnexpectedKeyword(Keyword::In))),
        Keyword::Html => parse_html(parser, pos),
    }
}
