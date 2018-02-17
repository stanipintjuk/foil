use compiler::tokens::*;
use compiler::lexer::{LexError};
use super::ast::*;

type TokenIterator<'i, 's: 'i> = Iterator<Item=Result<Token<'s>, LexError<'s>>> + 'i;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ParseError<'s> {
    Unexpected(Token<'s>),
    ExpectedExpression,
    Lexer(LexError<'s>),
}

struct Parser<'i, 's: 'i> {
    token_iter: &'i mut TokenIterator<'i, 's>,
}
impl<'i, 's: 'i> Parser<'i, 's> {
    fn new(token_iter: &'i mut TokenIterator<'i, 's>) -> Self {
        Parser{token_iter: token_iter}
    }

    fn parse_bin_op(&mut self, op: BinOp) 
        -> Option<Result<Ast<'s>, ParseError<'s>>> {
            // Get the left expression
            let left = match self.next() {
                Some(Ok(expr)) => expr,
                Some(Err(err)) => {
                    return Some(Err(err));
                },
                None => {
                    return Some(Err(ParseError::ExpectedExpression));
                }
            };

            // Get the right expression
            let right = match self.next() {
                Some(Ok(expr)) => expr,
                Some(Err(err)) => {
                    return Some(Err(err));
                },
                None => {
                    return Some(Err(ParseError::ExpectedExpression));
                }
            };

            Some(Ok(Ast::BinOp(op, 
                               Box::new(left), 
                               Box::new(right))))
        }
}
impl<'i, 's: 'i> Iterator for Parser<'i, 's> {
    type Item = Result<Ast<'s>, ParseError<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.token_iter.next() {
            match token {
                Ok(Token::BinOp(_, op)) => self.parse_bin_op(op),
                Ok(Token::Val(_, val)) => Some(Ok(Ast::Val(val))),
                Ok(t) => Some(Err(ParseError::Unexpected(t))),
                Err(err) => Some(Err(ParseError::Lexer(err))),
            }
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

        let iter = input.iter();
        let mut iter_map = iter.map(Clone::clone);
        let actual: Vec<_> = Parser::new(&mut iter_map).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_nested_binary_op_test() {
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
}
