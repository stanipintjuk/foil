use helpers::all_ok;
use compiler::models::{Ast, SetField, Token, Keyword};
use compiler::parser::{ParseError, ParseResult, Parser};


pub fn parse_let(parser: &mut Parser, pos: usize) -> Option<ParseResult> {
    let (pos, id_name) = expect_id!(parser.token_iter, pos);
    let pos = expect_assignment!(parser.token_iter, pos);
    let value = expect_expression!(parser, pos);
    let pos = expect_keyword!(Keyword::In, parser.token_iter, pos);
    let expr = expect_expression!(parser, pos);
    all_ok(
        Ast::Let(
            Box::new(SetField {
                name: id_name,
                value: value
            }),
            Box::new(expr)))

}
