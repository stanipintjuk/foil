use helpers::all_ok;
use compiler::models::{Ast, SetField, Token, Keyword};
use compiler::parser::{ParseResult, Parser};
use compiler::errors::ParseError;

pub fn parse_fn(parser: &mut Parser, pos: usize) -> Option<ParseResult> {
    let (pos, arg_name) = expect_id!(parser.token_iter, pos);
    let token = next_token!(parser.token_iter, pos);

    let pos = match token {
        Token::Colon(pos) => pos,
        token => {
            return Some(Err(ParseError::ExpectedColon(token)));
        }
    };

    let expr = expect_expression!(parser, pos);

    all_ok(Ast::Fn(arg_name, Box::new(expr)))
}
