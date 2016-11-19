use std::fmt;

use data::Data;
use data::Data::*;
use error::ExecuteError;
use expr::Result;

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
        match (self, left, right) {
            (&Add, &Number(l), &Number(r)) => Ok(Number(l + r)),
            (&Sub, &Number(l), &Number(r)) => Ok(Number(l - r)),
            (&Mul, &Number(l), &Number(r)) => Ok(Number(l * r)),
            (&Div, &Number(l), &Number(r)) => Ok(Number(l / r)),
            (&Mod, &Number(l), &Number(r)) => Ok(Number(l % r)),
            (&Eq, _, _) => Ok(Boolean(left == right)),
            (&Lt, &Number(l), &Number(r)) => Ok(Boolean(l < r)),
            (&LtEq, &Number(l), &Number(r)) => Ok(Boolean(l <= r)),
            (&Gt, &Number(l), &Number(r)) => Ok(Boolean(l > r)),
            (&GtEq, &Number(l), &Number(r)) => Ok(Boolean(l >= r)),
            (o, l, r) => {
                Err(ExecuteError::InvalidOperation {
                    left: l.type_name(),
                    op: o.clone(),
                    right: r.type_name(),
                })
            }
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

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Add => write!(f, "+"),
            &Sub => write!(f, "-"),
            &Mul => write!(f, "*"),
            &Div => write!(f, "/"),
            &Mod => write!(f, "%"),
            &Eq => write!(f, "=="),
            &Lt => write!(f, "<"),
            &LtEq => write!(f, "<="),
            &Gt => write!(f, ">"),
            &GtEq => write!(f, ">="),
        }
    }
}

#[cfg(test)]
mod tests {
    use data::Data::*;
    use error::ExecuteError::*;
    use super::BinaryOp::*;

    #[test]
    fn test_binary_expr() {
        let cases = vec![
            // Arithmetic
            (Add, Number(1.0), Number(2.0), Number(3.0)),
            (Sub, Number(1.0), Number(2.0), Number(-1.0)),
            (Mul, Number(1.5), Number(2.0), Number(3.0)),
            (Div, Number(1.0), Number(2.0), Number(0.5)),
            (Mod, Number(17.0), Number(4.0), Number(1.0)),
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

        // Invalid operation
        assert_eq!(Add.eval(&Number(1.0), &Boolean(false)),
                   Err(InvalidOperation {
                       left: "number".to_owned(),
                       op: Add,
                       right: "boolean".to_owned(),
                   }));
    }
}
