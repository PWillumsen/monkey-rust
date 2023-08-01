#![allow(dead_code)]
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>, // TODO: change to Peekable<Chars<_>> ?
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

trait IsLetter {
    fn is_letter(&self) -> bool;
}
impl IsLetter for char {
    fn is_letter(&self) -> bool {
        self.is_alphabetic() || *self == '_'
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }
    fn peek_char(&self) -> Option<&char> {
        if self.read_position >= self.input.len() {
            return None;
        } else {
            return self.input.get(self.read_position);
        }
    }
    fn read_char(&mut self) {
        self.ch = match self.input.get(self.read_position) {
            Some(&char) => Some(char),
            None => None,
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn match_next_char(&mut self, c: char) -> bool {
        match self.peek_char() {
            Some(cc) if *cc == c => {
                self.read_char();
                true
            }
            _ => false,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
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
                '!' => match self.match_next_char('=') {
                    true => Token::NotEqual,
                    false => Token::Bang,
                },
                '=' => match self.match_next_char('=') {
                    true => Token::Equal,
                    false => Token::Assign,
                },
                c if c.is_letter() => {
                    let s = self.read_identifier();
                    return self.lookup_identifier(s);
                }
                c if c.is_digit(10) => return Token::Integer(self.read_number()),
                _ => Token::Illegal(t.to_string()),
            },
            None => Token::EOF,
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.unwrap_or('!').is_letter() {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
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

    fn skip_whitespace(&mut self) {
        while self.ch.unwrap_or('!').is_whitespace() {
            self.read_char()
        }
    }

    fn read_number(&mut self) -> i32 {
        let start = self.position;
        while self.ch.unwrap_or('a').is_digit(10) {
            self.read_char();
        }
        self.input[start..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position <= self.input.len() {
            Some(self.next_token())
        } else {
            None
        }
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
            assert_eq!(&tok, tt);
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
            assert_eq!(&tok, tt);
        }
    }
}
