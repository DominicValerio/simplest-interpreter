
use crate::{object::Object};
use std::{collections::HashMap, vec};

/// Used to simulate variable scope
#[derive(Debug, Clone)]
pub struct Environment {
    stack: Vec<HashMap<String, Object>>
}

impl Environment {
    pub fn from(globals: HashMap<String, Object>) -> Environment {
        Environment { stack: vec![globals] }
    }

    pub fn get(&self, k: &String) -> Option<&Object> {
        for curmap in &self.stack {
            if let Some(v) = curmap.get(k) { 
                return Some(v);
            }
        }
        return None;
    }

    pub fn contains(&self, k: &String) -> bool {
        for curmap in &self.stack {
            if curmap.contains_key(k) { 
                return true;
            }
        }
        dbg!(&self.stack);
        return false;
    }

    pub fn insert(&mut self, k: String, v: Object) {
        //find the outermost variable name, then assign that
        for i in 0..self.stack.len() {
            let dict = &self.stack[i];
            if dict.contains_key(&k) {
                self.stack[i].insert(k, v);
                return;
            }
        }
        // assign it in the current scope
        let len = self.stack.len();
        self.stack[len - 1].insert(k, v);
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop().expect("Environment tried to pop an empty stack");
    }
}