use std::{iter::Peekable, str::Chars};

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

trait IsLetter {
    fn is_letter(&self) -> bool;
}
impl IsLetter for char {
    fn is_letter(&self) -> bool {
        self.is_alphabetic() || *self == '_'
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(_) = self.input.next_if(|c| c.is_whitespace()) {}

        let token = match self.input.next() {
            Some(t) => match t {
                '+' => Token::Plus,
                '-' => Token::Minus,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '/' => Token::Slash,
                '*' => Token::Asterisk,
                '<' => Token::LT,
                '>' => Token::GT,
                '!' => match self.input.next_if_eq(&'=') {
                    Some(_) => Token::NotEqual,
                    None => Token::Bang,
                },
                '=' => match self.input.next_if_eq(&'=') {
                    Some(_) => Token::Equal,
                    None => Token::Assign,
                },
                c if c.is_letter() => self.read_identifier(c),
                c if c.is_digit(10) => self.read_number(c),
                _ => Token::Illegal(t.to_string()),
            },
            None => return None,
        };
        Some(token)
    }

    fn read_identifier(&mut self, c: char) -> Token {
        let mut ident = String::from(c);
        while let Some(l) = self.input.next_if(|cc| cc.is_letter()) {
            ident.push(l)
        }
        self.lookup_identifier(ident)
    }

    fn lookup_identifier(&self, s: String) -> Token {
        match s.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Identifier(s),
        }
    }

    fn read_number(&mut self, c: char) -> Token {
        let mut num = String::from(c);
        while let Some(n) = self.input.next_if(|cc| cc.is_digit(10)) {
            num.push(n)
        }
        Token::Integer(num.parse().unwrap())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {

    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn lexer_tokenizes_simple_chars() {
        let input = "=+(){},;";

        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let mut l = Lexer::new(input);

        for tt in tests.iter() {
            let tok = l.next_token();
            dbg!(&tt);
            dbg!(&tok);
            assert_eq!(&tok.unwrap_or(Token::EOF), tt);
        }
    }

    #[test]
    fn lexer_tokenizes_all_monkey_tokens() {
        let input = "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10; 
            10 != 9;
            ";
        let tests = vec![
            Token::Let,
            Token::Identifier("five".into()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RParen,
            Token::LBrace,
            Token::Identifier("x".into()),
            Token::Plus,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("add".into()),
            Token::LParen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::LT,
            Token::Integer(10),
            Token::GT,
            Token::Integer(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Integer(5),
            Token::LT,
            Token::Integer(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Integer(10),
            Token::Equal,
            Token::Integer(10),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEqual,
            Token::Integer(9),
            Token::Semicolon,
            Token::EOF,
        ];

        let mut l = Lexer::new(input);

        for tt in tests.iter() {
            dbg!(&tt);
            let tok = l.next_token();
            dbg!(&tok);
            assert_eq!(&tok.unwrap_or(Token::EOF), tt);
        }
    }
}
