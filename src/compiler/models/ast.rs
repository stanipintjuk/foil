use compiler::models::{BinOp, Val};
use std::fmt::{Display, Formatter, self};

/// AST - Abstract Syntax Tree
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Ast {
    BinOp(BinOp, Box<Ast>, Box<Ast>),
    Val(Val),
    Set(Set),
    Let(Box<SetField>, Box<Ast>),
    Fn(String, Box<Ast>),
    Call(Box<Ast>, Box<Ast>),
    Id(Id),
    Import(usize, String),

    /// Represents an HTML element
    Html{
        tag_name: String, 
        attributes: Vec<SetField>,
        children: Vec<Ast>,
    },

    /// Represents a self-closing tag HTML element
    HtmlClosed{
        tag_name: String,
        attributes: Vec<SetField>
    },
}

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Ast::BinOp(ref op, ref l, ref r) => write!(f, "{} {} {}", op, l, r),
            &Ast::Val(ref val) => write!(f, "{}", val),
            &Ast::Set(_) => write!(f, "set.."),
            &Ast::Let(ref field, ref expr) => write!(f, "let {}={} in {}", field.name, field.value, expr),
            &Ast::Fn(ref param, ref expr) => write!(f, "fn {}: {}", param, expr),
            &Ast::Call(ref param, ref expr) => write!(f, "({} {})", param, expr),
            &Ast::Id(ref id) => write!(f, "{}", id.1),
            &Ast::Import(_, ref file) => write!(f, "import {}", file),
            &Ast::Html{..} | &Ast::HtmlClosed{..} => write!(f, "html!.."),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Id(pub usize, pub String);

pub type Set = Vec<SetField>;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct SetField {
    pub name: String,
    pub value: Ast,
}
