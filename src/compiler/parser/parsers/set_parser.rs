use helpers::all_ok;
use compiler::tokenizer::tokens::{Token, Keyword};
use compiler::parser::ast::{Ast, SetField};
use compiler::parser::{ParseError, ParseResult, Parser};

pub fn parse_set(parser: &mut Parser, pos: usize) -> Option<ParseResult> {
    // Get the token
    let token = next_token!(parser.token_iter, pos);

    // All sets need to start with '{'
    // So expect GroupL
    match token {
        Token::BlockL(_) => { },
        token => { 
            return Some(Err(ParseError::ExpectedGroupL(token)));
        }
    };

    let mut set_fields: Vec<SetField> = Vec::new();


    // Now find all the set fields
    loop {
        let result = parse_set_field(parser, pos);
        if let Some(Ok(field)) = result {
            set_fields.push(field);

        } else if let Some(Err(err)) = result {
            return Some(Err(err));

        } else {
            break;
        };

        // Get the token
        let token = next_token!(parser.token_iter, pos);

        // Expect comma between set fields
        // Or if BlockR is found then stop looking for fields
        match token {
            Token::Comma(_) => { },
            Token::BlockR(_) => { break; },
            token => { 
                return Some(Err(ParseError::ExpectedComma(token)));
            }
        };

    };

    Some(Ok(Ast::Set(set_fields)))
}

fn parse_set_field(parser: &mut Parser, pos: usize) -> Option<Result<SetField, ParseError>> {

    // Expect an id token
    let (pos, field_name) = expect_id!(parser.token_iter, pos);

    // Expect next token to be '='
    let pos = expect_assignment!(parser.token_iter, pos);

    // And let the value be any kind of expression
    let value = expect_expression!(parser, pos);

    return Some(Ok(SetField { name: field_name, value: value }));
}
