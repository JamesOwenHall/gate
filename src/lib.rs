use std::collections::HashMap;

pub trait Expression {
    fn eval(&self, p: &mut Program) -> Data;
    fn clone_expr(&self) -> Box<Expression>;
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

#[derive(Clone,Debug)]
pub enum Data {
    Nil,
    Boolean(bool),
    Number(f64),
    Str(String),
}

impl Expression for Data {
    fn eval(&self, _: &mut Program) -> Data {
        self.clone()
    }

    fn clone_expr(&self) -> Box<Expression> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
}

impl Expression for Variable {
    fn eval(&self, p: &mut Program) -> Data {
        match p.vars.get(&self.name) {
            Some(d) => d.clone(),
            None => Data::Nil,
        }
    }

    fn clone_expr(&self) -> Box<Expression> {
        Box::new(self.clone())
    }
}

pub struct Assignment {
    pub left: Variable,
    pub right: Box<Expression>,
}

impl Expression for Assignment {
    fn eval(&self, p: &mut Program) -> Data {
        let res = self.right.eval(p);
        p.vars.insert(self.left.name.clone(), res.clone());
        res
    }

    fn clone_expr(&self) -> Box<Expression> {
        Box::new(Assignment {
            left: self.left.clone(),
            right: self.right.clone_expr(),
        })
    }
}

pub fn println(v: &Vec<Data>) -> Data {
    for item in v {
        print!("{:?}", item);
    }
    println!("");
    Data::Nil
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Box<Expression>>,
}

impl Expression for FunctionCall {
    fn eval(&self, p: &mut Program) -> Data {
        let f = match self.name.as_ref() {
            "println" => println,
            _ => return Data::Nil,
        };

        let mut args = Vec::new();
        for item in self.args.iter() {
            args.push(item.eval(p));
        }

        f(&args)
    }

    fn clone_expr(&self) -> Box<Expression> {
        let mut args = Vec::new();
        for item in self.args.iter() {
            args.push(item.clone_expr());
        }

        Box::new(FunctionCall {
            name: self.name.clone(),
            args: args,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Data::*;

    #[test]
    fn test_things() {
        let mut ast: Vec<Box<Expression>> = Vec::new();
        ast.push(Box::new(Assignment {
            left: Variable{name: "x".to_owned()},
            right: Box::new(Number(2.0)),
        }));
        ast.push(Box::new(FunctionCall {
            name: "println".to_owned(),
            args: vec![Box::new(Variable{name: "x".to_owned()})]
        }));

        let mut p = Program::new();
        for e in ast {
            p.eval(e.as_ref());
        }
    }
}
