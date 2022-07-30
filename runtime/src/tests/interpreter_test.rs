use std::{cell::RefMut, mem, time::Instant};

use crate::{interpreter::*, lexer::*, parser::*};

#[test]
fn function_scope() {
	let src = r#"
    var x = 1
    fn add(x, y) {
      x = x + y
      return x
    }
    var _ = add(x, 1)
    println(x)
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
	assert_eq!(i.stdout, "1\n".to_string())
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

#[test]
fn scope() {
	let src = r"
  var x = 1
  while x < 10 {
    x = x + 1
    while x < 4 {
      x = x + 11
    }
  }
  println(x)
";
	let mut l = Lexer::new(src);
	let toks = l.parse();
	//dbg!(&toks);
	let mut p = Parser::new(l.parse().unwrap());
	let res = p.parse().unwrap();

	//dbg!((&res));
	let mut i = Interpreter::new(res);
	let instant = Instant::now();
	i.run().unwrap();
	assert_eq!(i.stdout, "13\n".to_string())
}
