use super::evaluator::Evaluator;
use super::output::Output;
use super::scope::{OpenScope, Scope};

use compiler::parser::ast::{Ast, SetField, Id};
use compiler::tokenizer::tokens::{Val, BinOp};

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
                    name: "x".to_string(),
                    value: Ast::Val(Val::Int(2))
                }),
                Box::new(Ast::BinOp(
                        BinOp::Add,
                        Box::new(Ast::Id(Id(0, "x".to_string()))),
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
                    name: "y".to_string(),
                    value: Ast::Val(Val::Int(2))
                }),
                Box::new(Ast::BinOp(
                        BinOp::Add,
                        Box::new(Ast::Id(Id(0, "x".to_string()))),
                        Box::new(Ast::Id(Id(0, "y".to_string()))))));
    let input = 
        Ast::Let(
            Box::new(
                SetField {
                    name: "x".to_string(),
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
                    name: "x".to_string(),
                    value: Ast::Val(Val::Int(2))
                }),
                Box::new(Ast::BinOp(
                        BinOp::Add,
                        Box::new(Ast::Id(Id(0, "x".to_string()))),
                        Box::new(Ast::Val(Val::Int(1))))));
    let input = 
        Ast::Let(
            Box::new(
                SetField {
                    name: "x".to_string(),
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
                           Box::new(Ast::Id(Id(9, "x".to_string()))),
                           Box::new(Ast::Val(Val::Int(1))));
    let func = Ast::Fn("x".to_string(), Box::new(binop));
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
        Box::new(Ast::Id(Id(0, "func".to_string()))),
        Box::new(Ast::Val(Val::Int(1))));
    let func = Ast::Fn("y".to_string(), 
                       Box::new(Ast::BinOp(
                               BinOp::Add,
                               Box::new(Ast::Id(Id(0, "y".to_string()))),
                               Box::new(Ast::Id(Id(0, "x".to_string()))))));
    let inner_let = Ast::Let(
        Box::new(SetField{name: "func".to_string(), value: func}),
        Box::new(fncall));


    let outer_let = Ast::Let(
        Box::new(SetField{
            name: "x".to_string(),
            value: Ast::Val(Val::Int(2))
        }),
        Box::new(inner_let));


    let expected = Ok(Output::Int(3));
    let scope = OpenScope::new();
    let actual = Evaluator::new(&outer_let, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}

#[test]
fn shadowing_in_closure_works() {
    // let x = 100 in
    //  let func = fn x: x + 5 in
    //   (func 1)
    
    let fncall = Ast::Call(
        Box::new(Ast::Id(Id(0, "func".to_string()))),
        Box::new(Ast::Val(Val::Int(1))));
    let func = Ast::Fn("x".to_string(), 
                       Box::new(Ast::BinOp(
                               BinOp::Add,
                               Box::new(Ast::Id(Id(0, "x".to_string()))),
                               Box::new(Ast::Val(Val::Int(5))))));
    let inner_let = Ast::Let(
        Box::new(SetField{name: "func".to_string(), value: func}),
        Box::new(fncall));


    let outer_let = Ast::Let(
        Box::new(SetField{
            name: "x".to_string(),
            value: Ast::Val(Val::Int(100))
        }),
        Box::new(inner_let));


    let expected = Ok(Output::Int(6));
    let scope = OpenScope::new();
    let actual = Evaluator::new(&outer_let, Scope::Open(&scope)).eval();
    assert_eq!(expected, actual);
}

#[test]
fn import_works() {
    use tempdir::TempDir;
    use std::io::{Write};
    use std::fs::{File, create_dir_all};

    let tmpdir = TempDir::new("test").unwrap();
    create_dir_all(tmpdir.path()).unwrap();

    let contents = "+ 1 2".as_bytes();
    let import_file = tmpdir.path().join("expr.foil");
    {
        let mut f = File::create(&import_file).unwrap();
        f.write_all(contents).unwrap();
        f.sync_all().unwrap();
    }

    let input = Ast::Import(0, import_file.to_str().unwrap().to_string());
    let expected = Ok(Output::Int(3));
    let scope = OpenScope::new();
    let actual = Evaluator::with_file(&input, Scope::Open(&scope), import_file.to_path_buf()).eval();
    assert_eq!(expected, actual);
}
