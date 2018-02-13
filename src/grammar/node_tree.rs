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
    Sum(Box<Content>, Box<Content>)
}
impl PartialEq for Content {
    fn eq(&self, other: &Content) -> bool {
        match (self, other) {
            (&Content::Literal(ref s1), &Content::Literal(ref s2)) => s1 == s2,
            (&Content::Path(ref p1, ref pos1), 
             &Content::Path(ref p2, ref pos2)) => p1 == p2 && pos1 == pos2,
            (&Content::Sum(ref l1, ref l2),
             &Content::Sum(ref r1, ref r2)) => l1 == r1 && l2 == r2,
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


