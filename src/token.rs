use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    // Literals
    Identifier,
    Integer,
    Float,
    String,
    True,
    False,
    Comment,
    
    // Keywords
    Return,
    Var,
    Fn,
    While,

    //// All operators
    Assign,

    // math
    Plus,
    Minus,
    Mul,
    Slash,
    Star,

    // boolean
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

    // puncuation
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