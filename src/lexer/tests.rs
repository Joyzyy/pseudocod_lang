use super::{Lexer, TokenType};

#[test]
fn simple_lexer() {
    let input = r#"==+{}();!="#.to_string();
    let tests = vec![
        TokenType::Eq,
        TokenType::Plus,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::LParen,
        TokenType::RParen,
        TokenType::Semicolon,
        TokenType::NotEq,
    ];

    let mut lexer = Lexer::new(input);

    for test in tests {
        let token = lexer.next_token();
        assert_eq!(test, token.token_type);
    }
}

#[test]
fn complex_lexer() {
    let input = r#"
                let five = 5;
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

                10>=4
                5<=5
            "#
    .to_string();

    let tests = vec![
        TokenType::Let,
        TokenType::Ident("five".to_string()),
        TokenType::Assign,
        TokenType::Int("5".to_string()),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident("ten".to_string()),
        TokenType::Assign,
        TokenType::Int("10".to_string()),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident("add".to_string()),
        TokenType::Assign,
        TokenType::Function,
        TokenType::LParen,
        TokenType::Ident("x".to_string()),
        TokenType::Comma,
        TokenType::Ident("y".to_string()),
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::Ident("x".to_string()),
        TokenType::Plus,
        TokenType::Ident("y".to_string()),
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident("result".to_string()),
        TokenType::Assign,
        TokenType::Ident("add".to_string()),
        TokenType::LParen,
        TokenType::Ident("five".to_string()),
        TokenType::Comma,
        TokenType::Ident("ten".to_string()),
        TokenType::RParen,
        TokenType::Semicolon,
        TokenType::Bang,
        TokenType::Minus,
        TokenType::Slash,
        TokenType::Asterisk,
        TokenType::Int("5".to_string()),
        TokenType::Semicolon,
        TokenType::Int("5".to_string()),
        TokenType::Lt,
        TokenType::Int("10".to_string()),
        TokenType::Gt,
        TokenType::Int("5".to_string()),
        TokenType::Semicolon,
        TokenType::If,
        TokenType::LParen,
        TokenType::Int("5".to_string()),
        TokenType::Lt,
        TokenType::Int("10".to_string()),
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::Return,
        TokenType::True,
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::Else,
        TokenType::LBrace,
        TokenType::Return,
        TokenType::False,
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::Int("10".to_string()),
        TokenType::Eq,
        TokenType::Int("10".to_string()),
        TokenType::Semicolon,
        TokenType::Int("10".to_string()),
        TokenType::NotEq,
        TokenType::Int("9".to_string()),
        TokenType::Semicolon,
        TokenType::Int("10".into()),
        TokenType::Gte,
        TokenType::Int("4".into()),
        TokenType::Int("5".into()),
        TokenType::Lte,
        TokenType::Int("5".into()),
        TokenType::Eof,
    ];

    let mut lexer = Lexer::new(input);

    for test in tests {
        let token = lexer.next_token();
        assert_eq!(test, token.token_type);
    }
}
