use std::{time::Instant, mem, cell::RefMut};

use crate::{interpreter::*, lexer::*, parser::*, environment::Value};

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

struct Function {
  name: String,
  params: Vec<String>,
  body: Vec<String>,
}

struct NativeFunction {
  name: String,
  callback: fn(i32) -> i32,
}
enum Test {
  Nil,
  Number(f64),
  Str(String),
  Bool(bool),
  Function(Function),
  NativeFunction(Box<NativeFunction>),
}

#[test]
fn mem() {
  use mem::*;

  let x = &999999999;

  dbg!(mem::size_of_val(&*x));

  dbg!(mem::align_of::<Test>());
  dbg!(mem::size_of::<Test>()); 
  let inner = Function {name: "sdf".to_string(), params: vec![], body: vec![]};
  dbg!(mem::size_of_val(&inner));
  let x  = Test::Function(inner);
  dbg!(mem::size_of_val(&x));
}