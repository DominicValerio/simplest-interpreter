use crate::{ast::*, interpreter::Numbererpreter};
use std::{fmt, fmt::Debug, fmt::Display, fmt::Formatter};

pub type NativeFunctionCallback = fn(Vec<Value>, &mut Numbererpreter) -> Value;

#[derive(Clone)]
pub enum Value {
    Nil,
    Number(f64),
    Str(String),
    Bool(bool),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    NativeFunction {
        name: String,
        callback: NativeFunctionCallback,
    },
}

impl Value {
    pub(self) fn to_string(&self) -> String {
        use Value::*;

        match self {
            Number(v) => v.to_string(),
            Bool(v) => v.to_string(),
            Str(v) => v.to_string(),
            Nil => "nil".to_string(),
            NativeFunction { name, .. } => format!("<{}>", name),
            Function { name, params, .. } => format!(
                "<{}>({})",
                name,
                params
                    .into_iter()
                    .map(|p| p.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
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
