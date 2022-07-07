use std::{cell::RefMut, mem, time::Instant};

use crate::{environment::Value, interpreter::*, lexer::*, parser::*};

#[test]
fn simple() {
    let src = r#"
    var x = 10
    println(size_of(x))
    var y = true
    println(size_of(y))
    var z = nil
    println(size_of())
    var s = "bobbbbbbbbbbbbbbbbbbbb"
    println(size_of(s))
  "#;
    let l = Lexer::new(src);
    let toks = l.parse();
    //dbg!(&toks);
    let mut p = Parser::new(l.parse());
    let res = p.parse().unwrap();
    //dbg!((&res));
    let mut i = Interpreter::new(res);
    let instant = Instant::now();
    i.run().unwrap();
    println!("{}s", instant.elapsed().as_secs_f64());
}
