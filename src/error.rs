use std::fmt;

use binary_op::BinaryOp;
use scanner::Token;

use self::ExecuteError::*;

#[derive(Clone,Debug,PartialEq)]
pub enum ExecuteError {
    UndefinedVar(String),
    UndefinedFunc(String),
    InvalidOperation {
        left: String,
        op: BinaryOp,
        right: String,
    },
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &UndefinedVar(ref s) => write!(f, "undefined variable \"{}\"", s),
            &UndefinedFunc(ref s) => write!(f, "undefined function \"{}\"", s),
            &InvalidOperation { ref left, ref op, ref right } => {
                write!(f, "invalid operation ({} {} {})", left, op, right)
            }
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum ParseError {
    ScanError(TokenError),
    Unexpected(Token),
    UnexpectedEOF,
}

#[derive(Clone,Debug,PartialEq)]
pub enum TokenError {
    UnexpectedChar(char),
    IncompleteString,
    InvalidEscape,
}
