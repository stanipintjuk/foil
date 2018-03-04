use std::fmt::{Display, Formatter, self};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    BinOp(usize, BinOp),
    UnaryOp(usize, UnaryOp),
    Val(usize, Val),
    Keyword(usize, Keyword),
    Id(usize, String),
    GroupL(usize),
    GroupR(usize),
    BlockL(usize),
    BlockR(usize),
    Comma(usize),
    Colon(usize),
    Assign(usize),
    Semi(usize),
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Equals,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &BinOp::Add => write!(f, "+"),
            &BinOp::Sub => write!(f, "-"),
            &BinOp::Mul => write!(f, "*"),
            &BinOp::Div => write!(f, "/"),
            &BinOp::Mod => write!(f, "%"),
            &BinOp::Pow => write!(f, "**"),
            &BinOp::Equals => write!(f, "=="),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum UnaryOp {
    Not
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Val {
    Int(i64),
    Double(f64),
    String(String),
    Path(String),
    Bool(bool),
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Val::Int(ref x) => write!(f, "{}", x),
            &Val::Double(ref x) => write!(f, "{}", x),
            &Val::String(ref x) => write!(f, "\"{}\"", x),
            &Val::Path(ref x) => write!(f, "<{}>", x),
            &Val::Bool(ref x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Keyword {
    Let,
    Fn,
    Import,
    Set,
    In,
    Html,
}
