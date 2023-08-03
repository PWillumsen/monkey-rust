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
    ExpectedIdentifier,
    UnexpectedError,
    ExpectedAssignToken,
}

#[derive(Debug)]
struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    fn new(l: Lexer<'a>) -> Self {
        Parser {
            lexer: l.peekable(),
            errors: Vec::new(),
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut p = Program {
            statements: Vec::new(),
        };

        while let Some(_) = self.lexer.peek() {
            match self.parse_statement() {
                Ok(stmt) => p.statements.push(stmt),
                Err(e) => self.errors.push(e),
            };
            self.lexer.next();
        }

        p
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.lexer.next() {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement> {
        let identifier = match self.lexer.peek() {
            Some(Token::Identifier(_)) => self.lexer.next().unwrap(),
            _ => return Err(ParserError::ExpectedIdentifier),
        };

        if self.lexer.next_if_eq(&Token::Assign).is_none() {
            return Err(ParserError::ExpectedAssignToken);
        }

        self.lexer.next();
        let expr = self.parse_expression()?;
        while self.lexer.next_if(|t| t != &Token::Semicolon).is_some() {}
        self.lexer.next_if_eq(&Token::Semicolon);

        Ok(Statement::Let(identifier, expr))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        Ok(Expression::Integer(10))
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        while self.lexer.next_if(|t| t != &Token::Semicolon).is_some() {}
        Ok(Statement::Return(expr))
    }
}

#[cfg(test)]
mod test_parser_statements {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r"let x  10;
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
    #[test]
    fn test_return_statements() {
        let input = r"return 10;
                    return 5;
                    return 99999;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let expected = vec![
            Statement::Return(Expression::Integer(10)),
            Statement::Return(Expression::Integer(5)),
            Statement::Return(Expression::Integer(99999)),
        ];

        assert_eq!(program.statements, expected);
    }
}
