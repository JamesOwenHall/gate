use std::collections::HashMap;

use super::ast::{Data, Expression, Result};

pub struct Program {
    pub scopes: ScopeTree,
}

impl Program {
    pub fn new() -> Self {
        Program {
            scopes: ScopeTree::new(),
        }
    }

    pub fn eval(&mut self, e: &Expression) -> Result {
        e.eval(self)
    }

    pub fn var(&self, name: &str) -> Option<Data> {
        self.scopes.var(name)
    }

    pub fn set_var(&mut self, name: &str, val: Data) {
        self.scopes.set_var(name, val)
    }

    pub fn new_scope(&mut self) {
        self.scopes.frames.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.frames.pop();
    }
}

#[derive(Debug)]
pub struct ScopeTree {
    frames: Vec<Scope>,
}

impl ScopeTree {
    fn new() -> Self {
        ScopeTree{frames: vec![Scope::new()]}
    }

    fn var(&self, name: &str) -> Option<Data> {
        for frame in self.frames.iter().rev() {
            let var = frame.vars.get(name);
            if var.is_some() {
                return var.cloned();
            }
        }

        None
    }

    fn set_var(&mut self, name: &str, val: Data) {
        for frame in self.frames.iter_mut().rev() {
            match frame.vars.get_mut(name) {
                Some(v) => {
                    *v = val;
                    return;
                },
                None => {},
            }
        }

        self.frames.last_mut().unwrap().vars.insert(String::from(name), val);
    }
}

#[derive(Debug)]
struct Scope {
    vars: HashMap<String, Data>,
}

impl Scope {
    fn new() -> Self {
        Scope{vars: HashMap::new()}
    }
}
