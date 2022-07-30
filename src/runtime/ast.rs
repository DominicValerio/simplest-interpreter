//!Abstract Syntax Tree

use crate::token::{Token, TokenKind};

/// (statement, associated token)
pub type AstNode = (Statement, Token);
pub type Program = Vec<AstNode>;

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    VarDeclaration {
        name: String,
        value: Expression,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Bool(bool),
    Number(f64),
    Str(String),
    Identifier(String),
    Assign(Box<Expression>, Box<Expression>),
    Call { name: String, args: Vec<Expression> },
    BinOp(Box<Expression>, TokenKind, Box<Expression>), // left, op, right
}
