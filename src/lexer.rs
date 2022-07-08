// Lexer/Scanner/Tokenizer

use std::{vec, string::String as StdString, fmt::Display};
use crate::token::{*, TokenKind as tk};

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

    pub fn parse(&mut self) -> Result<Vec<Token>, String> {
        use tk::*;

        while self.endidx < self.source.len() {
            let ch = self.source[self.endidx];
            self.endidx += 1;
            self.col += 1;

            match ch {
                // operators
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
                    if self.curch_is('=') {
                        self.advance();
                        self.add_token(Equals);
                    } else {
                        self.add_token(Assign);
                    }
                }
                '!' => {
                    if self.curch_is('=') {
                        self.advance();
                        self.add_token(NotEquals);
                    } else {
                        self.add_token(Bang);
                    }
                }
                '<' => {
                    if self.curch_is('=') {
                        self.advance();
                        self.add_token(LessEquals);
                    } else {
                        self.add_token(LessThan);
                    }
                }
                '>' => {
                    if self.curch_is('=') {
                        self.advance();
                        self.add_token(GreaterEquals);
                    } else {
                        self.add_token(GreaterThan);
                    }
                }
                // whitespace
                ' ' => self.startidx += 1,
                '\t' => {
                    self.col += 3;
                    self.startidx += 1;
                }
                '\n' => {
                    self.startidx += 1;
                    self.col = 0;
                    self.ln += 1;
                }
                '\r' => self.col = 0,
                // numbers
                '0'..='9' => self.number(),
                // strings
                '"' => self.string()?,
                // identifiers
                'a'..='z' | '_' | 'A'..='Z' => self.ident(),
                _ => return Err(self.error(format!("Unknown symbol {ch}"))),
            }
        }

        self.add_token(EOF);

        return Ok(self.tokens.clone());
    }

    fn ident(&mut self) {
        while let Some(ch) = self.curch() {
            if ch.is_alphabetic() {
                self.advance();
            } else {
                break;
            }
        }

        let text: StdString = self.source[self.startidx..self.endidx].iter().collect();

        let kind = match keywords().get(&text) {
            Some(_kind) => _kind.clone(),
            None => tk::Identifier,
        };

        self.add_token(kind);
    }

    fn string(&mut self) -> Result<(), StdString> {
        while self.curch() != Some(&'"') && self.peek() != None {
            self.advance();
        }

        // if there's no matching quote
        if self.curch() != Some(&'"') {
            return Err(self.error(format!(
                "No closing quote for string {}",
                self.source[self.startidx + 1..self.endidx + 1]
                    .iter()
                    .collect::<StdString>()
            )));
        }

        // correct the position to not include quotes
        self.startidx += 1;
        self.endidx -= 1;
        self.col -= 2;

        self.advance();

        self.add_token(tk::String);

        // correct the position to the quotes that were in the string
        self.startidx += 1;
        self.endidx += 1;
        self.col += 2;
        Ok(())
    }

    fn number(&mut self) {
        while let Some(ch) = self.curch() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let kind = match self.curch() {
            Some('.') => {
                self.advance();
                tk::Float
            }
            _ => tk::Integer,
        };

        while let Some(ch) = self.curch() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        self.add_token(kind);
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

    fn error<S: Into<StdString> + Display>(&self, text: S) -> StdString {
        format!("(Ln {}, Col {}) {}", self.ln, self.col, text)
    }

    fn advance(&mut self) {
        self.endidx += 1;
        self.col += 1;
    }

    fn peek(&self) -> Option<&char> {
        return self.source.get(self.endidx + 1);
    }

    fn curch_is(&self, ch: char) -> bool {
        match self.source.get(self.endidx) {
            Some(peek_ch) => *peek_ch == ch,
            None => false,
        }
    }

    fn curch(&self) -> Option<&char> {
        return self.source.get(self.endidx);
    }
}
