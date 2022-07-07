use std::time::Instant;

use crate::{interpreter::*, lexer::*, parser::*};

#[test]
fn simple() {
    let src = r#"
  var x = 10.3 / 4
  println(x)
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
