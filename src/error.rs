use std::fmt;

use scanner::Token;

use ExecuteError::*;

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
