use std::result;

use binary_op::BinaryOp;
use data::Data;
use data::Data::*;
use error::ExecuteError;
use error::ExecuteError::*;
use program::Program;

use Expression::*;

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
    Assignment {
        left: String,
        right: Box<Expression>,
    },
    FunctionCall { name: String, args: Vec<Expression> },
    BinaryExpr {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    IfExpr {
        cond: Box<Expression>,
        body: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
    WhileLoop {
        cond: Box<Expression>,
        body: Box<Expression>,
    },
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
            }
            &ParenExpr(ref expr) => expr.eval(p),
            &Block(ref exprs) => {
                let mut last_result = Ok(Data::Nil);

                p.new_scope();
                for expr in exprs {
                    last_result = expr.eval(p);
                }
                p.pop_scope();

                last_result
            }
            &Assignment { ref left, ref right } => {
                let res = try!(right.eval(p));
                p.set_var(left, res.clone());
                Ok(res)
            }
            &FunctionCall { ref name, ref args } => {
                let f = match name.as_ref() {
                    "println" => println,
                    _ => return Err(UndefinedFunc(name.clone())),
                };

                let mut new_args = Vec::new();
                for item in args.iter() {
                    new_args.push(try!(item.eval(p)));
                }

                f(&new_args)
            }
            &BinaryExpr { ref left, ref op, ref right } => {
                let (left_data, right_data) = (try!(left.eval(p)), try!(right.eval(p)));
                op.eval(&left_data, &right_data)
            }
            &IfExpr { ref cond, ref body, ref else_branch } => {
                if try!(cond.eval(p)) != Boolean(false) {
                    body.eval(p)
                } else if let &Some(ref b) = else_branch {
                    b.eval(p)
                } else {
                    Ok(Nil)
                }
            }
            &WhileLoop { ref cond, ref body } => {
                let mut last_data = Ok(Nil);
                loop {
                    if let Boolean(false) = try!(cond.eval(p)) {
                        break;
                    }

                    last_data = body.eval(p);
                }

                last_data
            }
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
