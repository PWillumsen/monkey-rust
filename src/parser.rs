#![allow(dead_code)]
use std::iter::Peekable;

use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::Token,
};

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
enum ParserError {
    ExpectedIdentifier(Token),
    UnexpectedError,
}

#[derive(Debug)]
struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(l: Lexer<'a>) -> Self {
        Parser {
            lexer: l.peekable(),
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut p = Program {
            statements: Vec::new(),
        };

        while let Some(token) = self.lexer.next() {
            if let Ok(stmt) = self.parse_statement(token) {
                p.statements.push(stmt)
            };
            // TODO: handle errors
            self.lexer.next();
        }

        p
    }

    fn parse_statement(&mut self, token: Token) -> Result<Statement> {
        match token {
            Token::Let => self.parse_let_statement(),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement> {
        match self.lexer.peek() {
            Some(Token::Identifier(_)) => {
                // TODO: parse expressions
                let ident = self.lexer.next().unwrap();
                let expr = Expression::Integer(10);
                Ok(Statement::Let(ident, expr))
            }
            _ => Err(ParserError::ExpectedIdentifier(
                self.lexer.peek().clone().unwrap().clone(),
            )),
        }
    }
}

#[cfg(test)]
mod test_parser_statements {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r"let x = 10;
                    let y = true;
                    let z = y;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        // parser.check_parser_errors();

        let expected = vec![
            Statement::Let(Token::Identifier("x".into()), Expression::Integer(10)),
            Statement::Let(Token::Identifier("y".into()), Expression::Boolean(true)),
            Statement::Let(
                Token::Identifier("z".into()),
                Expression::Identifier(Token::Identifier("y".into())),
            ),
        ];

        assert_eq!(program.statements, expected);
    }
}
