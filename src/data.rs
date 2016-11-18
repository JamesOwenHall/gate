use std::fmt;

use Data::*;

#[derive(Clone,Debug,PartialEq)]
pub enum Data {
    Nil,
    Boolean(bool),
    Number(f64),
    Str(String),
}

impl Data {
    pub fn to_bool(&self) -> bool {
        match self {
            &Nil | &Boolean(false) => false,
            _ => true,
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            &Nil => "nil".to_owned(),
            &Boolean(_) => "boolean".to_owned(),
            &Number(_) => "number".to_owned(),
            &Str(_) => "string".to_owned(),
        }
    }
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
