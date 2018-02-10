pub use self::arithmetic::expression;

peg! arithmetic(r#"
#[pub]
expression -> i64
    = sum
sum -> i64
    = l:product "+" r:product { l+r }
    / product
product -> i64
    = l:atom "*" r:atom { l*r }
    / atom
atom -> i64
    = number
    / "(" v:sum ")" { v }
number -> i64
    = n:$([0-9]+) { n.parse().unwrap() }
"#);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        assert_eq!(expression("1+1"), Ok(2));
    }

    #[test]
    fn multiplication_works() {
        assert_eq!(expression("2*3"), Ok(6));
    }

    #[test]
    fn multiplication_before_addition_works() {
        assert_eq!(expression("2*3+2*5"), Ok(16));
    }
}
