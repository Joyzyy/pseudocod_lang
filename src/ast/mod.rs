use crate::lexer;

pub trait Node {
    fn token_literal(&self) -> Option<&str>;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

pub struct Identifier {
    pub token: lexer::Token,
    pub value: String,
}

pub struct LetStatement {
    pub token: lexer::Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for Program {
    fn token_literal(&self) -> Option<&str> {
        self.statements.first().map(|s| s.token_literal()).flatten()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.literal)
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.literal)
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}
