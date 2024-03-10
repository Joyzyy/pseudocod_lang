use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::{Lexer, Token, TokenType};

mod tests;

#[derive(Debug)]
struct Parser {
    lexer: Rc<RefCell<Lexer>>,
    errors: Vec<String>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: Rc::new(RefCell::new(lexer)),
            errors: Vec::new(),
            current_token: None,
            peek_token: None,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type,
            self.peek_token.as_ref().unwrap().token_type
        );
        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.borrow_mut().next_token());
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.as_ref().unwrap().token_type != TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Rc<RefCell<dyn Statement>>> {
        match self.current_token.as_ref().unwrap().token_type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Rc<RefCell<dyn Statement>>> {
        let mut statement = LetStatement {
            token: self.current_token.as_ref().unwrap().clone(),
            name: None,
            value: None,
        };

        if !self.expect_peek(TokenType::Ident(String::new())) {
            return None;
        }

        statement.name = Some(Identifier {
            token: self.current_token.as_ref().unwrap().clone(),
            value: self.current_token.as_ref().unwrap().literal.clone(),
        });

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Rc::new(RefCell::new(statement)))
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.as_ref().unwrap().token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.as_ref().unwrap().token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        match self.peek_token {
            Some(ref t) => match (&t.token_type, &token_type) {
                (TokenType::Ident(_), TokenType::Ident(_)) => {
                    self.next_token();
                    true
                }
                _ if t.token_type == token_type => {
                    self.next_token();
                    true
                }
                _ => {
                    self.peek_error(token_type);
                    false
                }
            },
            None => {
                self.peek_error(token_type);
                false
            }
        }
    }
}
