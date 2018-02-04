#[macro_use] extern crate nom;
use nom::*;

named!(valid_variable_name(&[u8]) -> String, 
       map!(take_while!(is_alpha), buf_to_string));

named!(pub tag_name(&[u8]) -> String, call!(valid_variable_name));
named!(pub take_string(&[u8]) -> String, 
       map!(delimited!(tag!("\""), is_not!("\""), tag!("\"")), buf_to_string));

named!(pub take_attribute(&[u8]) -> (String, String), do_parse!(
        name: valid_variable_name >>
        tag!("=") >>
        val: take_string >>
        (name, val)
    ));

named!(take_attributes(&[u8]) -> Vec<(String, String)>,
        many0!(
            do_parse!(
                consume_space >> 
                attr: take_attribute >> (attr)
                )
            ));

named!(take_DOM_node(&[u8]) -> DOMNode, do_parse!(
        name: valid_variable_name >>
        attrs: take_attributes >>
        consume_space >> 
        tag!("{") >>
        consume_space >>
        children: many0!(parse_DOM_tree) >>
        consume_space >>
        tag!("}") >>
        consume_space >>
        (DOMNode{name: name, attrs: attrs,  children: children})
    ));

named!(take_DOM_node_sc(&[u8]) -> DOMNodeSC, do_parse!(
        name: valid_variable_name >>
        attrs: take_attributes >>
        consume_space >> 
        tag!(";") >>
        (DOMNodeSC{ name: name, attrs: attrs })
    ));

named!(take_DOM_node_single_child(&[u8]) -> DOMNode, do_parse!(
        name: valid_variable_name >>
        attrs: take_attributes >>
        consume_space >> 
        child: parse_DOM_tree >>
        (DOMNode{name: name, attrs: attrs,  children: vec![child]})
    ));

named!(take_expression(&[u8]) -> Expression, map!(take_string, Expression::Text));
named!(pub parse_DOM_tree(&[u8]) -> DOMTree, alt!(
            call!(take_DOM_node) => { |n| DOMTree::Node(n) } |
            call!(take_DOM_node_single_child) => { |n| DOMTree::Node(n) } |
            call!(take_DOM_node_sc) => { |n| DOMTree::SelfClosingNode(n) } |
            call!(take_expression) => { |expr| DOMTree::Content(expr) }
        )
    );

named!(consume_space, take_while!(is_whitespace));

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
pub enum DOMTree {
    Node(DOMNode),
    SelfClosingNode(DOMNodeSC),
    Content(Expression),
}

impl DOMTree {
    pub fn to_html(&self) -> String {
        match self {
            &DOMTree::Node(ref n) => n.to_html(),
            &DOMTree::SelfClosingNode(ref n) => n.to_html(),
            &DOMTree::Content(ref n) => n.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct DOMNodeSC {
    pub name: String,
    pub attrs: Vec<(String, String)>,
}
impl DOMNodeSC {
    pub fn to_html(&self) -> String {
        let attrs = self.attrs.iter()
            .map(|&(ref attr, ref val)|{ 
                format!(" {}=\"{}\"", attr, val)
            })
            .fold("".to_string(), |attr, acc| {
                format!("{}{}", attr, acc)
            });

        format!("<{}{}/>", self.name, attrs)
    }
}

#[derive(Debug)]
pub struct DOMNode {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<DOMTree>,
}

impl DOMNode {
    pub fn to_html(&self) -> String {
        let attrs = self.attrs.iter()
            .map(|&(ref attr, ref val)|{ 
                format!(" {}=\"{}\"", attr, val)
            })
            .fold("".to_string(), |attr, acc| {
                format!("{}{}", attr, acc)
            });

        let children = self.children.iter()
            .map(|child_node| {child_node.to_html()})
            .fold("".to_string(), |child_html, acc| {
                format!("{}\n{}", child_html, acc)
            });

        format!("<{}{}>\n{}\n</{}>", 
                self.name, 
                attrs, 
                children, 
                self.name)
    }
}

pub fn is_alpha(ascii: u8) -> bool {
    (65 <= ascii && ascii <= 90) ||
    (97 <= ascii && ascii <= 122)
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
