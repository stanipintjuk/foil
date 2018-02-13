use grammar::node_tree::*;

#[derive(Debug)]
pub enum Expression {
    Sum(Atom, Atom),
    Atom(Atom),
}
impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        match (self, other) {
            (&Expression::Sum(ref l1, ref l2), 
             &Expression::Sum(ref r1, ref r2)) => l1 == r1 && l2 == r2,
            (&Expression::Atom(ref l), &Expression::Atom(ref r)) => l == r,
            _ => false
        }
    }
}

#[derive(Debug)]
pub enum Atom {
    Content(Content),
    Expression(Box<Expression>),
}
impl PartialEq for Atom {
    fn eq(&self, other: &Atom) -> bool {
        match (self, other) {
            (&Atom::Content(ref l), &Atom::Content(ref r)) => l == r,
            (&Atom::Expression(ref l), &Atom::Expression(ref r)) => l == r,
            _ => false,
        }
    }
}
