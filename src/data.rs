use std::fmt;

use Data::*;

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
