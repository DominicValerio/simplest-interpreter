use crate::token::TokenKind;

pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Block,
    },
    VarDeclaration {
        name: String,
        value: Expression,
    },
    While {
        condition: Expression,
        body: Block,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Nil,
    Bool(bool),
    Number(f64),
    Str(String),
    Identifier(String),
    Assign(Box<Expression>, Box<Expression>),
    Call { name: String, args: Vec<Expression> },
    BinOp(Box<Expression>, TokenKind, Box<Expression>), // left, op, right
}
