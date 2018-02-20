use compiler::tokens::{BinOp, Val};

/// AST - Abstract Syntax Tree
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Ast<'a> {
    BinOp(BinOp, Box<Ast<'a>>, Box<Ast<'a>>),
    Val(Val<'a>),
    Set(Set<'a>),
    Let(Box<SetField<'a>>, Box<Ast<'a>>),
    Fn(&'a str, Box<Ast<'a>>),
    Call(Box<Ast<'a>>, Box<Ast<'a>>),
    Id(Id<'a>),
    Import(usize, &'a str),
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Id<'a>(pub usize, pub &'a str);

pub type Set<'a> = Vec<SetField<'a>>;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct SetField<'a> {
    pub name: &'a str,
    pub value: Ast<'a>,
}
