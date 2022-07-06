use std::{fmt::Debug, fmt::Formatter, fmt::Display, fmt};
use crate::{interpreter::Interpreter, ast::*};

#[derive(Clone)]
pub enum Value {
  Nil,
  Int(i32),
  Str(String),
  Bool(bool),
  Function {
      name: String,
      params: Vec<String>,
      body: Vec<Statement>,
  },
  NativeFunction {
    name: String, 
    callback: fn (Vec<Value>, &mut Interpreter) -> Value,
  },
}

impl Value {
  pub (self) fn to_string(&self) -> String {
    use Value::*;

    match self {
      Int(v) => v.to_string(),
      Bool(v) => v.to_string(),
      Str(v) => v.to_string(),
      Nil => "nil".to_string(),
      NativeFunction{ name, .. } => format!("<{}>", name),
      Function { name, params, ..} => format!("<{}>({})", name, params.into_iter().map(|p| p.clone()).collect::<Vec<String>>().join(", ")),
    }
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}