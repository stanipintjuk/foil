use compiler::parser::*;
use compiler::parser::ast::*;
use compiler::tokenizer::tokens::*;

#[test]
fn parse_binary_op_test() {
    let input = vec![
        Ok(Token::BinOp(0, BinOp::Add)),
        Ok(Token::Val(0, Val::Int(3))),
        Ok(Token::Val(0, Val::Int(4))),
    ];

    let expected = vec![
        Ok(Ast::BinOp(
                BinOp::Add, 
                Box::new(Ast::Val(Val::Int(3))),
                Box::new(Ast::Val(Val::Int(4)))
                ))
    ];

    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn parse_nested_binary_op_test() {
    // "+ - 1 2 3"
    let input = vec![
        Ok(Token::BinOp(0, BinOp::Add)),
        Ok(Token::BinOp(0, BinOp::Sub)),
        Ok(Token::Val(0, Val::Int(1))),
        Ok(Token::Val(0, Val::Int(2))),
        Ok(Token::Val(0, Val::Int(3))),
    ];

    let expected = vec![
        Ok(Ast::BinOp(
                BinOp::Add,
                Box::new(Ast::BinOp(
                        BinOp::Sub,
                        Box::new(Ast::Val(Val::Int(1))),
                        Box::new(Ast::Val(Val::Int(2))))),
                        Box::new(Ast::Val(Val::Int(3)))))
    ];

    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn parse_nested_binary_op_second_order() {
    // + 1 - 2 3
    let input = vec![
        Ok(Token::BinOp(0, BinOp::Add)),
        Ok(Token::Val(0, Val::Int(1))),
        Ok(Token::BinOp(0, BinOp::Sub)),
        Ok(Token::Val(0, Val::Int(2))),
        Ok(Token::Val(0, Val::Int(3))),
    ];

    let expected = vec![
        Ok(Ast::BinOp(
                BinOp::Add,
                Box::new(Ast::Val(Val::Int(1))),
                Box::new(Ast::BinOp(
                        BinOp::Sub,
                        Box::new(Ast::Val(Val::Int(2))),
                        Box::new(Ast::Val(Val::Int(3)))))))
    ];

    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn test_set_construction() {
    /*
     * set { field1="value", field2=23}
     * */

    let input = vec![
        Ok(Token::Keyword(0, Keyword::Set)),
        Ok(Token::BlockL(0)),
        Ok(Token::Id(0, "field1".to_string())),
        Ok(Token::Assign(0)),
        Ok(Token::Val(0, Val::String("value".to_string()))),
        Ok(Token::Comma(0)),
        Ok(Token::Id(0, "field2".to_string())),
        Ok(Token::Assign(0)),
        Ok(Token::Val(0, Val::Int(23))),
        Ok(Token::BlockR(0))
    ];

    let expected = vec![
        Ok(Ast::Set( vec![ 
                     SetField { 
                         name: "field1".to_string(),
                         value: Ast::Val(Val::String("value".to_string()))
                     },
                     SetField { 
                         name: "field2".to_string(),
                         value: Ast::Val(Val::Int(23))
                     },
        ]))
    ];

    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn parse_let_test() {
    /*
     * Test this expression
     * let x = 2 in + x 1
     * */

    let input = vec![
        Ok(Token::Keyword(0, Keyword::Let)),
        Ok(Token::Id(0, "x".to_string())),
        Ok(Token::Assign(0)),
        Ok(Token::Val(0, Val::Int(2))),
        Ok(Token::Keyword(0, Keyword::In)),
        Ok(Token::BinOp(0, BinOp::Add)),
        Ok(Token::Id(0, "x".to_string())),
        Ok(Token::Val(0, Val::Int(1))),
    ];

    let expected = vec![
        Ok(Ast::Let(
                Box::new(
                    SetField {
                        name: "x".to_string(),
                        value: Ast::Val(Val::Int(2))
                    }),
                    Box::new(Ast::BinOp(
                            BinOp::Add,
                            Box::new(Ast::Id(Id(0, "x".to_string()))),
                            Box::new(Ast::Val(Val::Int(1)))))))
    ];

    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn parse_import_should_work_with_string() {
    // import "path/to/file"
    let input = vec![
        Ok(Token::Keyword(0, Keyword::Import)),
        Ok(Token::Val(2, Val::String("path/to/file".to_string()))),
    ];

    let expected = vec![
        Ok(Ast::Import(2, "path/to/file".to_string())),
    ];
    
    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn parse_import_should_not_work_with_path() {
    // import <path/to/file>
    let input = vec![
        Ok(Token::Keyword(0, Keyword::Import)),
        Ok(Token::Val(2, Val::Path("path/to/file".to_string()))),
    ];

    let expected = vec![
        Err(ParseError::ExpectedString(
                Token::Val(2, Val::Path("path/to/file".to_string()))
                )),
    ];
    
    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn parse_fn_test() {
    // fn x: + x 1
    let input = vec![
        Ok(Token::Keyword(0, Keyword::Fn)),
        Ok(Token::Id(3, "x".to_string())),
        Ok(Token::Colon(4)),
        Ok(Token::BinOp(6, BinOp::Add)),
        Ok(Token::Id(8, "x".to_string())),
        Ok(Token::Val(10, Val::Int(1)))
    ];

    let expected = vec![
        Ok(Ast::Fn("x".to_string(), 
                   Box::new(Ast::BinOp(BinOp::Add,
                                       Box::new(Ast::Id(Id(8, "x".to_string()))),
                                       Box::new(Ast::Val(Val::Int(1)))))))
    ];

    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);
}

#[test]
fn parse_function_call() {
    // (myFunc "test")
    let input = vec![
        Ok(Token::GroupL(0)),
        Ok(Token::Id(1, "myFunc".to_string())),
        Ok(Token::Val(8, Val::String("test".to_string()))),
        Ok(Token::GroupR(14))
    ];

    let expected = vec![
        Ok(Ast::Call(
                Box::new(Ast::Id(Id(1, "myFunc".to_string()))),
                Box::new(Ast::Val(Val::String("test".to_string())))
                )
            )
    ];

    let mut iter = input.iter().map(Clone::clone);
    let actual: Vec<_> = Parser::new(&mut iter).collect();
    assert_eq!(expected, actual);

}
