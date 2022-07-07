use std::{cell::RefMut, mem, time::Instant};

use crate::{interpreter::*, lexer::*, parser::*};

#[test]
fn simple() {
    let src = r#"
    var x = 1
    var z = 1
    fn foo(x, y) {
      var y = 3
      println(x)
      println(z)
    }

    foo(3, 1)

    var x = 0
    while x < 100 {
    }
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
    //dbg!(&i.env);

    println!("{}s", instant.elapsed().as_secs_f64());
}
