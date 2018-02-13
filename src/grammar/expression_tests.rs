#![cfg(test)]
use grammar::{
    NodeKind, 
    expression, 
    Content, 
    Expression, 
    Atom};

#[test]
fn content_expression_works() {
    let expected = 
        Expression::Atom(
            Atom::Content(
                Content::Literal("test".to_string())));
    assert_eq!(Ok(expected), expression("\"test\""));
}

#[test]
fn sum_works() {
    let l = Atom::Content(
        Content::Literal("teststring".to_string()));
    let r = Atom::Content(
        Content::Path("testpath".to_string(), 15));
    let expr = Expression::Sum(l, r);
    assert_eq!(Ok(expr), 
        expression("\"teststring\" + <testpath>"));
    
}

#[test]
fn parens_work() {
    let l1 = Atom::Content(
        Content::Literal("test".to_string()));
    let l2 = Atom::Content(
        Content::Path("path".to_string(), 10));
    let r = Atom::Content(
        Content::Path("anotherpath".to_string(), 20));
    let expr = Expression::Sum(
        Atom::Content(Content::Expression(Box::new(
                Expression::Sum(l1, l2)))), r);
    assert_eq!(Ok(expr), expression("(\"test\" + <path>) + <anotherpath>"));
}
