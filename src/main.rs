use monkeyterp::lexer::Lexer;
use monkeyterp::token::Token;
use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Could not read from stdin");

        let l = Lexer::new(&buffer);

        for token in l {
            match token {
                Token::EOF => break,
                _ => println!("{:?}", token),
            }
        }

        buffer.clear();
    }
}
