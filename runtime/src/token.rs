use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
	// Literals
	Identifier,
	Comment,
	String,
	Integer,
	Float,
	True,
	False,

	// Keywords
	Return,
	Var,
	Fn,
	While,

	//// All operators
	Assign,

	// Math Ops
	Plus,
	Minus,
	Slash,
	Star,

	// Boolean Ops
	Equals,
	NotEquals,
	LessThan,
	GreaterThan,
	LessEquals,
	GreaterEquals,
	Bang,

	// groupings
	Lparen,
	Rparen,
	Lbrace,
	Rbrace,

	// seperators
	Comma,
	Semicolon,
	Dot,

	EOF,
}

pub fn keywords() -> HashMap<String, TokenKind> {
	use TokenKind::*;
	[
		("var", Var),
		("fn", Fn),
		("while", While),
		("true", True),
		("false", False),
		("return", Return),
	]
	.into_iter()
	.map(|(k, v)| (k.to_string(), v))
	.collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub text: String,
	pub ln: usize,
	pub col: usize,
}
