#![allow(dead_code)]

use crate::lexer::token::Token;

pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement,
    ExpressionStatement,
}

#[derive(Debug)]
pub struct Node {}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token) -> Identifier {
        let value = token.literal.to_string();

        Identifier {
            token,
            value,
        }
    }
}
