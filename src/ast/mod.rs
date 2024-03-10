use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::lexer;

pub trait Node {
    fn token_literal(&self) -> Option<String>;
    fn string(&self) -> String {
        String::new()
    }
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

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: lexer::Token,
    pub return_value: Option<Rc<RefCell<dyn Expression>>>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: lexer::Token,
    pub expression: Option<Rc<RefCell<dyn Expression>>>,
}

#[derive(Debug)]
pub struct IntegralLiteral {
    pub token: lexer::Token,
    pub value: i64,
}

impl Node for Program {
    fn token_literal(&self) -> Option<String> {
        self.statements
            .first()
            .map(|s| s.as_ref().borrow().token_literal())
            .flatten()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(&s.as_ref().borrow().string());
        }
        out
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token.literal);
        out.push_str(" ");
        out.push_str(&self.name.as_ref().unwrap().value);
        out.push_str(" = ");
        if let Some(value) = &self.value {
            out.push_str(&value.as_ref().borrow().string());
        }
        out.push_str(";");
        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token.literal);
        out.push_str(" ");
        if let Some(value) = &self.return_value {
            out.push_str(&value.as_ref().borrow().string());
        }
        out.push_str(";");
        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }

    fn string(&self) -> String {
        if let Some(expression) = &self.expression {
            expression.as_ref().borrow().string()
        } else {
            String::new()
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for IntegralLiteral {
    fn token_literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for IntegralLiteral {
    fn expression_node(&self) {}
}
