use compiler::tokens::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Ast<'a> {
    BinOp(BinOp, Box<Ast<'a>>, Box<Ast<'a>>),
    Val(Val<'a>),
}
