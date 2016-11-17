use binary_op::BinaryOp;
use expr::Expression;

use parser::*;

#[test]
fn test_literal() {
    let mut parser = Parser::new(r#"nil true false 1 "foo""#);
    assert_eq!(parser.next(), Some(Ok(Expression::NilLiteral)));
    assert_eq!(parser.next(), Some(Ok(Expression::BooleanLiteral(true))));
    assert_eq!(parser.next(), Some(Ok(Expression::BooleanLiteral(false))));
    assert_eq!(parser.next(), Some(Ok(Expression::NumberLiteral(1.0))));
    assert_eq!(parser.next(),
               Some(Ok(Expression::StrLiteral("foo".to_owned()))));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_parenthesis() {
    let mut parser = Parser::new(r#"(nil)(((true)))"#);
    assert_eq!(parser.next(),
               Some(Ok(Expression::ParenExpr(Box::new(Expression::NilLiteral)))));
    assert_eq!(parser.next(), Some(Ok(Expression::ParenExpr(
        Box::new(Expression::ParenExpr(
            Box::new(Expression::ParenExpr(
                Box::new(Expression::BooleanLiteral(true)),
            )),
        )),
    ))));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_identifier_and_function_call() {
    let foo_var = Expression::Variable("foo".to_owned());

    let mut parser = Parser::new(r#"foo foo() foo(foo) foo(foo, foo)"#);
    assert_eq!(parser.next(), Some(Ok(foo_var.clone())));
    assert_eq!(parser.next(),
               Some(Ok(Expression::FunctionCall {
                   name: "foo".to_owned(),
                   args: vec![],
               })));
    assert_eq!(parser.next(),
               Some(Ok(Expression::FunctionCall {
                   name: "foo".to_owned(),
                   args: vec![foo_var.clone()],
               })));
    assert_eq!(parser.next(),
               Some(Ok(Expression::FunctionCall {
                   name: "foo".to_owned(),
                   args: vec![foo_var.clone(), foo_var.clone()],
               })));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_binary_expr() {
    let mut parser = Parser::new(r#"1 + 2 - 3 * 4 / 5"#);

    assert_eq!(parser.next(),
               Some(Ok(Expression::BinaryExpr {
                   left: Box::new(Expression::NumberLiteral(1.0)),
                   op: BinaryOp::Add,
                   right: Box::new(Expression::BinaryExpr {
                       left: Box::new(Expression::NumberLiteral(2.0)),
                       op: BinaryOp::Sub,
                       right: Box::new(Expression::BinaryExpr {
                           left: Box::new(Expression::NumberLiteral(3.0)),
                           op: BinaryOp::Mul,
                           right: Box::new(Expression::BinaryExpr {
                               left: Box::new(Expression::NumberLiteral(4.0)),
                               op: BinaryOp::Div,
                               right: Box::new(Expression::NumberLiteral(5.0)),
                           }),
                       }),
                   }),
               })));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_binary_op() {
    let cases = vec![
        ("+", BinaryOp::Add),
        ("-", BinaryOp::Sub),
        ("*", BinaryOp::Mul),
        ("/", BinaryOp::Div),
        ("==", BinaryOp::Eq),
        ("<", BinaryOp::Lt),
        ("<=", BinaryOp::LtEq),
        (">", BinaryOp::Gt),
        (">=", BinaryOp::GtEq),
    ];

    for (s, op) in cases {
        let expr_str = format!("1 {} 2", s);

        let mut parser = Parser::new(&expr_str);
        assert_eq!(parser.next(),
                   Some(Ok(Expression::BinaryExpr {
                       left: Box::new(Expression::NumberLiteral(1.0)),
                       op: op,
                       right: Box::new(Expression::NumberLiteral(2.0)),
                   })));
        assert_eq!(parser.next(), None);
    }
}

#[test]
fn test_block() {
    let mut parser = Parser::new("{1{}2}");

    assert_eq!(parser.next(),
               Some(Ok(Expression::Block(vec![
        Expression::NumberLiteral(1.0),
        Expression::Block(vec![]),
        Expression::NumberLiteral(2.0),
    ]))));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_assignment() {
    let mut parser = Parser::new("x = y = z");

    assert_eq!(parser.next(),
               Some(Ok(Expression::Assignment {
                   left: "x".to_owned(),
                   right: Box::new(Expression::Assignment {
                       left: "y".to_owned(),
                       right: Box::new(Expression::Variable("z".to_owned())),
                   }),
               })));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_if_expr() {
    let mut parser = Parser::new("if true {} else if false {}");

    assert_eq!(parser.next(),
               Some(Ok(Expression::IfExpr {
                   cond: Box::new(Expression::BooleanLiteral(true)),
                   body: Box::new(Expression::Block(vec![])),
                   else_branch: Some(Box::new(Expression::IfExpr {
                       cond: Box::new(Expression::BooleanLiteral(false)),
                       body: Box::new(Expression::Block(vec![])),
                       else_branch: None,
                   })),
               })));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_while_loop() {
    let mut parser = Parser::new("while true {}");

    assert_eq!(parser.next(),
               Some(Ok(Expression::WhileLoop {
                   cond: Box::new(Expression::BooleanLiteral(true)),
                   body: Box::new(Expression::Block(vec![])),
               })));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_precedence() {
    let mut parser = Parser::new("1 + 2 * 3  1 * 2 + 3");

    assert_eq!(parser.next(),
               Some(Ok(Expression::BinaryExpr {
                   left: Box::new(Expression::NumberLiteral(1.0)),
                   op: BinaryOp::Add,
                   right: Box::new(Expression::BinaryExpr {
                       left: Box::new(Expression::NumberLiteral(2.0)),
                       op: BinaryOp::Mul,
                       right: Box::new(Expression::NumberLiteral(3.0)),
                   }),
               })));
    assert_eq!(parser.next(),
               Some(Ok(Expression::BinaryExpr {
                   left: Box::new(Expression::BinaryExpr {
                       left: Box::new(Expression::NumberLiteral(1.0)),
                       op: BinaryOp::Mul,
                       right: Box::new(Expression::NumberLiteral(2.0)),
                   }),
                   op: BinaryOp::Add,
                   right: Box::new(Expression::NumberLiteral(3.0)),
               })));
    assert_eq!(parser.next(), None);
}
