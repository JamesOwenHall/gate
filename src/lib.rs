use std::collections::HashMap;

use Data::*;
use Expression::*;

#[derive(Clone,Debug)]
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
}
