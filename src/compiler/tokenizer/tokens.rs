#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token<'a> {
    BinOp(usize, BinOp),
    UnaryOp(usize, UnaryOp),
    Val(usize, Val<'a>),
    Keyword(usize, Keyword),
    Id(usize, &'a str),
    GroupL(usize),
    GroupR(usize),
    BlockL(usize),
    BlockR(usize),
    Comma(usize),
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
    Assign,
    Equals,
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
pub enum Val<'a> {
    Int(i64),
    Double(f64),
    String(&'a str),
    Path(&'a str),
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Keyword {
    Let,
    Fn,
    Import,
    Set,
}
