use crate::{ast::*, interpreter::Interpreter, object::Object};
use std::{collections::HashMap};

#[derive(Debug, Clone)]
pub struct Environment {
    pub current: HashMap<String, Object>,
    pub outer: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Environment { current: HashMap::new(), outer: None}
    }

    pub fn from(map: HashMap<String, Object>) -> Self {
        Environment { current: map, outer: None }
    }

    pub fn get(&self, k: &String) -> Option<&Object> {
        if let Some(v) = self.current.get(k) {
            return Some(v);
        } else if let Some(_outer) = &self.outer {
            return _outer.get(k);
        } else {
            return None;
        }
    }

    pub fn contains(&self, k: &String) -> bool {
        if self.current.contains_key(k) {
            return true;
        } else if let Some(_outer) = &self.outer {
            return _outer.contains(k);
        } else {
            return false;
        }
    }

    pub fn insert(&mut self, k: String, v: Object) {
        //find the outermost variable name, then assign that

        if self.current.contains_key(&k) {
            self.current.insert(k, v);
        } else if let Some(_outer) = &self.outer {
            if _outer.current.contains_key(&k) {
                let mut clone = _outer.current.clone();
                clone.insert(k, v);
                self.outer = Some(Box::new(Environment::from(clone)));
            } else {
                self.current.insert(k, v);
            }
        } else {
            self.current.insert(k, v);
        }
        //self.current.insert(k, v);
    }

    pub fn encase(&mut self) {
        let temp = self.current.clone();
        self.current = HashMap::new();
        self.outer = Some(Box::new(Environment::from(temp)));
    }

    pub fn uncover(&mut self) {
        self.current = self.outer.as_ref().unwrap().current.clone();
        match &self.outer {
            Some(outer) => {
                self.outer = Some(outer.clone())
            }
            None => self.outer = None,
        }        
    }
}


