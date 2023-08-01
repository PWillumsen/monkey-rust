#![allow(dead_code)]
use std::mem;

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
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(l: Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer: l,
            current_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut p = Program {
            statements: Vec::new(),
        };

        while let Some(_) = self.current_token {
            if let Ok(stmt) = self.parse_statement() {
                p.statements.push(stmt)
            };
            // TODO: handle errors
            self.next_token();
        }

        p
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.current_token {
            Some(Token::Let) => self.parse_let_statement(),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement> {
        let identifier: String;

        if let Some(Token::Identifier(ident)) = self.peek_token.clone() {
            identifier = ident;
            self.next_token();
        } else {
            return Err(ParserError::ExpectedIdentifier(
                self.peek_token.clone().unwrap(),
            ));
        }

        Ok(Statement::Let(identifier, Expression::Integer(10)))
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
            Statement::Let("x".to_string(), Expression::Integer(10)),
            Statement::Let("y".to_string(), Expression::Boolean(true)),
            Statement::Let("z".to_string(), Expression::Identifier("y".to_string())),
        ];

        assert_eq!(program.statements, expected);
    }
}
