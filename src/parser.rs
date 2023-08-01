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
struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut l: Lexer) -> Self {
        let current_token = l.next_token();
        let peek_token = l.next_token();
        Parser {
            lexer: l,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut p = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::EOF {
            let stmt = self.parse_statement();
            if stmt.is_ok() {
                p.statements.push(stmt.unwrap());
            };
            self.next_token();
        }

        p
    }

    fn parse_statement(&self) -> Result<Statement> {
        match self.current_token {
            Token::Let => Ok(self.parse_let_statement()?),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    fn parse_let_statement(&self) -> Result<Statement> {
        let identifier: String;

        if let Token::Identifier(ident) = self.peek_token {
            identifier = ident;
            self.next_token();
        } else {
            return Err(ParserError::ExpectedIdentifier(self.peek_token));
        }

        Ok(Statement::Let(identifier, expr))
    }
}

#[cfg(test)]
mod test_parser_statements {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r"let x = 5;
                    let y = true;
                    let z = y;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        // parser.check_parser_errors();

        let expected = vec![
            Statement::Let("x".to_string(), Expression::Integer(5)),
            Statement::Let("y".to_string(), Expression::Boolean(true)),
            Statement::Let("z".to_string(), Expression::Identifier("y".to_string())),
        ];

        assert_eq!(program.statements, expected);
    }
}
