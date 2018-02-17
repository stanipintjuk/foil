use compiler::tokens::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Ast<'a> {
    BinOp(BinOp, Box<Ast<'a>>, Box<Ast<'a>>),
    Val(Val<'a>),
    Set(Set<'a>),
    Let(Box<SetField<'a>>, Box<Ast<'a>>),
    Id(usize, &'a str),
}

pub type Set<'a> = Vec<SetField<'a>>;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct SetField<'a> {
    pub name: &'a str,
    pub value: Ast<'a>,
}
