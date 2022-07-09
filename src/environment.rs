//! The Environment is used to simulate variable scope. 
//! The implementation is stack based and follow similar rules to javascript.

use crate::object::Object;
use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
pub struct Environment {
    stack: Vec<HashMap<String, Object>>,
}

impl Environment {
    pub fn from(globals: HashMap<String, Object>) -> Environment {
        Environment {
            stack: vec![globals],
        }
    }
    /// returns the value starting from the innermost scope
    pub fn get(&self, k: &String) -> Option<&Object> {
        for curmap in self.stack.iter().rev() {
            if let Some(v) = curmap.get(k) {
                return Some(v);
            }
        }
        return None;
    }

    pub fn contains(&self, k: &String) -> Option<usize> {
        for (i, curmap) in self.stack.iter().rev().enumerate() {
            if curmap.contains_key(k) {
                return Some(self.stack.len() - 1 - i);
            }
        }
        return None;
    }

    pub fn insert_at(&mut self, k: String, v: Object, i: usize) {
        self.stack[i].insert(k, v);
    }


    /// sets the (key, value) pair, starting from the innermost scope
    pub fn insert(&mut self, k: String, v: Object) {
        // find the inner scoped variable name, then assign that
        for i in self.stack.len()..0 {
            let dict = &self.stack[i];
            if dict.contains_key(&k) {
                self.stack[i].insert(k, v);
                return;
            }
        }
        // if it doesn't exist, assign it in the current scope
        let len = self.stack.len();
        self.stack[len - 1].insert(k, v);
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.stack
            .pop()
            .expect("Environment tried to pop an empty stack");
    }
}
