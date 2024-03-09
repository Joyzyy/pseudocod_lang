use lexer::lexer::{Lexer, TokenType};

fn start() -> std::io::Result<Vec<TokenType>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let mut lexer = Lexer::new(input);

    let mut vec: Vec<TokenType> = vec![];
    loop {
        let token = lexer.next_token();
        if token == TokenType::Eof() {
            break;
        }
        vec.push(token);
    }

    Ok(vec)
}

fn main() {
    let prompt: String = String::from(">> ");
    loop {
        print!("{}", prompt);
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
