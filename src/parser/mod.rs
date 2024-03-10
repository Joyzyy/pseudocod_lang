use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{
    Expression, ExpressionStatement, Identifier, IntegralLiteral, LetStatement, Program,
    ReturnStatement, Statement,
};
use crate::lexer::{Lexer, Token, TokenType};

mod tests;

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug)]
struct Parser {
    lexer: Rc<RefCell<Lexer>>,
    errors: Vec<String>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    prefix_parse_fns: HashMap<TokenType, fn(&Parser) -> Option<Rc<RefCell<dyn Expression>>>>,
    infix_parse_fns:
        HashMap<TokenType, fn(Rc<RefCell<dyn Expression>>) -> Option<Rc<RefCell<dyn Expression>>>>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: Rc::new(RefCell::new(lexer)),
            errors: Vec::new(),
            current_token: None,
            peek_token: None,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.next_token();
        parser.next_token();

        parser.register_prefix(TokenType::Ident(String::new()), Parser::parse_identifier);
        parser.register_prefix(
            TokenType::Int(String::new()),
            Parser::parse_integral_literal,
        );

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
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
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

    fn parse_return_statement(&mut self) -> Option<Rc<RefCell<dyn Statement>>> {
        let mut statement = ReturnStatement {
            token: self.current_token.as_ref().unwrap().clone(),
            return_value: None,
        };

        self.next_token();

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Rc::new(RefCell::new(statement)))
    }

    fn parse_expression_statement(&mut self) -> Option<Rc<RefCell<dyn Statement>>> {
        let statement = ExpressionStatement {
            token: self.current_token.as_ref().unwrap().clone(),
            expression: self.parse_expression(Precedence::Lowest),
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Rc::new(RefCell::new(statement)))
    }

    fn parse_expression(&self, precedence: Precedence) -> Option<Rc<RefCell<dyn Expression>>> {
        let prefix = self
            .prefix_parse_fns
            .get(&self.current_token.as_ref().unwrap().token_type);

        if prefix.is_none() {
            return None;
        }

        let left_expression = prefix.unwrap()(self);

        left_expression
    }

    fn parse_identifier(&self) -> Option<Rc<RefCell<dyn Expression>>> {
        Some(Rc::new(RefCell::new(Identifier {
            token: self.current_token.as_ref().unwrap().clone(),
            value: self.current_token.as_ref().unwrap().literal.clone(),
        })))
    }

    fn parse_integral_literal(&self) -> Option<Rc<RefCell<dyn Expression>>> {
        let token = self.current_token.as_ref()?;
        Some(Rc::new(RefCell::new(IntegralLiteral {
            token: token.clone(),
            value: token.literal.parse::<i64>().unwrap_or(0),
        })))
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

    fn register_prefix(
        &mut self,
        token_type: TokenType,
        func: fn(&Parser) -> Option<Rc<RefCell<dyn Expression>>>,
    ) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    fn register_infix(
        &mut self,
        token_type: TokenType,
        func: fn(Rc<RefCell<dyn Expression>>) -> Option<Rc<RefCell<dyn Expression>>>,
    ) {
        self.infix_parse_fns.insert(token_type, func);
    }
}
