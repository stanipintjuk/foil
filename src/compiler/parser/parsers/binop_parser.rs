use compiler::models::{Ast, BinOp};
use compiler::parser::{ParseResult, Parser, ParseError};

pub fn parse_binop(parser: &mut Parser, op: BinOp, pos: usize) -> Option<ParseResult> {
    // Get the left expression
    let left = expect_expression!(parser, pos);

    // Get the right expression
    let right = expect_expression!(parser, pos);

    Some(Ok(Ast::BinOp(op, 
                       Box::new(left), 
                       Box::new(right))))
}

