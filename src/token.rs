use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    EOF,
    Whitespace,
    Identifier,
    Comment,

    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    NilLiteral,
    TrueLiteral,
    FalseLiteral,
    
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
}

#[inline]
fn keywords() -> HashMap<String, TokenKind> {
    use TokenKind::*;
    [
        ("var", Var),
        ("fn", Fn),
        ("while", While),
        ("nil", NilLiteral),
        ("true", TrueLiteral),
        ("false", FalseLiteral),
        ("return", Return),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

impl TokenKind {
    pub fn from_char(c: char) -> TokenKind {
        use TokenKind::*;

        match c {
            '+' => Plus,
            '-' => Minus,
            '/' => Slash,
            '*' => Mul,
            '=' => Assign,
            '(' => Lparen,
            ')' => Rparen,
            '<' => LessThan,
            '>' => GreaterThan,
            ';' => Semicolon,
            ',' => Comma,
            '}' => Rbrace,
            '{' => Lbrace,
            '!' => Bang,
            '#' => Comment,
            '.' => Dot,
            _ => {
                dbg!(c);
                unimplemented!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub ln: usize,
    pub col: usize,
}

pub struct TokenStream {
    value: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        Self { value: vec![] }
    }

    pub fn as_vec(&mut self) -> Vec<Token> {
        self.value.clone()
    }

    pub fn push(&mut self, tok: &mut Token) {
        use TokenKind::*;

        // match keywords
        if tok.kind == Identifier {
            if let Some(kind) = keywords().get(tok.text.as_str()) {
                tok.kind = kind.clone();
            }
        }

        if tok.kind != Whitespace {
            tok.col -= tok.text.chars().count();
            self.value.push(tok.clone());
            tok.col += tok.text.chars().count();
        }

        tok.clear();
    }
}

impl Token {
    pub fn new() -> Token {
        Token {
            kind: TokenKind::Whitespace,
            text: "".to_string(),
            ln: 1,
            col: 1,
        }
    }

    pub fn push_char(&mut self, ch: char) {
        self.text.push(ch);
        match ch {
            '\n' => {
                self.ln += 1;
                self.col = 1;
            }
            '\r' => self.col = 1,
            '\t' => self.col += 4,
            _ => self.col += 1,
        }
    }

    pub fn clear(&mut self) {
        self.kind = TokenKind::Whitespace;
        self.text.clear();
    }

    pub fn alt(self, text: &str, kind: TokenKind, ln: usize, col: usize) -> Token {
        Token {
            kind: kind,
            text: text.to_string(),
            ln: ln,
            col: col,
        }
    }
}
