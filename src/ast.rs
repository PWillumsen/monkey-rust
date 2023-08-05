use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(Token, Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Integer(i32),
    Boolean(bool),
    Identifier(Token),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(token, expr) => write!(f, "let {} = {}", token, expr),
            Statement::Return(expr) => write!(f, "return {}", expr),
            Statement::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{i}"),
            Expression::Boolean(b) => write!(f, "{b}"),
            Expression::Identifier(t) => write!(f, "{t}"),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.statements {
            write!(f, "{}\n", s)?
        }
        Ok(())
    }
}
