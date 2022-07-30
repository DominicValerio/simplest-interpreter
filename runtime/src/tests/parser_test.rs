use crate::{interpreter::*, lexer::*, parser::*};

#[test]
fn prefix() {
	let src = r"
  var x = -10000
  println(x)
  while x < -5 {
    x = x +1
  }
  println(x)
  ";

	let toks = Lexer::new(src).parse().unwrap();
	let ast = Parser::new(toks).parse().unwrap();
	//dbg!(&ast);
	Interpreter::new(ast).run().unwrap();
}
