use grammar::html::{NodeKind, OpenNode, ClosedNode, Content, Attribute};

/// Takes an NodeKind and returns the entire html it represents.
pub fn into_html<'a>(node: &NodeKind<'a>) -> String {
    match node {
        &NodeKind::OpenNode(ref n) => open_node_into_html(n),
        &NodeKind::ClosedNode(ref n) => closed_node_into_html(n),
        &NodeKind::Content(ref n) => content_node_into_html(n),
    }
}

fn open_node_into_html<'a>(node: &OpenNode<'a>) -> String {
    format!("<{}{}>{}</{}>", 
            node.name, 
            attributes_to_string(&node.attributes),
            node_list_into_html(&node.children),
            node.name
            )
}

fn closed_node_into_html<'a>(node: &ClosedNode<'a>) -> String {
    format!("<{}{}/>",
            node.name,
            attributes_to_string(&node.attributes)
            )
}

fn content_node_into_html<'a>(node: &Content) -> String {
    match node {
        &Content::Literal(ref s) => s.to_string()
    }
}

fn attributes_to_string<'a>(attribs: &Vec<Attribute<'a>>) -> String {
    attribs.iter()
        .map(|&(n, ref v)| format!("{}=\"{}\"", n, v))
        .fold(String::new(), |acc, attrib|{ format!("{} {}", acc, attrib)})
}

fn node_list_into_html<'a>(nodes: &Vec<NodeKind<'a>>) -> String {
    nodes.iter()
        .map(into_html)
        .fold(String::new(), |acc, html|{ format!("{}{}", acc, html)})
}


#[cfg(test)]
mod tests {
    use super::*;
    use grammar::html::node;

    #[test]
    fn self_closing_tag_works() {
        let dom_node = node("br;").unwrap();
        let expected = "<br/>";
        assert_eq!(expected, into_html(&dom_node));
    }

    #[test]
    fn open_tag_works() {
        let dom_node = node("p{}").unwrap();
        let expected = "<p></p>";
        assert_eq!(expected, into_html(&dom_node));
    }

    #[test]
    fn attributes_work() {
        let dom_node = node("div id=\"div1\" class=\"yellow\" { }").unwrap();
        let expected = "<div id=\"div1\" class=\"yellow\"></div>";
        assert_eq!(expected, into_html(&dom_node));
    }

    #[test]
    fn children_work() {
        let dom_node = node("ul { li \"first row\" li \"second row\" }").unwrap();
        let expected = "<ul><li>first row</li><li>second row</li></ul>";
        assert_eq!(expected, into_html(&dom_node));
    }
}
