use std::time::Instant;

use crate::lexer::*;
use crate::token::*;
use crate::parser::*;
use crate::interpreter::*;
use crate::environment::*;

use TokenKind::*;
use crate::ast::*;

use Expression::*;

#[test]
fn simple() {
  let src = r#"print(10)"#;
  let l = Lexer::new(src);
  let toks = l.parse();
  //dbg!(&toks);
  let mut p = Parser::new(l.parse());
  let res = p.parse().unwrap();
  dbg!((&res));
  let mut i = Interpreter::new(res);
  i.run();
}