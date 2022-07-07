use crate::{ast::*, interpreter::Interpreter, object::Object};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
pub struct Environment {
    current: HashMap<String, Object>,
    outer: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Environment { current: HashMap::new(), outer: None }
    }

    pub fn from(map: HashMap<String, Object>) -> Self {
        Environment { current: map, outer: None }
    }

    pub fn get(&self, k: &String) -> Option<&Object> {
        if let Some(v) = self.current.get(k) {
            return Some(v);
        }
        if let Some(outer) = &self.outer {
            return outer.get(k);
        }
        return None;
    }

    pub fn contains(&self, k: &String) -> bool {
        if self.current.contains_key(k) {
            return true;
        } else if let Some(outer) = &self.outer {
            return outer.contains(k);
        }
        false
    }

    pub fn insert(&mut self, k: String, v: Object) {
        self.current.insert(k, v);
    }
}


//use std::{fmt, fmt::Debug, fmt::Display, fmt::Formatter};

// #[derive(Clone)]
// pub enum Value {
    
//     Function(Box<FunctionDef>),
//     NativeFunction(Box<NativeFunctionDef>),
// }

// impl Value {
//     pub(self) fn to_string(&self) -> String {
//         use Value::*;

//         match self {
//             Number(v) => format!("{}", v),
//             Bool(v) => v.to_string(),
//             Str(v) => v.to_string(),
//             Nil => "nil".to_string(),
//             NativeFunction(f) => format!("<{}>", f.name),
//             Function(f) => format!(
//                 "<{}>({})",
//                 f.name,
//                 f.params
//                     .clone()
//                     .into_iter()
//                     .map(|p| p.clone())
//                     .collect::<Vec<String>>()
//                     .join(", ")
//             ),
//         }
//     }
// }

// impl Debug for Value {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

// impl Display for Value {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }
