use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::{Lexer, Token, TokenType};

mod tests;

struct Parser {
    lexer: Box<Lexer>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(lexer: Box<Lexer>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
            peek_token: None,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Option<Box<Program>> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.as_ref().unwrap().token_type != TokenType::Eof {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap())
            }
            self.next_token();
        }

        Some(Box::new(program))
    }

    fn parse_statement(&mut self) -> Option<Box<LetStatement>> {
        match self.current_token.as_ref().unwrap().token_type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<LetStatement>> {
        let mut stmt = Box::new(LetStatement {
            token: self.current_token.clone().unwrap(),
            name: None,
            value: None,
        });

        if self.peek_token_is(TokenType::Ident("".to_string())) {
            stmt.name = Some(Identifier {
                token: self.current_token.as_ref().unwrap().clone(),
                value: self.current_token.as_ref().unwrap().literal.clone(),
            });
        } else {
            return None;
        }

        if self.peek_token_is(TokenType::Assign) {
            while !self.current_token_is(TokenType::Semicolon) {
                self.next_token();
            }
        } else {
            return None;
        }

        Some(stmt)
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.as_ref().unwrap().token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.as_ref().unwrap().token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
