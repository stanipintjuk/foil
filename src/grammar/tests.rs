#![cfg(test)]
use grammar::{node, ParseError, ClosedNode, OpenNode, NodeKind, Content};

#[test]
fn self_closing_tag_works() {
    let expected_node = NodeKind::ClosedNode(
        ClosedNode{name: "br", attributes: vec![]});
    assert_eq!(Ok(expected_node), node("br;"));
}

#[test]
fn open_tag_works() {
    let expected_node = NodeKind::OpenNode(
        OpenNode{name: "br", attributes: vec![], children: vec![]});
    assert_eq!(Ok(expected_node), node("br{}"));
}

#[test]
fn single_attribute_open_node_works() {
    let expected = NodeKind::OpenNode(
        OpenNode{
            name: "div", 
            attributes: vec![("class", Content::Literal("row col12".to_string()))], 
            children: vec![]
        });
    assert_eq!(Ok(expected), node("div class=\"row col12\" {}"));
}

#[test]
fn single_attribute_closed_node_works() {
    let expected = NodeKind::ClosedNode(
        ClosedNode{
            name: "div", 
            attributes: vec![("class", Content::Literal("row col12".to_string()))],
        });
    assert_eq!(Ok(expected), node("div class=\"row col12\";"));
}

#[test]
fn multiple_attributes_open_node_works() {
    let expected = NodeKind::OpenNode(
        OpenNode{
            name: "div", 
            attributes: vec![
                ("id", Content::Literal("div1".to_string())), 
                ("class", Content::Literal("row col12".to_string()))], 
            children: vec![]
        });
    assert_eq!(Ok(expected), node("div id=\"div1\" class=\"row col12\" {}"));
}

#[test]
fn multiple_attribute_closed_node_works() {
    let expected = NodeKind::ClosedNode(
        ClosedNode{
            name: "div", 
            attributes: vec![
                ("id", Content::Literal("div2".to_string())), 
                ("class", Content::Literal("row col12".to_string()))],
        });
    assert_eq!(Ok(expected), node("div id=\"div2\" class=\"row col12\";"));
}

#[test]
fn open_node_single_child_works() {
    let child = NodeKind::Content(Content::Literal("test".to_string()));
    let expected = NodeKind::OpenNode(
        OpenNode{
            name: "h1", 
            attributes: vec![],
            children: vec![child],
        });
    assert_eq!(Ok(expected), node("h1 \"test\""));
}

#[test]
fn open_node_multiple_children_works() {
    let li1 = NodeKind::OpenNode(OpenNode{
        name: "li", attributes: vec![], 
        children: vec![NodeKind::Content(
            Content::Literal("list item 1".to_string()))],
    });

    let li2 = NodeKind::OpenNode(OpenNode{
        name: "li", attributes: vec![], 
        children: vec![NodeKind::Content(
            Content::Literal("list item 2".to_string()))],
    });

    let expected = NodeKind::OpenNode(
        OpenNode{
            name: "ul", 
            attributes: vec![],
            children: vec![li1, li2],
        });
    assert_eq!(Ok(expected), 
               node("ul 
                        { 
                            li \"list item 1\" 
                            li \"list item 2\"
                        }")
              );
}

#[test]
fn returns_correct_error_on_nonsence() {
    use std::collections::HashSet;

    let mut expected_symbols = HashSet::new();
    expected_symbols.insert("<");
    expected_symbols.insert(";");
    expected_symbols.insert("[a-zA-Z0-9]");
    expected_symbols.insert("{");
    expected_symbols.insert("\"");

    let expected = ParseError { 
        line: 2, 
        column: 17, 
        offset: 39, 
        expected: expected_symbols};

    assert_eq!(Err(expected), node("div class=\"row col12\" \n{ some nonsence }"));
}

#[test]
fn strings_work_with_escaped_chars() {
    let expected = NodeKind::Content(
        Content::Literal("\"\\testing".to_string()));
    assert_eq!(Ok(expected), node("\"\\\"\\\\testing\""));
}

#[test]
fn path_works_with_escaped_chars() {
    let expected = NodeKind::Content(
        Content::Path("strange\\folder>name".to_string(), 0));
    assert_eq!(Ok(expected), node("<strange\\\\folder\\>name>"));
}
