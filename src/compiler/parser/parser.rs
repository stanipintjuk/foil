use helpers::*;
use compiler::tokens::*;
use compiler::lexer::{LexError, LexResult};
use super::ast::*;

macro_rules! expect_expression {
    ( $parser:expr, $pos:expr ) => {
        match $parser.next() {
            Some(Ok(expr)) => expr,
            Some(Err(err)) => {
                return Some(Err(err));
            }
            None => {
                return Some(Err(ParseError::ExpectedExpression($pos)));
            }
        }
    }
}

macro_rules! next_token {
    ( $lexer:expr, $pos:expr ) => {
        match $lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(err)) => {
                return Some(Err(ParseError::Lexer(err)));
            }
            None => {
                return Some(Err(ParseError::UnexpectedEndOfCode($pos)));
            }
        }
    }
}

macro_rules! expect_assignment {
    ( $lexer:expr, $pos:expr ) => {{
        let token = next_token!($lexer, $pos);
        let pos = match token {
            Token::BinOp(pos, BinOp::Assign) => { pos },
            token => {
                return Some(Err(ParseError::ExpectedAssignment(token)));
            }
        };
        pos
    }}
}

macro_rules! expect_keyword {
    ($keyword:expr, $lexer:expr, $pos:expr) => {{
        let token = next_token!($lexer, $pos);
        let pos = match token {
            Token::Keyword(pos, keyword) => { 
                if keyword == $keyword {
                    pos 
                } else {
                    return Some(Err(
                            ParseError::ExpectedKeyword($keyword, 
                                                        Token::Keyword(pos, keyword))));
                }
            },
            token => {
                return Some(Err(ParseError::ExpectedKeyword($keyword, token)));
            }
        };
        pos
    }}
}

macro_rules! expect_id {
    ($lexer:expr, $pos:expr) => {{
        let token = next_token!($lexer, $pos);
        match token {
            Token::Id(pos, name) => (pos, name),
            token => { 
                return Some(Err(ParseError::ExpectedId(token)));
            }
        }
    }}
}

macro_rules! expect_group_r {
    ($lexer:expr, $pos:expr) => {{
        let token = next_token!($lexer, $pos);
        match token {
            Token::GroupR(pos) => pos,
            token => { 
                return Some(Err(ParseError::ExpectedGroupR(token)));
            }
        }
    }}
}

type TokenIterator<'i, 's: 'i> = Iterator<Item=Result<Token<'s>, LexError<'s>>> + 'i;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ParseError<'s> {
    Unexpected(Token<'s>),
    ExpectedExpression(usize),
    Lexer(LexError<'s>),
    ExpectedGroupL(Token<'s>),
    ExpectedId(Token<'s>),
    UnexpectedEndOfCode(usize),
    ExpectedAssignment(Token<'s>),
    ExpectedComma(Token<'s>),
    UnexpectedKeyword(Keyword),
    ExpectedKeyword(Keyword, Token<'s>),
    ExpectedPath(Token<'s>),
    ExpectedColon(Token<'s>),
    ExpectedGroupR(Token<'s>),
}

type ParseResult<'s> = Result<Ast<'s>, ParseError<'s>>;
pub struct Parser<'i, 's: 'i> {
    token_iter: &'i mut TokenIterator<'i, 's>,
}
impl<'i, 's: 'i> Parser<'i, 's> {

    pub fn new(token_iter: &'i mut TokenIterator<'i, 's>) -> Self {
        Parser{token_iter: token_iter}
    }

    fn parse_bin_op(&mut self, op: BinOp, pos: usize) -> Option<ParseResult<'s>> {
        // Get the left expression
        let left = expect_expression!(self, pos);

        // Get the right expression
        let right = expect_expression!(self, pos);

        Some(Ok(Ast::BinOp(op, 
                           Box::new(left), 
                           Box::new(right))))
    }

    fn parse_keyword(&mut self, keyword: Keyword, pos: usize) -> Option<ParseResult<'s>> {
        match keyword {
            Keyword::Let => self.parse_let(pos),
            Keyword::Fn => self.parse_fn(pos),
            Keyword::Import => self.parse_import(pos),
            Keyword::Set => self.parse_set(pos),
            Keyword::In => Some(Err(ParseError::UnexpectedKeyword(Keyword::In))),
        }
    }

    fn parse_let(&mut self, pos: usize) -> Option<ParseResult<'s>> {
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

    fn parse_fn(&mut self, pos: usize) -> Option<ParseResult<'s>> {
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

    fn parse_import(&mut self, pos: usize) -> Option<ParseResult<'s>> {
        let token = next_token!(self.token_iter, pos);
        match token {
            Token::Val(pos, Val::Path(path)) => {
                Some(Ok(Ast::Import(pos, path)))
            },
            token => {
                Some(Err(ParseError::ExpectedPath(token)))
            }
        }
    }

    fn parse_set(&mut self, pos: usize) -> Option<ParseResult<'s>> {
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

        let mut set_fields: Vec<SetField<'s>> = Vec::new();

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

    fn parse_set_field(&mut self, pos: usize) -> Option<Result<SetField<'s>, ParseError<'s>>> {

        // Expect an id token
        let (pos, field_name) = expect_id!(self.token_iter, pos);

        // Expect next token to be '='
        let pos = expect_assignment!(self.token_iter, pos);

        // And let the value be any kind of expression
        let value = expect_expression!(self, pos);

        return Some(Ok(SetField { name: field_name, value: value }));
    }

    fn parse_fn_call(&mut self, pos: usize) -> Option<ParseResult<'s>> {
        let (id_pos, id_name) = expect_id!(self.token_iter, pos);
        let param = expect_expression!(self, id_pos);
        expect_group_r!(self.token_iter, pos);
        all_ok(Ast::Call(
                Id(id_pos, id_name),
                Box::new(param)))
    }

    fn parse_token(&mut self, token: LexResult<'s>) -> Option<ParseResult<'s>> {
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
impl<'i, 's: 'i> Iterator for Parser<'i, 's> {
    type Item = Result<Ast<'s>, ParseError<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.token_iter.next() {
            self.parse_token(token)
        } else {
            None
        }
    }
}

