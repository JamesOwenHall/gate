use ast::Result;
use data::Data;
use data::Data::*;

use BinaryOp::*;

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
    pub fn eval(&self, left: &Data, right: &Data) -> Result {
        let answer = match (self, left, right) {
            (&Add, &Number(l), &Number(r)) => Number(l + r),
            (&Sub, &Number(l), &Number(r)) => Number(l - r),
            (&Mul, &Number(l), &Number(r)) => Number(l * r),
            (&Div, &Number(l), &Number(r)) => Number(l / r),
            (&Mod, &Number(l), &Number(r)) => Number(l % r),
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
