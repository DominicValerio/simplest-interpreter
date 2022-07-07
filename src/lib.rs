//#![allow(warnings)]
pub(crate) mod ast;
pub(crate) mod environment;
pub(crate) mod stdlib;
pub(crate) mod token;
pub(crate) mod object;

mod tests;

pub mod interpreter;
pub mod lexer;
pub mod parser;
