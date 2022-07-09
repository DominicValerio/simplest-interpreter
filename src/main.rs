//! Command line interface. Takes argument of a filepath to source code.

//#![allow(warnings)]
use lib::{interpreter::{*, self}, lexer::*, parser::*};
use std::env::args;

fn main() -> Result<(), String> {
    const help: &str = "Arg 1\tProvide a file path to run\nArg 2\t--bench\tprints the time taken after running";
    let filepath = match args().nth(1) {
        Some(v) => v,
        None => return Err("No argument of <filepath/> provided to application".to_string()),
    };

    let text = match std::fs::read_to_string(filepath.clone()) {
        Ok(v) => v.to_string(),
        Err(e) => {
            if filepath.eq("help") {
                println!("{help}");
                return Ok(());
            } else {
                return Err(e.to_string())
            }
        },
    };

    let now = std::time::Instant::now();

    interpreter::run_source(text.as_str())?;

    if args().nth(2) == Some(String::from("--bench")) {
        println!("\n{}s", now.elapsed().as_secs_f64());
    }

    Ok(())
}
