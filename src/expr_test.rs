use binary_op::BinaryOp::*;
use data::Data::*;
use error::ExecuteError::*;
use program::*;

use expr::*;
use expr::Expression::*;

#[test]
fn test_variables() {
    let ast = vec![
        Assignment {
            left: "x".to_owned(),
            right: Box::new(NumberLiteral(2.0)),
        },
        Assignment {
            left: "y".to_owned(),
            right: Box::new(Variable("x".to_owned())),
        },
        Assignment {
            left: "z".to_owned(),
            right: Box::new(Variable("w".to_owned())),
        },
        Assignment {
            left: "y".to_owned(),
            right: Box::new(BinaryExpr {
                left: Box::new(Variable("y".to_owned())),
                op: Add,
                right: Box::new(NumberLiteral(1.0)),
            }),
        },
    ];

    let mut p = Program::new();
    for exp in ast {
        p.eval(&exp).ok();
    }

    assert_eq!(p.var("w"), None);
    assert_eq!(p.var("x"), Some(Number(2.0)));
    assert_eq!(p.var("y"), Some(Number(3.0)));
    assert_eq!(p.var("z"), None);
}

#[test]
fn test_undefined_var() {
    let ast = Variable("foo".to_owned());
    let mut p = Program::new();
    let res = ast.eval(&mut p);
    assert_eq!(Err(UndefinedVar("foo".to_owned())), res);
}

#[test]
fn test_undefined_func() {
    let ast = FunctionCall {
        name: "foo".to_owned(),
        args: vec![],
    };
    let mut p = Program::new();
    let res = ast.eval(&mut p);
    assert_eq!(Err(UndefinedFunc("foo".to_owned())), res);
}

#[test]
fn test_paren_expr() {
    let expr = Expression::ParenExpr(Box::new(Expression::BooleanLiteral(true)));

    let mut p = Program::new();
    assert_eq!(expr.eval(&mut p).unwrap(), Boolean(true));
}

#[test]
fn test_block() {
    let block = Expression::Block(vec![
        Expression::NumberLiteral(1.0),
        Expression::NumberLiteral(2.0),
        Expression::NumberLiteral(3.0),
    ]);

    let mut p = Program::new();
    assert_eq!(block.eval(&mut p).unwrap(), Number(3.0));
}

#[test]
fn test_block_scope() {
    let var = Expression::Variable("x".to_owned());

    let block = Expression::Block(vec![
        Expression::Assignment{
            left: "x".to_owned(),
            right: Box::new(Expression::NumberLiteral(1.0)),
        },
        Expression::Variable("x".to_owned()),
    ]);

    let assign = Expression::Assignment {
        left: "x".to_owned(),
        right: Box::new(Expression::BooleanLiteral(true)),
    };

    let mut p = Program::new();
    assert_eq!(Err(UndefinedVar("x".to_owned())), var.eval(&mut p));
    assert_eq!(Ok(Number(1.0)), block.eval(&mut p));
    assert_eq!(Err(UndefinedVar("x".to_owned())), var.eval(&mut p));
    assert_eq!(Ok(Boolean(true)), assign.eval(&mut p));
    assert_eq!(Ok(Boolean(true)), var.eval(&mut p));
}

#[test]
fn test_if_expr() {
    let mut p = Program::new();

    let cases = vec![
        (BooleanLiteral(true), NumberLiteral(1.0), None, Number(1.0)),
        (BooleanLiteral(true), NumberLiteral(1.0), Some(NumberLiteral(2.0)), Number(1.0)),
        (BooleanLiteral(false), NumberLiteral(1.0), None, Nil),
        (BooleanLiteral(false), NumberLiteral(1.0), Some(NumberLiteral(2.0)), Number(2.0)),
        (NilLiteral, NumberLiteral(1.0), Some(NumberLiteral(2.0)), Number(2.0)),
    ];

    for (cond, body, else_branch, exp) in cases {
        let x = IfExpr {
            cond: Box::new(cond),
            body: Box::new(body),
            else_branch: else_branch.map(|e| Box::new(e)),
        };

        assert_eq!(x.eval(&mut p).unwrap(), exp);
    }
}

#[test]
fn test_while_loop() {
    let mut p = Program::new();
    p.eval(&Assignment {
            left: "x".to_owned(),
            right: Box::new(NumberLiteral(0.0)),
        })
        .unwrap();

    let out = p.eval(&WhileLoop {
            cond: Box::new(BinaryExpr {
                left: Box::new(Variable("x".to_owned())),
                op: Lt,
                right: Box::new(NumberLiteral(5.0)),
            }),
            body: Box::new(Assignment {
                left: "x".to_owned(),
                right: Box::new(BinaryExpr {
                    left: Box::new(Variable("x".to_owned())),
                    op: Add,
                    right: Box::new(NumberLiteral(1.0)),
                }),
            }),
        })
        .unwrap();

    assert_eq!(out, Number(5.0));
    assert_eq!(p.eval(&Variable("x".to_owned())).unwrap(), Number(5.0));
}
