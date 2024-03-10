use std::fmt::Display;

mod tests;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    Eq,
    NotEq,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Token {
    fn new<T: Into<String> + Display>(token_type: TokenType, literal: T) -> Self {
        Self {
            token_type,
            literal: literal.into(),
        }
    }

    fn lookup_ident(literal: &str) -> TokenType {
        match literal {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident(literal.to_string()),
        }
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::default();
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position > self.input.len() {
            char::default()
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenType::Eq, "==")
                }
                _ => Token::new(TokenType::Assign, self.ch),
            },
            ';' => Token::new(TokenType::Semicolon, self.ch),
            '(' => Token::new(TokenType::LParen, self.ch),
            ')' => Token::new(TokenType::RParen, self.ch),
            '{' => Token::new(TokenType::LBrace, self.ch),
            '}' => Token::new(TokenType::RBrace, self.ch),
            ',' => Token::new(TokenType::Comma, self.ch),
            '+' => Token::new(TokenType::Plus, self.ch),
            '-' => Token::new(TokenType::Minus, self.ch),
            '!' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=")
                }
                _ => Token::new(TokenType::Bang, self.ch),
            },
            '*' => Token::new(TokenType::Asterisk, self.ch),
            '/' => Token::new(TokenType::Slash, self.ch),
            '<' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenType::Lte, "<=")
                }
                _ => Token::new(TokenType::Lt, self.ch),
            },
            '>' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenType::Gte, ">=")
                }
                _ => Token::new(TokenType::Gt, self.ch),
            },
            '\0' => Token::new(TokenType::Eof, self.ch),
            _ => match (self.ch.is_ascii_alphabetic(), self.ch.is_ascii_digit()) {
                (true, _) => {
                    let ident = self.read_identifier();
                    let token_type = Token::lookup_ident(ident);
                    Token::new(token_type, ident)
                }
                (_, true) => {
                    let token_number = self.read_number();
                    Token::new(
                        TokenType::Int(token_number.to_string().clone()),
                        token_number,
                    )
                }
                _ => Token::new(TokenType::Illegal, self.ch),
            },
        };
        self.read_char();
        token
    }

    fn read_while<F>(&mut self, condition: F) -> &str
    where
        F: Fn(char) -> bool,
    {
        let position = self.position;
        while condition(self.ch) {
            self.read_char();
        }
        self.read_position -= 1;
        &self.input[position..self.read_position]
    }

    fn read_identifier(&mut self) -> &str {
        self.read_while(|ch| ch.is_ascii_alphabetic())
    }

    fn read_number(&mut self) -> &str {
        self.read_while(|ch| ch.is_ascii_digit())
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
