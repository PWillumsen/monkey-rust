#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(String, Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Integer(i32),
    Boolean(bool),
    Identifier(String),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
