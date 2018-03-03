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
