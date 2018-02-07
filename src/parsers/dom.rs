#![allow(non_snake_case)]

use nom::*;
use parsers::generic::*;
use constants::errors::*;
use std;
use std::cmp::PartialEq;

named!(valid_variable_name, take_while1!(is_valid_char));

named!(take_attribute_name, add_return_error!(
        ErrorKind::Custom(ERROR_INVALID_ATTRIBUTE_NAME),
        call!(valid_variable_name)));

named!(take_attribute_value<String>, add_return_error!(
        ErrorKind::Custom(ERROR_ATTRIBUTE_VALUE_NOT_STRING),
        call!(take_string)));

named!(take_attribute(&[u8]) -> (&[u8], String), do_parse!(
        name: take_attribute_name >>
        tag!("=") >>
        val: take_attribute_value >>
        (name, val)
    ));

named!(take_attributes(&[u8]) -> Vec<(&[u8], String)>,
        many0!(
            do_parse!(
                consume_space >> 
                attr: take_attribute >> (attr)
                )
            ));

named!(take_delimited_children<Vec<Box<DOMTree>>>, add_return_error!(
        ErrorKind::Custom(ERROR_EXPECTING_DELIMITERS),
        do_parse! (
        tag!("{") >>
        consume_space >>
        children: many0!(map!(parse_DOM_tree, Box::new)) >>
        consume_space >>
        tag!("}") >>
        (children)
    )));

named!(take_tag_name, add_return_error!(
        ErrorKind::Custom(ERROR_INVALID_TAG_NAME),
        call!(valid_variable_name)));

named!(take_DOM_node(&[u8]) -> DOMNode, add_return_error!(
        ErrorKind::Custom(ERROR_INVALID_DOM_NODE),
        do_parse!(
            name: take_tag_name >>
            attrs: take_attributes >>
            consume_space >> 
            children: alt!(
                take_delimited_children |
                parse_DOM_tree => { |n| vec![Box::new(n)] }) >>
            consume_space >>
            (DOMNode{name: name, attrs: attrs,  children: children})
    )));

named!(take_DOM_node_sc(&[u8]) -> DOMNodeSC, add_return_error!(
        ErrorKind::Custom(ERROR_INVALID_SELF_CLOSING_DOM_NODE),
        do_parse!(
            name: take_tag_name >>
            attrs: take_attributes >>
            consume_space >> 
            tag!(";") >>
            (DOMNodeSC{ name: name, attrs: attrs })
    )));

named!(take_expression(&[u8]) -> Expression, 
       add_return_error!(
           ErrorKind::Custom(ERROR_INVALID_EXPRESSION),
           alt!(
               map!(take_string, Expression::Text) |
               map!(take_path, Expression::Path)
            )));

named!(pub parse_DOM_tree(&[u8]) -> DOMTree, do_parse!(
        consume_space >>
        res: alt!(
            call!(take_expression) => { |expr| DOMTree::Content(expr) } |
            call!(take_DOM_node) => { |n| DOMTree::Node(Box::new(n)) } |
            call!(take_DOM_node_sc) => { |n| DOMTree::SelfClosingNode(Box::new(n)) }
            ) >>
        consume_space >>
        (res)
        )
      );

fn buf_to_string(buf: &[u8]) -> String {
    std::str::from_utf8(buf).unwrap().to_string() 
}

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

fn is_valid_char(ascii: u8) -> bool {
    (65 <= ascii && ascii <= 90) || //capital letters
    (97 <= ascii && ascii <= 122) || // letters
    (48 <= ascii && ascii <= 57) || // numbers
    ascii == 45 || // hyphen
    ascii == 95 // under line

}


#[cfg(test)]
mod tests {
    use super::*;
    use nom::*;

    #[test]
    fn parse_dom_tree_works_with_selfclosing() {
        //Should work with self-closing tags
        let expected_tree = DOMTree::SelfClosingNode(
            Box::new(DOMNodeSC{name: b"br", attrs: vec![]})
        );
        assert_eq!(IResult::Done(&[][..], expected_tree), parse_DOM_tree(b"br;"));
    }

    #[test]
    fn parse_dom_tree_works_with_children() {
        let expected_tree = DOMTree::Node(
                Box::new(DOMNode{ 
                    name: b"html", 
                    attrs: vec![],
                    children: vec![
                        Box::new(DOMTree::Content(Expression::Text("hej".to_string()))),
                        Box::new(DOMTree::Content(Expression::Path("/path/to/file".to_string())))
                    ]
                })
            );
        assert_eq!(IResult::Done(&[][..], expected_tree),parse_DOM_tree(b"html { \"hej\" <path/to/file>}"));
    }
}
