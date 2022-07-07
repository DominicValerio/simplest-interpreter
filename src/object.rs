use crate::{ast::Statement, interpreter::Interpreter};
use std::{fmt, fmt::Debug, fmt::Display, fmt::Formatter};

pub type NativeFunctionCallback = fn(Vec<Object>, &mut Interpreter) -> Object;

#[derive(Clone, Debug)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}
#[derive(Clone)]
pub struct NativeFunctionDef {
    pub name: String,
    pub callback: NativeFunctionCallback,
}

impl Debug for NativeFunctionDef {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug)]
pub enum Object {
  Nil,
  Bool(bool),
  Str(String),
  Number(f64),
  Function(Box<FunctionDef>),
  NativeFunction(Box<NativeFunctionDef>),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      use Object::*;
      let res = match self {
        Bool(v) => v.to_string(),
        Number(v) => v.to_string(),
        Str(v) => v.clone(), 
        _ => unimplemented!(),
      };
      write!(f, "{}", res)
    }
}