#![allow(non_snake_case)]

use nom::*;
use parsers::generic::*;
use parsers::errors::*;
use parsers::dom::types::*;
use std;

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
        children: many0!(map!(parse_dom_tree, Box::new)) >>
        consume_space >>
        tag!("}") >>
        (children)
    )));

named!(take_tag_name, add_return_error!(
        ErrorKind::Custom(ERROR_INVALID_TAG_NAME),
        call!(valid_variable_name)));

named!(take_dom_node(&[u8]) -> DOMNode, add_return_error!(
        ErrorKind::Custom(ERROR_INVALID_DOM_NODE),
        do_parse!(
            name: take_tag_name >>
            attrs: take_attributes >>
            consume_space >> 
            children: alt!(
                take_delimited_children |
                parse_dom_tree => { |n| vec![Box::new(n)] }) >>
            consume_space >>
            (DOMNode{name: name, attrs: attrs,  children: children})
    )));

named!(take_dom_node_sc(&[u8]) -> DOMNodeSC, add_return_error!(
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

named!(pub parse_dom_tree(&[u8]) -> DOMTree, do_parse!(
        consume_space >>
        res: alt!(
            call!(take_expression) => { |expr| DOMTree::Content(expr) } |
            call!(take_dom_node) => { |n| DOMTree::Node(Box::new(n)) } |
            call!(take_dom_node_sc) => { |n| DOMTree::SelfClosingNode(Box::new(n)) }
            ) >>
        consume_space >>
        (res)
        )
      );


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
        assert_eq!(IResult::Done(&[][..], expected_tree), parse_dom_tree(b"br;"));
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
        assert_eq!(IResult::Done(&[][..], expected_tree),parse_dom_tree(b"html { \"hej\" </path/to/file>}"));
    }

    #[test]
    fn parse_dom_tree_works_with_attributes() {
        let expected_tree = DOMTree::Node(
            Box::new(DOMNode{
                name: b"html",
                attrs: vec![
                    (b"attr1", "value1".to_string()),
                    (b"attr-with-stuff", "value2".to_string()),
                    (b"attr_underline",  "value3".to_string()),
                ],
                children: vec![]
            }));
        assert_eq!(IResult::Done(&[][..], expected_tree), 
                   parse_dom_tree(b"html 
                                  attr1=\"value1\" 
                                  attr-with-stuff=\"value2\"
                                  attr_underline=\"value3\" {}"));
    }
}
