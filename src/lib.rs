use std::collections::HashMap;

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

#[derive(Clone,Debug)]
pub enum Expression {
    NilLiteral,
    BooleanLiteral(bool),
    NumberLiteral(f64),
    StrLiteral(String),
    Variable(String),
    Assignment{left: String, right: Box<Expression>},
    FunctionCall{name: String, args: Vec<Expression>},
    BinaryExpr{left: Box<Expression>, op: BinaryOp, right: Box<Expression>},
    IfExpr{cond: Box<Expression>, body: Box<Expression>, else_branch: Option<Box<Expression>>},
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
                if cond.eval(p) == Boolean(true) {
                    body.eval(p)
                } else if let &Some(ref b) = else_branch {
                    b.eval(p)
                } else {
                    Nil
                }
            },
        }
    }
}

#[derive(Clone,Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
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
            (&Eq, _, _) => Boolean(left == right),
            (&Lt, &Number(l), &Number(r)) => Boolean(l < r),
            (&LtEq, &Number(l), &Number(r)) => Boolean(l <= r),
            (&Gt, &Number(l), &Number(r)) => Boolean(l > r),
            (&GtEq, &Number(l), &Number(r)) => Boolean(l >= r),
            _ => Nil,
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
        print!("{:?}", item);
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
    fn test_things() {
        let mut ast: Vec<Expression> = Vec::new();
        ast.push(Assignment {
            left: "x".to_owned(),
            right: Box::new(NumberLiteral(2.0)),
        });
        ast.push(FunctionCall {
            name: "println".to_owned(),
            args: vec![Variable("x".to_owned())]
        });

        let mut p = Program::new();
        for exp in ast {
            p.eval(&exp);
        }
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
}
