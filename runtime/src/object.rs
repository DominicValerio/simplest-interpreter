//! Simulates objects that can be used as variables

use crate::{ast::Statement, interpreter::Interpreter};
use std::fmt::{self, Debug, Display, Formatter};

pub type NativeFunctionCallback = fn(&Vec<Object>, &mut Interpreter) -> Object;

#[derive(Clone, Debug)]
pub enum Object {
	Bool(bool),
	Str(String),
	Number(f64),
	Function(Box<FunctionDef>),
	NativeFunction(Box<NativeFunctionDef>),
	/// Type that's used to signify no value
	Unit,
}

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

impl Display for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use Object::*;
		let res = match self {
			Bool(v) => v.to_string(),
			Number(v) => v.to_string(),
			Str(v) => v.clone(),
			_ => todo!(),
		};
		write!(f, "{}", res)
	}
}
