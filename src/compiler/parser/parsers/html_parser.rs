use helpers::all_ok;
use compiler::models::{Ast, SetField, Token};
use compiler::parser::{ParseError, ParseResult, Parser};

pub fn parse_html(parser: &mut Parser, pos: usize) -> Option<ParseResult> {

    let (pos, id) = expect_id!(parser.token_iter, pos);
    parse_html_tag_contents(parser, id,  pos)
}

#[derive(Debug)]
enum HtmlAttributeParserResult {
    Success(SetField),
    NotAttribute{id: String, next_token: Token},
}


fn parse_html_tag_contents(parser: &mut Parser, id: String, pos: usize) -> Option<ParseResult> {
    let token = next_token!(parser.token_iter, pos);
    parse_html_tag_contents_rest(parser, id, pos, token)
}

fn parse_html_tag_contents_rest(parser: &mut Parser, id: String, pos: usize, token: Token) -> Option<ParseResult> {
    let mut attributes = vec![];

    let mut token = token;
    // If the next token is an Id then the next statement can either be an attribute or a
    // single HTML child.
    while let Token::Id(pos, attr_id) = token {
        // Try parsing the attribute
        let res = parse_html_attribute(parser, attr_id, pos);
        match res {
            None => {
                return Some(Err(ParseError::UnexpectedEndOfCode(pos)));
            },
            Some(Err(err)) => {
                return Some(Err(err))
            },
            // If it turns to not be an attribute then it can only be a single HTML child, so we
            // exit the loop and return it.
            Some(Ok(HtmlAttributeParserResult::NotAttribute{id, next_token})) => {
                let res = parse_html_tag_contents_rest(parser, id.clone(), pos, next_token);
                match res {
                        Some(Ok(child)) => {
                            return all_ok(Ast::Html{
                                tag_name: id,
                                attributes: attributes,
                                children: vec![child],
                            });
                        },
                        Some(Err(err)) => {
                            return Some(Err(err));
                        },
                        None => {
                            return Some(Err(ParseError::UnexpectedEndOfCode(pos)));
                        }
                }
            },
            // If it in fact is an attribute then add it to the attributes vector and continute
            // looking for the next attribute.
            Some(Ok(HtmlAttributeParserResult::Success(attribute))) => {
                attributes.push(attribute);
            },
        }
        
        // get the next token and loop again.
        token = next_token!(parser.token_iter, pos);
    }

    // If a token is a semi-colon then it must be a self-closing tag
    if let Token::Semi(_) = token {
        return all_ok(Ast::HtmlClosed{
            tag_name: id,
            attributes: attributes,
        });
    }

    // Parse the children
    let res = parse_html_tag_children(parser, pos, token);
    match res {
        Some(Ok(children)) => 
            all_ok(Ast::Html{
                tag_name: id,
                attributes: attributes,
                children: children,
            }),
        Some(Err(err)) => Some(Err(err)),
        None => Some(Err(ParseError::UnexpectedEndOfCode(pos))),
    }

}

fn parse_html_tag_children(parser: &mut Parser, pos: usize, token: Token) -> Option<Result<Vec<Ast>, ParseError>> {
    let mut children = vec![];
    match token {
        Token::BlockL(pos) => {
            loop {
                let token = next_token!(parser.token_iter, pos);
                if let Token::BlockR(_) = token {
                    return all_ok(children);
                }
                match parse_html_child(parser, pos, token) {
                    Some(Ok(child)) => { children.push(child); },
                    Some(Err(err)) => { return Some(Err(err)) },
                    None => { return Some(Err(ParseError::UnexpectedEndOfCode(pos)))},
                }
            }
        },
        other_token => {
            match parse_html_child(parser, pos, other_token) {
                Some(Ok(child)) => all_ok(vec![child]),
                Some(Err(err)) => Some(Err(err)),
                None => { return Some(Err(ParseError::UnexpectedEndOfCode(pos)))},
            }
        }
    }
}

fn parse_html_child(parser: &mut Parser, _pos: usize, token: Token) -> Option<Result<Ast, ParseError>> {
    match token {
        Token::Val(_, val) => all_ok(Ast::Val(val)),
        Token::GroupL(pos) => {
            let expr = expect_expression!(parser, pos);
            expect_group_r!(parser.token_iter, pos);
            all_ok(expr)
        }
        Token::Id(pos, tag_name) => parse_html_tag_contents(parser, tag_name, pos),
        other_token => Some(Err(ParseError::Unexpected(other_token))),
    }
}

fn parse_html_attribute(parser: &mut Parser, tag_name: String, pos: usize) -> Option<Result<HtmlAttributeParserResult, ParseError>> {
    let token = next_token!(parser.token_iter, pos);
    match token {
        Token::Assign(pos) => {
            let expr = expect_expression!(parser, pos);
            return all_ok(HtmlAttributeParserResult::Success(SetField{name: tag_name, value: expr}));
        },
        other_token => {
            return all_ok(HtmlAttributeParserResult::NotAttribute{id: tag_name, next_token: other_token});
        }
    }
}
