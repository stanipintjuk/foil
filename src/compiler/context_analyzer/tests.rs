#[test]
fn constant_has_correct_namespace_test() {
    // let x = 2 in + x 1
    let input = vec![
        Ok(Ast::Let(
                Box::new(
                    SetField {
                        name: "x",
                        value: Ast::Val(Val::Int(2))
                    }),
                    Box::new(Ast::BinOp(
                            BinOp::Add,
                            Box::new(Ast::Id(Id(0, "x"))),
                            Box::new(Ast::Val(Val::Int(1)))))))
    ];

    let mut expected_name_space = HashMap::new();
    expected_name_space.insert("x", Ast::Val(Val::Int(2)));

    let expected = vec![
        Ok(At::Namespace(
                expected_name_space,
                Box::new(Ast::BinOp(
                        BinOp::Add,
                        Box::new(Ast::Id(Id(0, "x"))),
                        Box::new(Ast::Val(Val::Int(1)))))))
    ];

}
