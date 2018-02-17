use compiler::tokens::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Ast<'a> {
    BinOp(BinOp, Box<Ast<'a>>, Box<Ast<'a>>),
    Val(Val<'a>),
    Set(Vec<SetField<'a>>)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct SetField<'a> {
    pub name: &'a str,
    pub value: Ast<'a>,
}
