use crate::{ast::*, interpreter::Interpreter};
use std::{fmt, fmt::Debug, fmt::Display, fmt::Formatter};

pub type NativeFunctionCallback = fn(Vec<Value>, &mut Interpreter) -> Value;

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Str(String),
    Number(Box<f64>),
    Function(Box<FunctionDef>),
    NativeFunction(Box<NativeFunctionDef>),
}

impl Value {
    pub(self) fn to_string(&self) -> String {
        use Value::*;

        match self {
            Number(v) => format!("{}", v),
            Bool(v) => v.to_string(),
            Str(v) => v.to_string(),
            Nil => "nil".to_string(),
            NativeFunction(f) => format!("<{}>", f.name),
            Function(f) => format!(
                "<{}>({})",
                f.name,
                f.params
                    .clone()
                    .into_iter()
                    .map(|p| p.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
