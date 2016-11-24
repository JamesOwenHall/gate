use std::collections::HashMap;

use data::Data;

#[derive(Debug)]
pub struct Scope {
    vars: HashMap<String, Data>,
}

impl Scope {
    pub fn new() -> Self {
        Scope { vars: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct ScopeTree {
    pub frames: Vec<Scope>,
}

impl ScopeTree {
    pub fn new() -> Self {
        ScopeTree { frames: vec![Scope::new()] }
    }

    pub fn var(&self, name: &str) -> Option<Data> {
        for frame in self.frames.iter().rev() {
            let var = frame.vars.get(name);
            if var.is_some() {
                return var.cloned();
            }
        }

        None
    }

    pub fn set_var(&mut self, name: &str, val: Data) {
        for frame in self.frames.iter_mut().rev() {
            match frame.vars.get_mut(name) {
                Some(v) => {
                    *v = val;
                    return;
                }
                None => {}
            }
        }

        self.frames.last_mut().unwrap().vars.insert(String::from(name), val);
    }
}
