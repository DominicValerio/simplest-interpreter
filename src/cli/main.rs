//! Command line interface. Takes argument of a filepath to source code.

use lib::interpreter;
use std::env::args;

fn main() -> Result<(), String> {
    const HELP: &str = "Arg 1\tProvide a file path to run\nArg 2\t--bench\tprints the time taken after running";

    let filepath = match args().nth(1) {
        Some(v) => v,
        None => return Err("No argument of <filepath/> provided to application".to_string()),
    };

    let source_text = match std::fs::read_to_string(filepath.clone()) {
        Ok(v) => v,
        Err(e) => {
            if filepath.eq("help") {
                println!("{HELP}");
                return Ok(());
            } else {
                return Err(e.to_string())
            }
        },
    };

    let now = std::time::Instant::now();

    interpreter::run_source(source_text.as_str())?;

    if args().nth(2) == Some(String::from("--bench")) {
        println!("\n{}s", now.elapsed().as_secs_f64());
    }

    Ok(())
}
