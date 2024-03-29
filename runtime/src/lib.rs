#![allow(warnings)]
pub(crate) mod tests;

pub(crate) mod ast;
pub(crate) mod context;
pub(crate) mod object;
pub(crate) mod stdlib;
pub(crate) mod token;

pub mod interpreter;
pub mod lexer;
pub mod parser;
