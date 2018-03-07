use compiler::parser::{ParseResult, Parser, ParseError};
use compiler::parser::ast::Ast;
use compiler::tokenizer::tokens::Token;

pub fn parse_call(parser: &mut Parser, pos: usize) -> Option<ParseResult> {
    let func = expect_expression!(parser, pos);
    let param = expect_expression!(parser, pos);
    expect_group_r!(parser.token_iter, pos);
    Some(Ok(Ast::Call(
            Box::new(func),
            Box::new(param))))
}
