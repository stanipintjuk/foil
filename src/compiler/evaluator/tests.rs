use super::evaluator::{Evaluator, Output, Scope};
use compiler::parser::ast::{Ast, SetField, Id};
use compiler::tokens::{Val, BinOp};
use std::collections::LinkedList;

#[test]
fn test_execute_binary_op() {
    // + 3 4

    let input = Ast::BinOp(
        BinOp::Add, 
        Box::new(Ast::Val(Val::Int(3))),
        Box::new(Ast::Val(Val::Int(4)))
        );

    let expected = Ok(Output::Int(7));

    let scope = Scope::new();
    let actual = Evaluator::new(&input, &scope).eval();
    assert_eq!(expected, actual);
}

#[test]
fn test_execute_recursive() {
    // + - 1 2 3  
    // = (1 - 2) + 3 = (-1) + 3 = 2
    let input = Ast::BinOp(
        BinOp::Add,
        Box::new(Ast::BinOp(
                BinOp::Sub,
                Box::new(Ast::Val(Val::Int(1))),
                Box::new(Ast::Val(Val::Int(2))))),
                Box::new(Ast::Val(Val::Int(3))));
    let expected = Ok(Output::Int(2));
    let scope = Scope::new();
    let actual = Evaluator::new(&input, &scope).eval();
    assert_eq!(expected, actual);
}

#[test]
fn test_execute_let_statement() {
    // let x = 2 in + x 1
    let input = 
        Ast::Let(
            Box::new(
                SetField {
                    name: "x",
                    value: Ast::Val(Val::Int(2))
                }),
                Box::new(Ast::BinOp(
                        BinOp::Add,
                        Box::new(Ast::Id(Id(0, "x"))),
                        Box::new(Ast::Val(Val::Int(1))))));

    let expected = Ok(Output::Int(3));
    let scope = Scope::new();
    let actual = Evaluator::new(&input, &scope).eval();
    assert_eq!(expected, actual);
}


