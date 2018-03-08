use super::tokenizer::Tokenizer;
use super::parser::Parser;
use super::evaluator::{Evaluator, Scope, OpenScope};
use super::models::Output;

#[test]
fn trivial_test() {
    let input = "+ 1 2";
    let expected = Output::Int(3);

    let mut tokenizer = Tokenizer::new(input);
    let mut parser = Parser::new(&mut tokenizer);
    let ast = parser.next().unwrap().unwrap();

    let scope = OpenScope::new();
    let actual = Evaluator::without_files(&ast,  Scope::Open(&scope)).eval();
    assert_eq!(Ok(expected), actual);
}
