//! Command line interface. Takes argument of a filepath to source code.

#![allow(warnings)]
use lib::{interpreter::*, lexer::*, parser::*};
use std::{env::args, process::exit};

fn main() -> Result<(), String> {
    let filepath = match std::env::args().nth_back(0) {
        Some(v) => v,
        None => return Err("No argument of <filepath/> provided to application".to_string()),
    };

    let text = match std::fs::read_to_string(filepath.clone()) {
        Ok(v) => v.to_string(),
        Err(e) => return Err(e.to_string()),
    };


    let mut l = Lexer::new(text.as_str());
    let tokens =  l.parse()?;
    let abstract_syntax_tree = Parser::new(tokens).parse()?;
    Interpreter::new(abstract_syntax_tree).run()?;
    Ok(())
}
