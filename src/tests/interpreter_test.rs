use std::{cell::RefMut, mem, time::Instant};

use crate::{interpreter::*, lexer::*, parser::*};

#[test]
fn scope() {
    let src = r#"
    var x = 0
    var i = 0
    while x < 100 {
      x = x + 1
      var c = true
      c = false
      while c == false {
        c = true
        i = i + 1
      }
    }
    println(i)
  "#;
    let mut l = Lexer::new(src);
    let toks = l.parse();
    //dbg!(&toks);
    let mut p = Parser::new(l.parse().unwrap());
    let res = p.parse().unwrap();
    //dbg!((&res));
    let mut i = Interpreter::new(res);
    let instant = Instant::now();
    i.run().unwrap();
    //dbg!(&i.env);

    let time = instant.elapsed().as_secs_f64();
    assert_eq!(i.stdout, "100\n".to_string())
}

#[test]
fn blocks() {
    let src = r#"
    var x = 5
    {
      var x = 3
      {
        var y = 3
        print(x)
        x = 10
      }
      
    }
    print(x)
  "#;
    let mut l = Lexer::new(src);
    let toks = l.parse();
    //dbg!(&toks);
    let mut p = Parser::new(l.parse().unwrap());
    let res = p.parse().unwrap();
    //dbg!((&res));
    let mut i = Interpreter::new(res);
    let instant = Instant::now();
    i.run().unwrap();
    //dbg!(&i.env);

    let time = instant.elapsed().as_secs_f64();
}

#[test]
fn error_tests() {
    let src = r#"
    var x = 10
    println(x)

    fn add(x, y) {
      return x + y
    }

  while x < 10 {
    {
      x = add(x, 2)
    }
  }

  print("here")
  "#;
    let mut l = Lexer::new(src);
    let toks = l.parse();
    //dbg!(&toks);
    let mut p = Parser::new(l.parse().unwrap());
    let res = p.parse().unwrap();

    //dbg!((&res));
    let mut i = Interpreter::new(res);
    let instant = Instant::now();
    i.run().unwrap();
    //dbg!(&i.env);

    let time = instant.elapsed().as_secs_f64();
}
