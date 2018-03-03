use helpers::all_ok;

use compiler::tokenizer::tokens::{Token, BinOp, Val, Keyword};
use compiler::tokenizer::{TokenIterator, TokenResult};

use super::ast::{Ast, Id, SetField};
use super::error::ParseError;

pub type ParseResult = Result<Ast, ParseError>;

pub struct Parser<'i> {
    token_iter: &'i mut TokenIterator<'i>,
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

    fn parse_keyword(&mut self, keyword: Keyword, pos: usize) -> Option<ParseResult> {
        match keyword {
            Keyword::Let => self.parse_let(pos),
            Keyword::Fn => self.parse_fn(pos),
            Keyword::Import => self.parse_import(pos),
            Keyword::Set => self.parse_set(pos),
            Keyword::In => Some(Err(ParseError::UnexpectedKeyword(Keyword::In))),
            Keyword::Html => self.parse_html(pos),
        }
    }

    fn parse_html(&mut self, pos: usize) -> Option<ParseResult> {
        let (pos, id_name) = expect_id!(self.token_iter, pos);
        self.parse_html_rest(pos, id_name)
    }

    fn parse_html_rest(&mut self, pos: usize, tag_name: String) -> Option<ParseResult> {
        let mut attributes = vec![];
        let mut children = vec![];

        // Parse attributes sometime in the future

        let token = next_token!(self.token_iter, pos);
        match token {
            Token::Semi(_) => {
                return all_ok(Ast::HtmlClosed{tag_name:tag_name, attributes: attributes});
            },
            Token::BlockL(pos) => {
                match self.parse_html_children(pos) {
                    Ok(ch) => { children = ch; },
                    Err(err) => { return Some(Err(err)); }
                }
            },
            token => {
                let child = self.parse_html_child(token);
            }
        }

        all_ok(Ast::Html{
            tag_name: tag_name,
            attributes: attributes,
            children: children,
        })
    }

    fn parse_html_children(&mut self, pos: usize) -> Result<Vec<Ast>, ParseError> {
        unimplemented!()
    }

    fn parse_html_child(&mut self, token: Token) -> Option<ParseResult> {
        match token {
            Token::GroupL(pos) => {
                let expr = self.next();
                expect_group_r!(self.token_iter, pos);
                expr
            }
            Token::Val(_, val) => all_ok(Ast::Val(val)),
            Token::Id(pos, tag_name) => self.parse_html_rest(pos, tag_name),
            other => Some(Err(ParseError::Unexpected(other))),
        }
    }

    fn parse_let(&mut self, pos: usize) -> Option<ParseResult> {
        let (pos, id_name) = expect_id!(self.token_iter, pos);
        let pos = expect_assignment!(self.token_iter, pos);
        let value = expect_expression!(self, pos);
        let pos = expect_keyword!(Keyword::In, self.token_iter, pos);
        let expr = expect_expression!(self, pos);
        all_ok(
            Ast::Let(
                Box::new(SetField {
                    name: id_name,
                    value: value
                }),
                Box::new(expr)))

    }

    fn parse_fn(&mut self, pos: usize) -> Option<ParseResult> {
        let (pos, arg_name) = expect_id!(self.token_iter, pos);
        let token = next_token!(self.token_iter, pos);

        let pos = match token {
            Token::Colon(pos) => pos,
            token => {
                return Some(Err(ParseError::ExpectedColon(token)));
            }
        };

        let expr = expect_expression!(self, pos);
        
        all_ok(Ast::Fn(arg_name, Box::new(expr)))
    }

    fn parse_import(&mut self, pos: usize) -> Option<ParseResult> {
        let token = next_token!(self.token_iter, pos);
        match token {
            Token::Val(pos, Val::String(path)) => {
                Some(Ok(Ast::Import(pos, path)))
            },
            token => {
                Some(Err(ParseError::ExpectedString(token)))
            }
        }
    }

    fn parse_set(&mut self, pos: usize) -> Option<ParseResult> {
        // Get the token
        let token = next_token!(self.token_iter, pos);

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
            let result = self.parse_set_field(pos);
            if let Some(Ok(field)) = result {
                set_fields.push(field);

            } else if let Some(Err(err)) = result {
                return Some(Err(err));

            } else {
                break;
            };

            // Get the token
            let token = next_token!(self.token_iter, pos);

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

    fn parse_set_field(&mut self, pos: usize) -> Option<Result<SetField, ParseError>> {

        // Expect an id token
        let (pos, field_name) = expect_id!(self.token_iter, pos);

        // Expect next token to be '='
        let pos = expect_assignment!(self.token_iter, pos);

        // And let the value be any kind of expression
        let value = expect_expression!(self, pos);

        return Some(Ok(SetField { name: field_name, value: value }));
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
            Ok(Token::Keyword(pos, keyword)) => self.parse_keyword(keyword, pos),
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

