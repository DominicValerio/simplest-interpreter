use crate::token::*;
use TokenKind::*;

#[derive(Debug)]
pub struct Lexer {
    text: String,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            text: input.to_string(),
        }
    }

    pub fn parse(&self) -> Vec<Token> {
        let mut curtok = Token::new();
        let mut list = TokenStream::new();

        for ch in self.text.chars() {
            match ch {
                // numbers
                '0'..='9' => match curtok.kind {
                    Whitespace => {
                        curtok.kind = NumberegerLiteral;
                        curtok.push_char(ch);
                    }
                    _ => curtok.push_char(ch),
                },
                // single operators
                '+' | '-' | '*' | '{' | '}' | '(' | ')' | ',' | ';' => match curtok.kind {
                    StringLiteral | Comment => curtok.push_char(ch),
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = TokenKind::from_char(ch);
                        curtok.push_char(ch);
                        list.push(&mut curtok);
                    }
                },
                // operators that can be combined. e.g <=
                '<' | '>' | '!' => match curtok.kind {
                    StringLiteral | Comment => curtok.push_char(ch),
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = TokenKind::from_char(ch);
                        curtok.push_char(ch);
                    }
                },
                // Assignment
                '=' => match curtok.kind {
                    StringLiteral | Comment => curtok.push_char(ch),
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
                    curtok.push_char(ch);
                }
                // Whitespace
                ' ' => match curtok.kind {
                    Comment | StringLiteral => {
                        curtok.push_char(ch);
                    }
                    _ => {
                        list.push(&mut curtok);
                        curtok.col += 1;
                    }
                },
                '\t' => match curtok.kind {
                    Comment | StringLiteral => {
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
                    StringLiteral => {
                        list.push(&mut curtok);
                    }
                    Comment => curtok.push_char(ch),
                    _ => {
                        list.push(&mut curtok);
                        curtok.kind = StringLiteral;
                    }
                },
                _ => match curtok.kind {
                    Whitespace | NumberegerLiteral => {
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
}
