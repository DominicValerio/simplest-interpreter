//#![allow(warnings)] 
pub (crate) mod ast;
pub (crate) mod stdlib;
pub (crate) mod token;
pub (crate) mod environment;

mod tests;

pub mod lexer;
pub mod parser;
pub mod interpreter;