use super::evaluator::Evaluator;
use super::output::Output;
use super::scope::{OpenScope, Scope};
use compiler::parser::ast::{Ast, SetField, Id};
use compiler::tokens::{Val, BinOp};

#[test]
fn test_execute_binary_op() {
    // + 3 4

    let input = Ast::BinOp(
        BinOp::Add, 
        Box::new(Ast::Val(Val::Int(3))),
        Box::new(Ast::Val(Val::Int(4)))
        );

    let expected = Ok(Output::Int(7));

    let scope = OpenScope::new();
    let actual = Evaluator::new(&input, Scope::Open(&scope)).eval();
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
    let scope = OpenScope::new();
    let actual = Evaluator::new(&input, Scope::Open(&scope)).eval();
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
    let scope = OpenScope::new();
    let actual = Evaluator::new(&input, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}

#[test]
fn test_nested_let() {
    // let x = 1 in let y = 2 in + x y
    
    let inner = 
        Ast::Let(
            Box::new(
                SetField {
                    name: "y",
                    value: Ast::Val(Val::Int(2))
                }),
                Box::new(Ast::BinOp(
                        BinOp::Add,
                        Box::new(Ast::Id(Id(0, "x"))),
                        Box::new(Ast::Id(Id(0, "y"))))));
    let input = 
        Ast::Let(
            Box::new(
                SetField {
                    name: "x",
                    value: Ast::Val(Val::Int(1))
                }),
                Box::new(inner));

    let expected = Ok(Output::Int(3));
    let scope = OpenScope::new();
    let actual = Evaluator::new(&input, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}


#[test]
fn test_shadowing_works() {
    // let x = 1 in let x = 2 in + x 1
    
    let inner = 
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
    let input = 
        Ast::Let(
            Box::new(
                SetField {
                    name: "x",
                    value: Ast::Val(Val::Int(1))
                }),
                Box::new(inner));

    let expected = Ok(Output::Int(3));
    let scope = OpenScope::new();
    let actual = Evaluator::new(&input, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}


#[test]
fn test_function_call() {
    // (fn x: + x 1 2)
    let binop = Ast::BinOp(BinOp::Add,
                           Box::new(Ast::Id(Id(9, "x"))),
                           Box::new(Ast::Val(Val::Int(1))));
    let func = Ast::Fn("x", Box::new(binop));
    let fncall = 
        Ast::Call(
            Box::new(func),
            Box::new( Ast::Val(Val::Int(2))));

    let expected = Ok(Output::Int(3));
    let scope = OpenScope::new();
    let actual = Evaluator::new(&fncall, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}

#[test]
fn closure_works() {
    // let x = 2 in
    //  let func = fn y: y + x in
    //   (func 1)
    
    let fncall = Ast::Call(
        Box::new(Ast::Id(Id(0, "func"))),
        Box::new(Ast::Val(Val::Int(1))));
    let func = Ast::Fn("y", 
                       Box::new(Ast::BinOp(
                               BinOp::Add,
                               Box::new(Ast::Id(Id(0, "y"))),
                               Box::new(Ast::Id(Id(0, "x"))))));
    let inner_let = Ast::Let(
        Box::new(SetField{name: "func", value: func}),
        Box::new(fncall));


    let outer_let = Ast::Let(
        Box::new(SetField{
            name: "x",
            value: Ast::Val(Val::Int(2))
        }),
        Box::new(inner_let));


    let expected = Ok(Output::Int(3));
    let scope = OpenScope::new();
    let actual = Evaluator::new(&outer_let, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}

#[test]
fn showing_in_closure_works() {
    // let x = 100 in
    //  let func = fn x: x + 5 in
    //   (func 1)
    
    let fncall = Ast::Call(
        Box::new(Ast::Id(Id(0, "func"))),
        Box::new(Ast::Val(Val::Int(1))));
    let func = Ast::Fn("x", 
                       Box::new(Ast::BinOp(
                               BinOp::Add,
                               Box::new(Ast::Id(Id(0, "x"))),
                               Box::new(Ast::Val(Val::Int(5))))));
    let inner_let = Ast::Let(
        Box::new(SetField{name: "func", value: func}),
        Box::new(fncall));


    let outer_let = Ast::Let(
        Box::new(SetField{
            name: "x",
            value: Ast::Val(Val::Int(100))
        }),
        Box::new(inner_let));


    let expected = Ok(Output::Int(6));
    let scope = OpenScope::new();
    let actual = Evaluator::new(&outer_let, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}
