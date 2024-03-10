use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::lexer;

pub trait Node {
    fn token_literal(&self) -> Option<String>;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Expression: Node + fmt::Debug {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Rc<RefCell<dyn Statement>>>,
}

#[derive(Debug)]
pub struct Identifier {
    pub token: lexer::Token,
    pub value: String,
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: lexer::Token,
    pub name: Option<Identifier>,
    pub value: Option<Rc<RefCell<dyn Expression>>>,
}

impl Node for Program {
    fn token_literal(&self) -> Option<String> {
        self.statements
            .first()
            .map(|s| s.as_ref().borrow().token_literal())
            .flatten()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
