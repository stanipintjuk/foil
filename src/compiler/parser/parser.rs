use compiler::tokens::*;
use super::ast::*;

type TokenIterator<'i, 's: 'i> = Iterator<Item=Token<'s>> + 'i;

struct Parser<'i, 's: 'i> {
    token_iter: &'i mut TokenIterator<'i, 's>,
}
impl<'i, 's: 'i> Parser<'i, 's> {
    fn new(token_iter: &'i mut TokenIterator<'i, 's>) -> Self {
        Parser{token_iter: token_iter}
    }

    fn parse_bin_op(&mut self, op: BinOp) -> Option<Ast<'s>> {
        let left = self.next();
        let right = self.next();
        if let Some(left) = left {
            if let Some(right) = right {
                Some(Ast::BinOp(op, Box::new(left), Box::new(right)))
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl<'i, 's: 'i> Iterator for Parser<'i, 's> {
    type Item = Ast<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.token_iter.next() {
            match token {
                Token::BinOp(_, op) => self.parse_bin_op(op),
                Token::Val(_, val) => Some(Ast::Val(val)),
                _ => None,
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
            Token::Val(0, Val::Int(3)),
            Token::BinOp(0, BinOp::Add),
            Token::Val(0, Val::Int(4)),
        ];

        let expected = vec![
            Ast::BinOp(
                BinOp::Add, 
                Box::new(Ast::Val(Val::Int(3))),
                Box::new(Ast::Val(Val::Int(4)))
            )
        ];
        
        let iter = input.iter();
        let mut iter_map = iter.map(Clone::clone);
        {
            let actual: Vec<Ast> = Parser::new(&mut iter_map).collect();
            assert_eq!(expected, actual);
        }
    }
}
