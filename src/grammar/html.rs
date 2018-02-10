pub use self::html::{node, ParseError};

peg! html(r#"
use super::*;

#[pub]
node -> NodeKind<'input>
    = whitespace? n:node_kind whitespace? {n}

node_kind -> NodeKind<'input>
    = n:closed_node { NodeKind::ClosedNode(n) }
    / n:open_node { NodeKind::OpenNode(n) }
    / n:content { NodeKind::Content(n) }

closed_node -> ClosedNode<'input>
    = n:tag_name a:attributes whitespace ?";" { 
            ClosedNode{ name: n, attributes: a } 
        }

open_node -> OpenNode<'input>
    = n:tag_name a:attributes whitespace? c:children { 
        OpenNode{
            name: n, 
            attributes: a,
            children: c,
        }}

children -> Vec<NodeKind<'input>>
    = "{" whitespace? c:node* whitespace? "}" { c }
    / c:node_kind { vec![c] }

content -> Content
    = s:string {Content::Literal(s)}
    / pos:#position p:path {Content::Path(p, pos)}

tag_name -> &'input str
    = $([a-zA-Z0-9]+)

attributes -> Vec<Attribute<'input>>
    = attribute*

attribute -> Attribute<'input>
    = whitespace n:tag_name "=" v:content { (n, v) }

string -> String
    = "\"" s:$(("\\\\"/"\\\""/[^\"])*) "\"" { strip_escape_chars(s, "\\", "\"") }

path -> String
    = "<" s:$(( "\\\\" / "\\>" /[^>])*) ">" { strip_escape_chars(s, "\\", ">") }

whitespace = #quiet<[ \n\t]+>

"#);

fn strip_escape_chars(s: &str, escape_char: &str, delimiter: &str) -> String {
    let escaped_escape_char = format!("{}{}", escape_char, escape_char);
    let escaped_delimiter = format!("{}{}", escape_char, delimiter);

    s.replace(&escaped_escape_char, &escape_char)
        .replace(&escaped_delimiter, &delimiter)
}

/// A tuple that describes an attribute's name and value
pub type Attribute<'a> = (&'a str,  Content);

/// A node the DOM tree
#[derive(Debug)]
pub enum NodeKind<'a> {
    OpenNode(OpenNode<'a>),
    ClosedNode(ClosedNode<'a>),
    Content(Content),
}
impl<'a> PartialEq for NodeKind<'a> {
    fn eq(&self, other: &NodeKind) -> bool {
        match (self, other) {
            (&NodeKind::OpenNode(ref n1), &NodeKind::OpenNode(ref n2)) => n1 == n2,
            (&NodeKind::ClosedNode(ref n1), &NodeKind::ClosedNode(ref n2)) => n1 == n2,
            (&NodeKind::Content(ref n1), &NodeKind::Content(ref n2)) => n1 == n2,
            _ => false
        }
    }
}

/// Represents different kinds of content in the HTML.
#[derive(Debug)]
pub enum Content {
    Literal(String),

    /// Stores the path as a string and the position 
    /// of this element in the code, so that it is easier
    /// to generate error messages.
    Path(String, usize),
}
impl PartialEq for Content {
    fn eq(&self, other: &Content) -> bool {
        match (self, other) {
            (&Content::Literal(ref s1), &Content::Literal(ref s2)) => s1 == s2,
            (&Content::Path(ref p1, ref pos1), 
             &Content::Path(ref p2, ref pos2)) => p1 == p2 && pos1 == pos2,
            _ => false
        }
    }
}

/// Represents a normal DOM node.
#[derive(Debug)]
pub struct OpenNode<'a> {
    pub name: &'a str,
    pub attributes: Vec<Attribute<'a>>,
    pub children: Vec<NodeKind<'a>>,
}
impl<'a> PartialEq for OpenNode<'a> {
    fn eq(&self, other: &OpenNode) -> bool {
        self.name == other.name && self.attributes == other.attributes
    }
}

/// A self-closing node in the DOM tree.
/// like e.g. <br/>
#[derive(Debug)]
pub struct ClosedNode<'a> {
    pub name: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}
impl<'a> PartialEq for ClosedNode<'a> {
    fn eq(&self, other: &ClosedNode) -> bool {
        self.name == other.name && self.attributes == other.attributes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
