use data::Data;
use expr::{Expression, Result};
use scope::{Scope, ScopeTree};

pub struct Program {
    pub scopes: ScopeTree,
}

impl Program {
    pub fn new() -> Self {
        Program { scopes: ScopeTree::new() }
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
