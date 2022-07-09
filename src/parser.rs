//! Recursive descent based parser

use crate::{ast::*, token::TokenKind as tk, token::*};
use std::{fmt::Display, iter::Peekable, vec::IntoIter};

#[derive(Debug)]
pub struct Parser {
    curtok: Token,
    iter: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            curtok: tokens[0].clone(),
            iter: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        self.next();

        let mut program = vec![];

        while !self.curtok_is(tk::EOF) {
            self.parse_next(&mut program)?;
        }

        return Ok(program);
    }

    fn parse_next(&mut self, program: &mut Program) -> Result<(), String> {
        use tk::*;
        use Statement::{Block, Expression};

        let clone = self.curtok.clone();
        match self.curtok.kind {
            Var => program.push((self.parse_var()?, clone)),
            Fn => program.push((self.parse_function()?, clone)),
            While => program.push((self.parse_while()?, clone)),
            Lbrace => program.push((Block(self.parse_block()?), clone)),
            Semicolon | Comment => drop(self.next()),
            _ => program.push((Expression(self.parse_expression(Precedence::Iota)?), clone)),
        }
        Ok(())
    }

    fn next(&mut self) -> Option<Token> {
        match self.iter.next() {
            Some(v) => {
                self.curtok = v.clone();
                return Some(v);
            }
            None => None,
        }
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        self.expect_kind(TokenKind::While)?;
        self.next();

        let condition = self.parse_expression(Precedence::Iota)?;

        let body = self.parse_block()?;

        return Ok(Statement::While {
            condition: condition,
            body: body,
        });
    }

    fn parse_function(&mut self) -> Result<Statement, String> {
        // expect fn keyword (just in case)
        self.expect_kind(TokenKind::Fn)?;
        self.next();
        //expect identifier
        let iden = self.expect_kind(TokenKind::Identifier)?;
        self.next();
        //expect parameters
        self.expect_kind(TokenKind::Lparen)?;
        self.next();

        //form parameters
        let mut params = Vec::new();

        while !self.curtok_is(TokenKind::Rparen) {
            if self.curtok_is(TokenKind::Comma) {
                let _param = self.expect_peek(TokenKind::Identifier)?;
                self.next();
            }

            if self.curtok_is(TokenKind::Identifier) {
                params.push(self.curtok.text.clone());
                self.next();
            }
        }

        //expect function body
        self.next();

        let body = self.parse_block()?;

        Ok(Statement::FunctionDeclaration {
            name: iden.text,
            body: body,
            params: params,
        })
    }

    fn parse_var(&mut self) -> Result<Statement, String> {
        // sanity check
        self.expect_kind(TokenKind::Var)?;
        self.next();
        // expect identifier
        let iden = self.expect_kind(TokenKind::Identifier)?;
        self.next();

        // expect assignment
        self.expect_kind(TokenKind::Assign)?;
        self.next();

        // parse expression
        let expr = self.parse_expression(Precedence::Iota)?;

        let res = Statement::VarDeclaration {
            name: iden.text,
            value: expr,
        };

        return Ok(res);
    }

    /// Parses the areas between {}
    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        self.expect_kind(TokenKind::Lbrace)?;
        self.next();

        let mut block = vec![];

        while self.curtok.kind != TokenKind::Rbrace {
            block.push(self.parse_statement()?);
        }

        self.next();

        return Ok(block);
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        use tk::*;
        match self.curtok.kind {
            Var => self.parse_var(),
            Return => self.parse_return(),
            While => self.parse_while(),
            Lbrace => Ok(Statement::Block(self.parse_block()?)),
            _ => Ok(Statement::Expression(
                self.parse_expression(Precedence::Iota)?,
            )),
        }
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        // sanity check
        self.expect_kind(TokenKind::Return)?;
        self.next();

        let res = Statement::Return(self.parse_expression(Precedence::Iota)?);

        return Ok(res);
    }

    /*
    The first time this function is called, the precendence is the lowest.
    Subsequent times, the precedence is replaced with the precedence of a token.
    */
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        use tk::*;

        let mut left = match self.curtok.kind {
            Integer | Float => {
                let clone = self.curtok.clone();
                Expression::Number(clone.text.parse().unwrap())
            }
            Identifier => Expression::Identifier(self.curtok.text.clone()),
            Lparen => {
                self.next();
                //TODO: probably can't have multiple paranthesis inside each otehr
                self.parse_expression(Precedence::Iota)?
            }
            String => Expression::Str(self.curtok.text.clone()),
            True => Expression::Bool(true),
            False => Expression::Bool(false),
            _ => {
                dbg!(&self);
                return Err(self.error(format!(
                    "Expected an expression. Instead got {:?}",
                    self.curtok.kind
                )));
            }
        };

        self.next();

        while !self.curtok_is(TokenKind::EOF) && precedence < Precedence::of_token(&self.curtok) {
            if let Some(expression) = self.parse_postfix_expression(&left)? {
                left = expression;
            } else if let Some(expression) = self.parse_infix_expression(&left)? {
                left = expression;
            } else {
                break;
            }
        }

        if self.curtok_is(TokenKind::Semicolon) {
            self.next();
        }

        return Ok(left);
    }

    fn parse_infix_expression(&mut self, left: &Expression) -> Result<Option<Expression>, String> {
        use tk::*;
        match self.curtok.kind {
            Slash | Mul | Minus | Plus | Equals | NotEquals | LessThan | GreaterThan
            | GreaterEquals | LessEquals => {
                let token = self.curtok.clone();

                self.next();

                let right = self.parse_expression(Precedence::of_token(&token))?;

                return Ok(Some(Expression::BinOp(
                    Box::new(left.clone()),
                    token.kind,
                    Box::new(right),
                )));
            }
            Assign => {
                self.next();

                let right = self.parse_expression(Precedence::Iota)?;

                Ok(Some(Expression::Assign(
                    Box::new(left.clone()),
                    Box::new(right),
                )))
            }
            _ => Ok(None),
        }
    }

    fn parse_postfix_expression(
        &mut self,
        left: &Expression,
    ) -> Result<Option<Expression>, String> {
        use tk::*;
        match self.curtok.kind {
            Lparen => {
                // parse a possibly delimited list
                self.next();

                let mut args = Vec::new();

                while !self.curtok_is(Rparen) {
                    args.push(self.parse_expression(Precedence::Iota)?);

                    if self.curtok.kind == Comma {
                        self.next();
                    }
                }

                self.expect_kind(Rparen)?;

                self.next();

                if let Expression::Identifier(v) = left {
                    let res = Expression::Call {
                        name: v.clone(),
                        args: args,
                    };

                    return Ok(Some(res));
                }
                dbg!(left);
                unreachable!();
            }
            _ => Ok(None),
        }
    }

    fn peek(&mut self) -> Result<Token, String> {
        if let Some(t) = self.iter.peek() {
            return Ok(t.clone());
        }

        Err(self.error("Parsed past EOF"))
    }

    fn error<S: Into<String> + Display>(&self, text: S) -> String {
        format!("(Ln {}, Col {}) {}", self.curtok.ln, self.curtok.col, text)
    }

    fn expect_peek(&mut self, kind: TokenKind) -> Result<Token, String> {
        let peek = self.peek()?;
        if peek.kind == kind {
            return Ok(peek);
        }
        Err(self.error(format!(
            "Expected the next token to be {:?}, instead got {:?}",
            kind, peek.kind
        )))
    }

    fn expect_kind(&mut self, kind: TokenKind) -> Result<Token, String> {
        if self.curtok.kind == kind {
            Ok(self.curtok.clone())
        } else {
            Err(self.error(format!(
                "Expected {:?}. Instead got {:?}",
                kind, self.curtok.kind
            )))
        }
    }

    fn curtok_is(&mut self, kind: TokenKind) -> bool {
        return self.curtok.kind == kind;
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    /// The lowest and starting point of precedence
    Iota,
    Statement,
    Assign,
    LessThanGreaterThan,
    Equals,
    Sum,
    Product,
    /// The highest precedence
    Call,
}

impl Precedence {
    pub fn of_token(tok: &Token) -> Precedence {
        use tk::*;
        use Precedence as prec;

        match tok.kind {
            Lparen => prec::Call,
            Slash | Mul => prec::Product,
            Plus | Minus => prec::Sum,
            Equals | NotEquals => prec::Equals,
            LessThan | GreaterThan | LessEquals | GreaterEquals => prec::LessThanGreaterThan,
            Assign => prec::Assign,
            Semicolon => prec::Statement,
            _ => prec::Iota,
        }
    }
}
