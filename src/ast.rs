use std::collections::HashMap;
use std::fmt;

use BinaryOp::*;
use Data::*;
use Expression::*;

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
    pub fn eval(&self, p: &mut Program) -> Data {
        match self {
            &NilLiteral => Nil,
            &BooleanLiteral(b) => Boolean(b),
            &NumberLiteral(n) => Number(n),
            &StrLiteral(ref s) => Str(s.clone()),
            &Variable(ref name) => {
                match p.vars.get(name) {
                    Some(d) => d.clone(),
                    None => Nil,
                }
            },
            &ParenExpr(ref expr) => expr.eval(p),
            &Block(ref exprs) => {
                let mut last_result = Data::Nil;
                for expr in exprs {
                    last_result = expr.eval(p);
                }
                last_result
            },
            &Assignment{ref left, ref right} => {
                let res = right.eval(p);
                p.vars.insert(left.clone(), res.clone());
                res
            },
            &FunctionCall{ref name, ref args} => {
                let f = match name.as_ref() {
                    "println" => println,
                    _ => return Nil,
                };

                let mut new_args = Vec::new();
                for item in args.iter() {
                    new_args.push(item.eval(p));
                }

                f(&new_args)
            },
            &BinaryExpr{ref left, ref op, ref right} => {
                let (left_data, right_data) = (left.eval(p), right.eval(p));
                op.eval(&left_data, &right_data)
            },
            &IfExpr{ref cond, ref body, ref else_branch} => {
                if cond.eval(p) != Boolean(false) {
                    body.eval(p)
                } else if let &Some(ref b) = else_branch {
                    b.eval(p)
                } else {
                    Nil
                }
            },
            &WhileLoop{ref cond, ref body} => {
                let mut last_data = Nil;
                loop {
                    if let Boolean(false) = cond.eval(p) {
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
    fn eval(&self, left: &Data, right: &Data) -> Data {
        match (self, left, right) {
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
        }
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

pub struct Program {
    pub vars: HashMap<String, Data>,
    pub funcs: HashMap<String, i64>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn eval(&mut self, e: &Expression) -> Data {
        e.eval(self)
    }
}

pub fn println(v: &Vec<Data>) -> Data {
    for item in v {
        print!("{}", item);
    }
    println!("");
    Data::Nil
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::BinaryOp::*;
    use super::Data::*;
    use super::Expression::*;

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
            p.eval(&exp);
        }

        assert_eq!(p.vars.get("w"), None);
        assert_eq!(p.vars.get("x"), Some(&Number(2.0)));
        assert_eq!(p.vars.get("y"), Some(&Number(3.0)));
        assert_eq!(p.vars.get("z"), Some(&Nil));
    }

    #[test]
    fn test_paren_expr() {
        let expr = Expression::ParenExpr(
            Box::new(Expression::BooleanLiteral(true)),
        );

        let mut p = Program::new();
        assert_eq!(expr.eval(&mut p), Data::Boolean(true));
    }

    #[test]
    fn test_block() {
        let block = Expression::Block(vec![
            Expression::NumberLiteral(1.0),
            Expression::NumberLiteral(2.0),
            Expression::NumberLiteral(3.0),
        ]);

        let mut p = Program::new();
        assert_eq!(block.eval(&mut p), Data::Number(3.0));
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
            assert_eq!(op.eval(&left, &right), exp);
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

            assert_eq!(x.eval(&mut p), exp);
        }
    }

    #[test]
    fn test_while_loop() {
        let mut p = Program::new();
        p.eval(&Assignment{left: "x".to_owned(), right: Box::new(NumberLiteral(0.0))});
        
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
        });

        assert_eq!(out, Number(5.0));
        assert_eq!(p.eval(&Variable("x".to_owned())), Number(5.0));
    }
}
