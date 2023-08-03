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
