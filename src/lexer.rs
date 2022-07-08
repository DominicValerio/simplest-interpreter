use std::vec;

use crate::token::{*, self};
use std::string::String as StdString;
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
            col: 1,
            ln: 1,
            startidx: 0,
            endidx: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        while self.endidx < self.source.len() {
            let ch = self.source[self.endidx];
            self.endidx += 1;
            self.col += 1;

            match ch {
                ' ' => self.startidx += 1,
                '\t' => {
                    self.col += 3;
                    self.startidx += 1;
                },
                '\n' => {
                    self.startidx += 1;
                    self.col = 0;
                    self.ln += 1;
                }
                '\r' => self.col = 0,
                '+' => self.add_token(Plus),
                '-' => self.add_token(Minus),
                '*' => self.add_token(Star),
                '/' => self.add_token(Slash),
                '{' => self.add_token(Lbrace),
                '}' => self.add_token(Rbrace),
                '(' => self.add_token(Lparen),
                ')' => self.add_token(Rparen),
                ',' => self.add_token(Comma),
                ';' => self.add_token(Semicolon),
                '=' => {
                    if self.cur_is('=') {
                        self.advance();
                        self.add_token(Equals);
                    } else {
                        self.add_token(Assign);
                    }
                }
                '!' => {
                    if self.cur_is('=') {
                        self.advance();
                        self.add_token(NotEquals);
                    } else {
                        self.add_token(Bang);
                    }
                }
                '<' => {
                    if self.cur_is('=') {
                        self.advance();
                        self.add_token(LessEquals);
                    } else {
                        self.add_token(LessThan);
                    }
                }
                '>' => {
                    if self.cur_is('=') {
                        self.advance();
                        self.add_token(GreaterEquals);
                    } else {
                        self.add_token(GreaterThan);
                    }
                }
                '0'..='9' => self.number(),
                '"' => self.string(),
                _ => self.ident(),
            }
        }

        self.add_token(EOF);

        return self.tokens.clone();
    }

    fn peek(&self) -> Option<&char> {
        return self.source.get(self.endidx + 1);
    }

    fn peek_is(&self, ch: char) -> bool {
        match self.peek() {
            Some(peek_ch) => *peek_ch == ch,
            None => false,
        }
    }

    fn cur_is(&self, ch: char) -> bool {
        match self.source.get(self.endidx) {
            Some(peek_ch) => *peek_ch == ch,
            None => false,
        }
    }

    fn curch(&self) -> Option<&char> {
        return self.source.get(self.endidx);
    }

    fn ident(&mut self) {
        while let Some(ch) = self.curch(){
            if ch.is_alphabetic() {
                self.advance();
            } else {
                break;
            }
        }

        let text: StdString = self.source[self.startidx..self.endidx].iter().collect();

        let kind = match keywords().get(&text) {
            Some(_kind) => _kind.clone(),
            None => Identifier,
        };

        self.add_token(kind);
    }

    fn string(&mut self) {
        self.startidx += 1;
        while let Some(ch) = self.source.get(self.endidx) {
            self.endidx += 1;
            self.col += 1;
            if *ch == '"' {
                self.endidx -= 1;
                break;
            }
        }

       
        self.add_token(String);
        
        self.endidx += 1;
        self.col += 1;
        self.startidx = self.endidx;     
        
        // self.startidx = self.endidx + 1;
    }

    fn number(&mut self) {
        while let Some(ch) = self.source.get(self.endidx) {
            if ch.is_ascii_digit() {
                self.endidx += 1;
                self.col += 1;
            } else {
                break;
            }
        }

        let mut kind = Integer;

        if self.curch() == Some(&'.') {
            kind = Float;
            self.endidx += 1;
            self.col += 1;
        }

        while let Some(ch) = self.source.get(self.endidx) {
            if ch.is_ascii_digit() {
                self.endidx += 1;
                self.col += 1;
            } else {
                break;
            }
        }

        self.add_token(kind);
    }

    fn advance(&mut self) {
        self.endidx += 1;
        self.col += 1;
    }


    fn add_token(&mut self, kind: TokenKind) {
        let text: StdString = self.source[self.startidx..self.endidx].iter().collect();
        let len = &text.len();

        self.tokens.push(Token {
            kind: kind,
            text: text,
            ln: self.ln,
            col: self.col - len,
        });

        self.startidx = self.endidx;
    }
}
