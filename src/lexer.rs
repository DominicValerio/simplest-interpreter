use std::vec;

use crate::token::*;
use TokenKind::*;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    startidx: usize,
    endidx: usize,
    col: usize,
    ln: usize,
}


impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            source: input.chars().collect(),
            tokens: vec![],
            col: 0,
            ln: 0,
            startidx: 0,
            endidx: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut curtok = Token::new();
        let mut list = TokenStream::new();

        while self.endidx < self.source.len() {
            let ch = self.source[self.endidx];

            match ch {
                // numbers
                '0'..='9' => match curtok.kind {
                    Whitespace => {
                        curtok.kind = Integer;
                        curtok.push_char(ch);
                    }
                    Dot => {
                        curtok.kind = Float;
                        curtok.text.push(ch);
                    }
                    _ => curtok.push_char(ch),
                },
                // single operators
                '+' | '-' | '*' | '{' | '}' | '(' | ')' | ',' | ';' => match curtok.kind {
                    String | Comment => curtok.push_char(ch),
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = TokenKind::from_char(ch);
                        curtok.push_char(ch);
                        list.push(&mut curtok);
                    }
                },
                // operators that can be combined. e.g <=
                '<' | '>' | '!' => match curtok.kind {
                    String | Comment => curtok.push_char(ch),
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = TokenKind::from_char(ch);
                        curtok.push_char(ch);
                    }
                },
                // Assignment
                '=' => match curtok.kind {
                    String | Comment => curtok.push_char(ch),
                    Assign => {
                        curtok.kind = Equals;
                        curtok.push_char(ch);
                        list.push(&mut curtok);
                    }
                    Bang => {
                        curtok.kind = NotEquals;
                        curtok.push_char(ch);
                        list.push(&mut curtok);
                    }
                    LessThan => {
                        curtok.kind = LessEquals;
                        curtok.push_char(ch);
                        list.push(&mut curtok);
                    }
                    GreaterThan => {
                        curtok.kind = GreaterEquals;
                        curtok.push_char(ch);
                        list.push(&mut curtok);
                    }
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = Assign;
                        curtok.push_char(ch);
                    }
                },
                // Division Operator
                '/' => {
                    match curtok.kind {
                        String | Comment => curtok.push_char(ch),
                        _ => {
                            list.push(&mut curtok);
                            curtok.kind = Slash;
                        }
                    }
                    curtok.push_char(ch);
                }
                // Dot
                '.' => match curtok.kind {
                    String | Comment => curtok.push_char(ch),
                    Integer => {
                        curtok.kind = Float;
                        curtok.push_char(ch);
                    }
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = Dot;
                    }
                },
                // Whitespace
                ' ' => match curtok.kind {
                    Comment | String => {
                        curtok.push_char(ch);
                    }
                    _ => {
                        list.push(&mut curtok);
                        curtok.col += 1;
                    }
                },
                '\t' => match curtok.kind {
                    Comment | String => {
                        curtok.push_char(ch);
                    }
                    _ => {
                        list.push(&mut curtok);
                        curtok.col += 4;
                    }
                },
                // new line (directs cursor to next row)
                '\n' => {
                    list.push(&mut curtok);
                    curtok.ln += 1;
                    curtok.col = 1;
                }
                // carriage return (directs cursor to beginning of col)
                '\r' => {
                    list.push(&mut curtok);
                    curtok.col = 1;
                }
                // comment
                '#' => {
                    list.push(&mut curtok);
                    curtok.kind = Comment;
                    curtok.push_char(ch);
                }
                // string
                '"' => match curtok.kind {
                    String => {
                        list.push(&mut curtok);
                    }
                    Comment => curtok.push_char(ch),
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = String;
                    }
                },
                _ => match curtok.kind {
                    Whitespace | Integer | Float => {
                        list.push(&mut curtok);
                        curtok.kind = Identifier;
                        curtok.push_char(ch);
                    }
                    _ => {
                        curtok.push_char(ch);
                    }
                },
            }
        }
        // might have  an identifier at the end of the document
        list.push(&mut curtok);

        // add EOF
        curtok.kind = EOF;
        list.push(&mut curtok);

        return list.as_vec();
    }

    fn peek(&self) -> Option<&char> {
        return self.source.get(self.endidx + 1);
    }

    fn string() {
        
    }

    fn add_token(&mut self, kind: TokenKind) {
        let text = self.source[self.startidx..self.endidx].iter().collect();

        self.tokens.push(Token {
            kind: kind,
            text: text,
            ln: self.ln,
            col: self.col,
        });

        self.startidx = self.endidx;
    }
}
