#[macro_use] extern crate nom;
use nom::*;

named!(valid_variable_name, take_while1!(is_valid_char));

named!(pub take_string<String>, map!(delimited!(
        tag!("\""), 
        escaped_transform!(is_not!("\"\\"), '\\', tag!("\"")), 
        tag!("\"")),
        |bytes| { String::from_utf8(bytes).unwrap() }
    ));

named!(pub take_attribute_name, add_return_error!(
        ErrorKind::Custom(ERROR_INVALID_ATTRIBUTE_NAME),
        call!(valid_variable_name)));

named!(pub take_attribute_value<String>, add_return_error!(
        ErrorKind::Custom(ERROR_ATTRIBUTE_VALUE_NOT_STRING),
        call!(take_string)));

named!(pub take_attribute(&[u8]) -> (&[u8], String), do_parse!(
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

named!(pub take_delimited_children<Vec<Box<DOMTree>>>, add_return_error!(
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
           map!(take_string, Expression::Text)));

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

named!(consume_space, take_while!(is_whitespace));

pub const ERROR_INVALID_TAG_NAME: u32 = 1;
pub const ERROR_INVALID_ATTRIBUTE_NAME: u32 = 2;
pub const ERROR_ATTRIBUTE_VALUE_NOT_STRING: u32 = 3;
pub const ERROR_EXPECTED_WHITESPACE: u32 = 4;
pub const ERROR_INVALID_EXPRESSION: u32 = 5;
pub const ERROR_INVALID_DOM_NODE: u32 = 6;
pub const ERROR_INVALID_SELF_CLOSING_DOM_NODE: u32 = 7;
pub const ERROR_EXPECTING_DELIMITERS: u32 = 8;

fn buf_to_string(buf: &[u8]) -> String {
    std::str::from_utf8(buf).unwrap().to_string() 
}

#[derive(Debug)]
pub enum Expression {
    Text(String)
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            &Expression::Text(ref string) => string.clone(),
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

#[derive(Debug)]
pub struct DOMNodeSC<'a> {
    pub name: &'a [u8],
    pub attrs: Vec<(&'a [u8], String)>,
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

pub fn is_whitespace(ascii: u8) -> bool {
    ascii == '\n' as u8 || ascii == ' ' as u8 || ascii == '\t' as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::*;

    #[test]
    fn tag_name_works() {
        assert_eq!(Done(&[][..], b"html"), year(b"html"));
        assert_eq!(Done(&[][..], b"html []"), year(b"html"));
    }
}
