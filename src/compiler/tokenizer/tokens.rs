#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token<'a> {
    Op(usize, Op),
    Val(usize, Val<'a>),
    Keyword(usize, Keyword),
    Id(usize, &'a str),
    Garbage(usize, &'a str),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Not,
    Assign,
    Equals,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Val<'a> {
    Int(i64),
    Double(f64),
    String(&'a str),
    Path(&'a str),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Keyword {
    Let,
    Fn,
    Import,
    Set,
}
