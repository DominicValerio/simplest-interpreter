use crate::token::TokenKind;

pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    None,
    Bool(bool),
    Int(i32),
    Str(String),
    Identifier(String),
    Assign(Box<Expression>, Box<Expression>),
    Call {
      name: String,
      args: Vec<Expression>,
    },
    BinOp(Box<Expression>, TokenKind, Box<Expression>), // left, op, right
}

impl Expression {
    pub fn some(self) -> Option<Self> {
        Some(self)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
