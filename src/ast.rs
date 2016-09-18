use std::{fmt, result};

use BinaryOp::*;
use Data::*;
use ExecuteError::*;
use Expression::*;
use super::program::Program;

#[derive(Clone,Debug,PartialEq)]
pub enum Data {
    Nil,
    Boolean(bool),
    Number(f64),
    Str(String),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Nil => write!(f, "nil"),
            &Boolean(b) => write!(f, "{}", b),
            &Number(n) => write!(f, "{}", n),
            &Str(ref s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum ExecuteError {
    UndefinedVar(String),
    UndefinedFunc(String),
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &UndefinedVar(ref s) => write!(f, "undefined variable \"{}\"", s),
            &UndefinedFunc(ref s) => write!(f, "undefined function \"{}\"", s),
        }
    }
}

pub type Result = result::Result<Data, ExecuteError>;

#[derive(Clone,Debug,PartialEq)]
pub enum Expression {
    NilLiteral,
    BooleanLiteral(bool),
    NumberLiteral(f64),
    StrLiteral(String),
    Variable(String),
    ParenExpr(Box<Expression>),
    Block(Vec<Expression>),
    Assignment{left: String, right: Box<Expression>},
    FunctionCall{name: String, args: Vec<Expression>},
    BinaryExpr{left: Box<Expression>, op: BinaryOp, right: Box<Expression>},
    IfExpr{cond: Box<Expression>, body: Box<Expression>, else_branch: Option<Box<Expression>>},
    WhileLoop{cond: Box<Expression>, body: Box<Expression>},
}

impl Expression {
    pub fn eval(&self, p: &mut Program) -> Result {
        match self {
            &NilLiteral => Ok(Nil),
            &BooleanLiteral(b) => Ok(Boolean(b)),
            &NumberLiteral(n) => Ok(Number(n)),
            &StrLiteral(ref s) => Ok(Str(s.clone())),
            &Variable(ref name) => {
                match p.var(name) {
                    Some(d) => Ok(d.clone()),
                    None => Err(UndefinedVar(name.clone())),
                }
            },
            &ParenExpr(ref expr) => expr.eval(p),
            &Block(ref exprs) => {
                let mut last_result = Ok(Data::Nil);

                p.new_scope();
                for expr in exprs {
                    last_result = expr.eval(p);
                }
                p.pop_scope();

                last_result
            },
            &Assignment{ref left, ref right} => {
                let res = try!(right.eval(p));
                p.set_var(left, res.clone());
                Ok(res)
            },
            &FunctionCall{ref name, ref args} => {
                let f = match name.as_ref() {
                    "println" => println,
                    _ => return Err(UndefinedFunc(name.clone())),
                };

                let mut new_args = Vec::new();
                for item in args.iter() {
                    new_args.push(try!(item.eval(p)));
                }

                f(&new_args)
            },
            &BinaryExpr{ref left, ref op, ref right} => {
                let (left_data, right_data) = (try!(left.eval(p)), try!(right.eval(p)));
                op.eval(&left_data, &right_data)
            },
            &IfExpr{ref cond, ref body, ref else_branch} => {
                if try!(cond.eval(p)) != Boolean(false) {
                    body.eval(p)
                } else if let &Some(ref b) = else_branch {
                    b.eval(p)
                } else {
                    Ok(Nil)
                }
            },
            &WhileLoop{ref cond, ref body} => {
                let mut last_data = Ok(Nil);
                loop {
                    if let Boolean(false) = try!(cond.eval(p)) {
                        break
                    }

                    last_data = body.eval(p);
                }

                last_data
            },
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}

impl BinaryOp {
    fn eval(&self, left: &Data, right: &Data) -> Result {
        let answer = match (self, left, right) {
            (&Add, &Number(l), &Number(r)) => Number(l+r),
            (&Sub, &Number(l), &Number(r)) => Number(l-r),
            (&Mul, &Number(l), &Number(r)) => Number(l*r),
            (&Div, &Number(l), &Number(r)) => Number(l/r),
            (&Mod, &Number(l), &Number(r)) => Number(l%r),
            (&Eq, _, _) => Boolean(left == right),
            (&Lt, &Number(l), &Number(r)) => Boolean(l < r),
            (&LtEq, &Number(l), &Number(r)) => Boolean(l <= r),
            (&Gt, &Number(l), &Number(r)) => Boolean(l > r),
            (&GtEq, &Number(l), &Number(r)) => Boolean(l >= r),
            _ => Nil,
        };
        Ok(answer)
    }

    pub fn precendence(&self) -> u8 {
        match self {
            &Add => 3,
            &Sub => 3,
            &Mul => 4,
            &Div => 4,
            &Mod => 2,
            &Eq => 0,
            &Lt => 1,
            &LtEq => 1,
            &Gt => 1,
            &GtEq => 1,
        }
    }
}

pub fn println(v: &Vec<Data>) -> Result {
    for item in v {
        print!("{}", item);
    }
    println!("");
    Ok(Data::Nil)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::BinaryOp::*;
    use super::Data::*;
    use super::ExecuteError::*;
    use super::Expression::*;
    use super::super::program::*;

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
        let ast = FunctionCall{name: "foo".to_owned(), args: vec![]};
        let mut p = Program::new();
        let res = ast.eval(&mut p);
        assert_eq!(Err(UndefinedFunc("foo".to_owned())), res);
    }

    #[test]
    fn test_paren_expr() {
        let expr = Expression::ParenExpr(
            Box::new(Expression::BooleanLiteral(true)),
        );

        let mut p = Program::new();
        assert_eq!(expr.eval(&mut p).unwrap(), Data::Boolean(true));
    }

    #[test]
    fn test_block() {
        let block = Expression::Block(vec![
            Expression::NumberLiteral(1.0),
            Expression::NumberLiteral(2.0),
            Expression::NumberLiteral(3.0),
        ]);

        let mut p = Program::new();
        assert_eq!(block.eval(&mut p).unwrap(), Data::Number(3.0));
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

        let assign = Expression::Assignment{
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
    fn test_binary_expr() {
        let cases = vec![
            // Add
            (Add, Number(1.0), Number(2.0), Number(3.0)),
            (Add, Boolean(false), Number(2.0), Nil),
            (Add, Number(2.0), Str("1.0".to_owned()), Nil),
            // Sub
            (Sub, Number(1.0), Number(2.0), Number(-1.0)),
            (Sub, Boolean(false), Number(2.0), Nil),
            (Sub, Number(2.0), Str("1.0".to_owned()), Nil),
            // Mul
            (Mul, Number(1.5), Number(2.0), Number(3.0)),
            (Mul, Boolean(false), Number(2.0), Nil),
            (Mul, Number(2.0), Str("1.0".to_owned()), Nil),
            // Div
            (Div, Number(1.0), Number(2.0), Number(0.5)),
            (Div, Boolean(false), Number(2.0), Nil),
            (Div, Number(2.0), Str("1.0".to_owned()), Nil),
            // Mod
            (Mod, Number(17.0), Number(4.0), Number(1.0)),
            (Mod, Boolean(false), Number(2.0), Nil),
            (Mod, Number(2.0), Str("1.0".to_owned()), Nil),
            // Eq
            (Eq, Number(2.0), Number(2.0), Boolean(true)),
            (Eq, Number(-2.0), Number(2.0), Boolean(false)),
            (Eq, Str("foo".to_owned()), Str("foo".to_owned()), Boolean(true)),
            (Eq, Str("foo".to_owned()), Str("bar".to_owned()), Boolean(false)),
            (Eq, Boolean(false), Boolean(false), Boolean(true)),
            (Eq, Boolean(true), Boolean(true), Boolean(true)),
            (Eq, Boolean(true), Boolean(false), Boolean(false)),
            (Eq, Nil, Boolean(false), Boolean(false)),
            (Eq, Nil, Nil, Boolean(true)),
            // Lt
            (Lt, Number(-1.0), Number(0.5), Boolean(true)),
            (Lt, Number(1.0), Number(1.0), Boolean(false)),
            (Lt, Number(1.0), Number(0.5), Boolean(false)),
            // LtEq
            (LtEq, Number(-1.0), Number(0.5), Boolean(true)),
            (LtEq, Number(1.0), Number(1.0), Boolean(true)),
            (LtEq, Number(1.0), Number(0.5), Boolean(false)),
            // Gt
            (Gt, Number(-1.0), Number(0.5), Boolean(false)),
            (Gt, Number(1.0), Number(1.0), Boolean(false)),
            (Gt, Number(1.0), Number(0.5), Boolean(true)),
            // GtEq
            (GtEq, Number(-1.0), Number(0.5), Boolean(false)),
            (GtEq, Number(1.0), Number(1.0), Boolean(true)),
            (GtEq, Number(1.0), Number(0.5), Boolean(true)),
        ];

        for (op, left, right, exp) in cases {
            assert_eq!(op.eval(&left, &right).unwrap(), exp);
        }
    }

    #[test]
    fn test_if_expr() {
        let mut p = Program::new();

        let cases = vec![
            (BooleanLiteral(true), NumberLiteral(1.0), None, Number(1.0)),
            (BooleanLiteral(true), NumberLiteral(1.0), Some(NumberLiteral(2.0)), Number(1.0)),
            (BooleanLiteral(false), NumberLiteral(1.0), None, Nil),
            (BooleanLiteral(false), NumberLiteral(1.0), Some(NumberLiteral(2.0)), Number(2.0)),
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
        p.eval(&Assignment{left: "x".to_owned(), right: Box::new(NumberLiteral(0.0))}).unwrap();
        
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
        }).unwrap();

        assert_eq!(out, Number(5.0));
        assert_eq!(p.eval(&Variable("x".to_owned())).unwrap(), Number(5.0));
    }
}
