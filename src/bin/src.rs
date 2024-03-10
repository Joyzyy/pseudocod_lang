use library::lexer::{Lexer, Token, TokenType};
use std::io::{self, Write};

fn start() -> std::io::Result<Vec<Token>> {
    print!(">> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut lexer = Lexer::new(input);
    let mut vec: Vec<Token> = vec![];
    loop {
        let token = lexer.next_token();
        if token.token_type == TokenType::Eof {
            break;
        }
        vec.push(token);
    }

    Ok(vec)
}

fn main() {
    loop {
        match start() {
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
