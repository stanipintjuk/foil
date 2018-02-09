use std::cmp::PartialEq;
use std;

#[derive(Debug)]
pub enum Expression {
    Text(String),
    Path(String)
}

impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        match (self, other) {
            (&Expression::Text(ref my_s), &Expression::Text(ref other_s)) => my_s == other_s,
            (&Expression::Path(ref my_p), &Expression::Path(ref other_p)) => my_p == other_p,
            _ => false,
        }
    }
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            &Expression::Text(ref string) => string.clone(),
            &Expression::Path(ref path) => path.clone(),
        }
    }
}

#[derive(Debug)]
pub enum DOMTree<'a> {
    Node(Box<DOMNode<'a>>),
    SelfClosingNode(Box<DOMNodeSC<'a>>),
    Content(Expression),
}

impl<'a> DOMTree<'a> {
    pub fn to_html(&self) -> String {
        match self {
            &DOMTree::Node(ref n) => n.to_html(),
            &DOMTree::SelfClosingNode(ref n) => n.to_html(),
            &DOMTree::Content(ref n) => n.to_string(),
        }
    }
}

impl<'a> PartialEq for DOMTree<'a> {
    fn eq(&self, other: &DOMTree) -> bool {
        match (self, other) {
            (&DOMTree::Node(ref my_node), &DOMTree::Node(ref other_node)) => my_node == other_node,
            (&DOMTree::SelfClosingNode(ref my_node), &DOMTree::SelfClosingNode(ref other_node)) => my_node == other_node,
            (&DOMTree::Content(ref my_expr), &DOMTree::Content(ref other_expr)) => my_expr == other_expr,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct DOMNodeSC<'a> {
    pub name: &'a [u8],
    pub attrs: Vec<(&'a [u8], String)>,
}

impl<'a> PartialEq for DOMNodeSC<'a> {
    fn eq(&self, other: &DOMNodeSC) -> bool {
        self.name == other.name && self.attrs == other.attrs
    }
}
impl<'a> DOMNodeSC<'a> {
    pub fn to_html(&self) -> String {
        let attrs = self.attrs.iter()
            .map(|&(ref attr, ref val)|{ 
                format!(" {}=\"{}\"", buf_to_string(attr), val)
            })
            .fold("".to_string(), |attr, acc| {
                format!("{}{}", attr, acc)
            });

        format!("<{}{}/>", buf_to_string(self.name), attrs)
    }
}

#[derive(Debug)]
pub struct DOMNode<'a> {
    pub name: &'a [u8],
    pub attrs: Vec<(&'a [u8], String)>,
    pub children: Vec<Box<DOMTree<'a>>>,
}

impl<'a> PartialEq for DOMNode<'a> {
    fn eq(&self, other: &DOMNode<'a>) -> bool {
        self.name ==  other.name &&
        self.attrs == other.attrs &&
        self.children == other.children
    }
}

impl<'a> DOMNode<'a> {
    pub fn to_html(&self) -> String {
        let attrs = self.attrs.iter()
            .map(|&(ref attr, ref val)|{ 
                format!(" {}=\"{}\"", buf_to_string(attr), val)
            })
            .fold("".to_string(), |attr, acc| {
                format!("{}{}", attr, acc)
            });

        let children = self.children.iter()
            .map(|child_node| {child_node.to_html()})
            .fold("".to_string(), |child_html, acc| {
                format!("{}{}", child_html, acc)
            });

        format!("<{}{}>{}</{}>", 
                buf_to_string(self.name), 
                attrs, 
                children, 
                buf_to_string(self.name))
    }
}

fn buf_to_string(buf: &[u8]) -> String {
    std::str::from_utf8(buf).unwrap().to_string() 
}
