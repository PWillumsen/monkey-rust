use monkeyterp::lexer::Lexer;
use monkeyterp::parser::Parser;
use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Could not read from stdin");

        let lexer = Lexer::new(&buffer);
        let parser = Parser::new(lexer);
        let program = parser.parse_program();
        // for token in l {
        //     println!("{:?}", token);
        // }
        println!("{program}");
        buffer.clear();
    }
}
