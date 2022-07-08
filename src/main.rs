#![allow(warnings)]
use lib::{interpreter::*, lexer::*, parser::*};
use std::{env::args, process::exit};

fn main() {
    let filepath = match args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("No argument of $filepath provided to binary file.");
            exit(1)
        }
    };

    let text = match std::fs::read_to_string(filepath.clone()) {
        Ok(v) => v,
        Err(v) => {
            eprintln!("{}", v);
            exit(1);
        }
    };

    let mut l = Lexer::new(&text);
    let mut p = Parser::new(l.parse());
    let res = p.parse();

    let ast;

    match res {
        Ok(v) => ast = v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
    let res = Interpreter::new(ast).run();

    if let Err(e) = res {
        eprintln!("{}", e);
    }
}
