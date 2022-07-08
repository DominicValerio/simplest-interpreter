//#![allow(warnings)]
pub(crate) mod ast;
pub(crate) mod environment;
pub(crate) mod object;
pub(crate) mod stdlib;
pub(crate) mod token;

mod tests;

pub mod interpreter;
pub mod lexer;
pub mod parser;
