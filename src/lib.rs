#[derive(Debug, PartialEq)]
enum TokenType {
    Illegal(),
    Eof(),

    Ident(String),
    Int(String),

    Comma(String),
    Semicolon(String),

    LParen(String),
    RParen(String),
    LBrace(String),
    RBrace(String),

    // keywords
    Function(String),
    Let(String),
    True(String),
    False(String),
    If(String),
    Else(String),
    Return(String),

    // logic operators
    Eq(String),
    NotEq(String),

    // operators
    Assign(String),
    Plus(String),
    Minus(String),
    Bang(String),
    Asterisk(String),
    Slash(String),
    Lt(String),
    Gt(String),
}

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

#[allow(dead_code)]
impl Lexer {
    fn new(input: String) -> Self {
        let mut l = Self {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::default();
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            char::default()
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    TokenType::Eq("==".to_string())
                }
                _ => TokenType::Assign(self.ch.to_string()),
            },
            ';' => TokenType::Semicolon(self.ch.to_string()),
            '(' => TokenType::LParen(self.ch.to_string()),
            ')' => TokenType::RParen(self.ch.to_string()),
            '{' => TokenType::LBrace(self.ch.to_string()),
            '}' => TokenType::RBrace(self.ch.to_string()),
            ',' => TokenType::Comma(self.ch.to_string()),
            '+' => TokenType::Plus(self.ch.to_string()),
            '-' => TokenType::Minus(self.ch.to_string()),
            '!' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    TokenType::NotEq("!=".to_string())
                }
                _ => TokenType::Bang(self.ch.to_string()),
            },
            '*' => TokenType::Asterisk(self.ch.to_string()),
            '/' => TokenType::Slash(self.ch.to_string()),
            '<' => TokenType::Lt(self.ch.to_string()),
            '>' => TokenType::Gt(self.ch.to_string()),
            '\0' => TokenType::Eof(),
            _ => {
                if self.is_letter() {
                    let ident = self.read_identifier();
                    self.lookup_ident(ident)
                } else if self.is_digit() {
                    TokenType::Int(self.read_number())
                } else {
                    TokenType::Illegal()
                }
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit() {
            self.read_char();
        }
        // after the loop, self.position will be the next character after the number
        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn is_letter(&self) -> bool {
        self.ch.is_ascii_alphabetic()
    }

    fn is_digit(&self) -> bool {
        self.ch.is_ascii_digit()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn lookup_ident(&self, ident: String) -> TokenType {
        match ident.as_str() {
            "fn" => TokenType::Function(ident),
            "let" => TokenType::Let(ident),
            "true" => TokenType::True(ident),
            "false" => TokenType::False(ident),
            "if" => TokenType::If(ident),
            "else" => TokenType::Else(ident),
            "return" => TokenType::Return(ident),
            _ => TokenType::Ident(ident),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, TokenType};
    #[test]
    fn test_one() {
        let input = r#"=+{}();"#.to_string();
        let tests = vec![
            TokenType::Assign("=".to_string()),
            TokenType::Plus("+".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Eof(),
        ];

        let mut lexer = Lexer::new(input);
        for tt in tests {
            let t = lexer.next_token();
            assert_eq!(t, tt);
        }
    }

    #[test]
    fn test_two() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "#
        .to_string();

        let tests = vec![
            TokenType::Let("let".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Function("fn".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Plus("+".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("result".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Eof(),
        ];

        let mut lexer = Lexer::new(input);

        for tt in tests {
            let t = lexer.next_token();
            assert_eq!(t, tt);
        }
    }

    #[test]
    fn test_three() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
        "#
        .to_string();
        let tests = vec![
            TokenType::Let("let".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Function("fn".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Plus("+".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("result".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Bang("!".to_string()),
            TokenType::Minus("-".to_string()),
            TokenType::Slash("/".to_string()),
            TokenType::Asterisk("*".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Lt("<".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Gt(">".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Eof(),
        ];

        let mut lexer = Lexer::new(input);
        for tt in tests {
            let t = lexer.next_token();
            assert_eq!(t, tt);
        }
    }

    #[test]
    fn test_four() {
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
        "#
        .to_string();

        let tests = vec![
            TokenType::Let("let".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Function("fn".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Plus("+".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("result".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Bang("!".to_string()),
            TokenType::Minus("-".to_string()),
            TokenType::Slash("/".to_string()),
            TokenType::Asterisk("*".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Lt("<".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Gt(">".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::If("if".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Lt("<".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Return("return".to_string()),
            TokenType::True("true".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Else("else".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Return("return".to_string()),
            TokenType::False("false".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Eof(),
        ];

        let mut lexer = Lexer::new(input);
        for tt in tests {
            let t = lexer.next_token();
            assert_eq!(t, tt);
        }
    }

    #[test]
    fn test_five() {
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
        "#
        .to_string();

        let tests = vec![
            TokenType::Let("let".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Function("fn".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Ident("x".to_string()),
            TokenType::Plus("+".to_string()),
            TokenType::Ident("y".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Let("let".to_string()),
            TokenType::Ident("result".to_string()),
            TokenType::Assign("=".to_string()),
            TokenType::Ident("add".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Ident("five".to_string()),
            TokenType::Comma(",".to_string()),
            TokenType::Ident("ten".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Bang("!".to_string()),
            TokenType::Minus("-".to_string()),
            TokenType::Slash("/".to_string()),
            TokenType::Asterisk("*".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Lt("<".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Gt(">".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::If("if".to_string()),
            TokenType::LParen("(".to_string()),
            TokenType::Int("5".to_string()),
            TokenType::Lt("<".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::RParen(")".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Return("return".to_string()),
            TokenType::True("true".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Else("else".to_string()),
            TokenType::LBrace("{".to_string()),
            TokenType::Return("return".to_string()),
            TokenType::False("false".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::RBrace("}".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Eq("==".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Int("10".to_string()),
            TokenType::NotEq("!=".to_string()),
            TokenType::Int("9".to_string()),
            TokenType::Semicolon(";".to_string()),
            TokenType::Eof(),
        ];

        let mut lexer = Lexer::new(input);
        for tt in tests {
            let t = lexer.next_token();
            assert_eq!(t, tt);
        }
    }
}
