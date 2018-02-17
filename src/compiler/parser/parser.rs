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

type TokenIterator<'i, 's: 'i> = Iterator<Item=Result<Token<'s>, LexError<'s>>> + 'i;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ParseError<'s> {
    Unexpected(Token<'s>),
    ExpectedExpression(usize),
    UnexpectedKeyword(usize, Keyword),
    Lexer(LexError<'s>),
    ExpectedGroupL(Token<'s>),
    ExpectedSetField(usize),
    ExpectedSetFieldName(Token<'s>),
    UnexpectedEndOfCode(usize),
    ExpectedAssignment(Token<'s>),
    ExpectedComma(Token<'s>),
}

type ParseResult<'s> = Result<Ast<'s>, ParseError<'s>>;
struct Parser<'i, 's: 'i> {
    token_iter: &'i mut TokenIterator<'i, 's>,
}
impl<'i, 's: 'i> Parser<'i, 's> {

    fn new(token_iter: &'i mut TokenIterator<'i, 's>) -> Self {
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
        }
    }

    fn parse_let(&mut self, pos: usize) -> Option<ParseResult<'s>> {
        unimplemented!()
    }

    fn parse_fn(&mut self, pos: usize) -> Option<ParseResult<'s>> {
        unimplemented!()
    }

    fn parse_import(&mut self, pos: usize) -> Option<ParseResult<'s>> {
        unimplemented!()
    }

    fn parse_set(&mut self, pos: usize) -> Option<ParseResult<'s>> {
        // Get the token
        let token = match self.token_iter.next() {
            Some(Ok(token)) => token,
            Some(Err(err)) => {
                return Some(Err(ParseError::Lexer(err)))
            },
            None => {
                return Some(Err(ParseError::UnexpectedEndOfCode(pos)))
            },
        };

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
            let token = match self.token_iter.next() {
                Some(Ok(token)) => token,
                Some(Err(err)) => {
                    return Some(Err(ParseError::Lexer(err)))
                },
                None => {
                    return Some(Err(ParseError::UnexpectedEndOfCode(pos)))
                },
            };

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
        // Get the token
        let token = match self.token_iter.next() {
            Some(Ok(token)) => token,
            Some(Err(err)) => {
                return Some(Err(ParseError::Lexer(err)))
            },
            None => {
                return Some(Err(ParseError::UnexpectedEndOfCode(pos)))
            },
        };

        // Expect the token to be an Id,
        // or return None if '}' is encountered.
        let (pos, field_name) = match token {
            Token::Id(pos, name) => {
                (pos, name)
            },
            Token::BlockR(_) => {
                return None;
            },
            token => {
                return Some(
                    Err(ParseError::ExpectedSetFieldName(token)));
            }
        };
        

        let token = match self.token_iter.next() {
            Some(Ok(token)) => token,
            Some(Err(err)) => {
                return Some(Err(ParseError::Lexer(err)))
            },
            None => {
                return Some(Err(ParseError::UnexpectedEndOfCode(pos)))
            },
        };

        // Expect next token to be '='
        let pos =  match token {
            Token::BinOp(pos, BinOp::Assign) => { pos },
            token => {
                return Some(Err(ParseError::ExpectedAssignment(token)));
            }
        };

        let expr = expect_expression!(self, pos);

        return Some(Ok(SetField { name: field_name, value: expr }));
    }

    fn parse_token(&mut self, token: LexResult<'s>) -> Option<ParseResult<'s>> {
        match token {
            Ok(Token::BinOp(pos, op)) => self.parse_bin_op(op, pos),
            Ok(Token::Val(_, val)) => Some(Ok(Ast::Val(val))),
            Ok(Token::Keyword(pos, keyword)) => self.parse_keyword(keyword, pos),
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

#[cfg(test)]
mod tests {
    use super::*;
    use compiler::tokens::*;
    use super::super::ast::*;

    #[test]
    fn parse_binary_op_test() {
        let input = vec![
            Ok(Token::BinOp(0, BinOp::Add)),
            Ok(Token::Val(0, Val::Int(3))),
            Ok(Token::Val(0, Val::Int(4))),
        ];

        let expected = vec![
            Ok(Ast::BinOp(
                    BinOp::Add, 
                    Box::new(Ast::Val(Val::Int(3))),
                    Box::new(Ast::Val(Val::Int(4)))
                    ))
        ];

        let mut iter = input.iter().map(Clone::clone);
        let actual: Vec<_> = Parser::new(&mut iter).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_nested_binary_op_test() {
        // "+ - 1 2 3"
        let input = vec![
            Ok(Token::BinOp(0, BinOp::Add)),
            Ok(Token::BinOp(0, BinOp::Sub)),
            Ok(Token::Val(0, Val::Int(1))),
            Ok(Token::Val(0, Val::Int(2))),
            Ok(Token::Val(0, Val::Int(3))),
        ];

        let expected = vec![
            Ok(Ast::BinOp(
                    BinOp::Add,
                    Box::new(Ast::BinOp(
                            BinOp::Sub,
                            Box::new(Ast::Val(Val::Int(1))),
                            Box::new(Ast::Val(Val::Int(2))))),
                    Box::new(Ast::Val(Val::Int(3)))))
        ];

        let mut iter = input.iter().map(Clone::clone);
        let actual: Vec<_> = Parser::new(&mut iter).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_nested_binary_op_second_order() {
        // + 1 - 2 3
        let input = vec![
            Ok(Token::BinOp(0, BinOp::Add)),
            Ok(Token::Val(0, Val::Int(1))),
            Ok(Token::BinOp(0, BinOp::Sub)),
            Ok(Token::Val(0, Val::Int(2))),
            Ok(Token::Val(0, Val::Int(3))),
        ];

        let expected = vec![
            Ok(Ast::BinOp(
                    BinOp::Add,
                    Box::new(Ast::Val(Val::Int(1))),
                    Box::new(Ast::BinOp(
                            BinOp::Sub,
                            Box::new(Ast::Val(Val::Int(2))),
                            Box::new(Ast::Val(Val::Int(3)))))))
        ];

        let mut iter = input.iter().map(Clone::clone);
        let actual: Vec<_> = Parser::new(&mut iter).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_set_construction() {
        /*
         * set { field1="value", field2=23}
         * */
        
        let input = vec![
            Ok(Token::Keyword(0, Keyword::Set)),
            Ok(Token::BlockL(0)),
            Ok(Token::Id(0, "field1")),
            Ok(Token::BinOp(0, BinOp::Assign)),
            Ok(Token::Val(0, Val::String("value"))),
            Ok(Token::Comma(0)),
            Ok(Token::Id(0, "field2")),
            Ok(Token::BinOp(0, BinOp::Assign)),
            Ok(Token::Val(0, Val::Int(23))),
            Ok(Token::BlockR(0))
        ];

        let expected = vec![
            Ok(Ast::Set( vec![ 
                    SetField { 
                        name: "field1",
                        value: Ast::Val(Val::String("value"))
                    },
                    SetField { 
                        name: "field2",
                        value: Ast::Val(Val::Int(23))
                    },
            ]))
        ];

        let mut iter = input.iter().map(Clone::clone);
        let actual: Vec<_> = Parser::new(&mut iter).collect();
        assert_eq!(expected, actual);

    }
}
