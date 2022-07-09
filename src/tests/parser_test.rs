use crate::{interpreter::*, lexer::*, parser::*};


#[test]
fn prefix() {
  let src = r"
  var x = -1
  ";

  let toks = Lexer::new(src).parse().unwrap();
  let ast = Parser::new(toks).parse().unwrap();
}