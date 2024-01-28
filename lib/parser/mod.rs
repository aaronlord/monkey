#![allow(dead_code)]

mod ast;

use crate::lexer::token::Name;
use crate::lexer::{token::Token, Lexer};
use ast::{Program, Statement};

use self::ast::{Expression, Identifier};

pub struct Parser {
    lexer: Lexer,
    token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            token,
            peek_token,
        }
    }

    pub fn next_token(&mut self) {
        self.token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = vec![];

        while !self.token.is_eof() {
            let statement = self.parse_statement();

            program.push(statement);

            self.next_token();
        }

        return program;
    }

    pub fn parse_statement(&mut self) -> Statement {
        match self.token.name {
            Name::LET => self.parse_let_statement(),
            Name::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_let_statement(&mut self) -> Statement {
        match self.peek_token.name {
            Name::IDENT(_) => (),
            _ => panic!("parse_let_statement: expected IDENT, got {:?}", self.peek_token.name),
        }

        self.next_token();

        let identifier = Identifier::new(self.token.clone());

        match self.peek_token.name {
            Name::ASSIGN => (),
            _ => panic!("parse_let_statement: expected ASSIGN, got {:?}", self.peek_token.name),
        }

        self.next_token();

        // @todo parse expression
        while !self.token.is_semicolon() {
            self.next_token();
        }

        let expression = Expression::Identifier(identifier.clone());

        Statement::LetStatement(identifier, expression)
    }

    pub fn parse_return_statement(&mut self) -> Statement {
        Statement::ReturnStatement
    }

    pub fn parse_expression_statement(&mut self) -> Statement {
        Statement::ExpressionStatement
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_let_statements() {
        let input = String::from(
            "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ",
        );

        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.len(), 3);
    }
}
