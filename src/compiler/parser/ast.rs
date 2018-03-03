use compiler::tokenizer::tokens::{BinOp, Val};

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
