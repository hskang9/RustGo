use crate::token::*;

pub trait Node {
    fn token_literal(&self) -> Vec<u8>;
}

pub trait Statement {
    fn token_literal(&self) -> Vec<u8>;
    fn statement_node(&self);
}

pub trait Expression {
    fn token_literal(&self) -> Vec<u8>;
    fn expression_node(&self);
}

pub trait Draw {
    fn draw(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for Program {
    fn token_literal(&self) -> Vec<u8> {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return [].to_vec();
        }
    }
    fn statement_node(&self) {}
}

// Let Statment
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn token_literal(&self) -> Vec<u8> {
        return self.token.literal.clone();
    }
    fn statement_node(&self) {}
}

#[derive(Clone, PartialEq)]
pub struct Identifier {
    pub token: Option<Token>,
    pub value: Vec<u8>,
}

impl Expression for Identifier {
    fn token_literal(&self) -> Vec<u8> {
        return self.token.clone().unwrap().literal;
    }
    fn expression_node(&self) {}
}

// Return statement
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatement {
    fn token_literal(&self) -> Vec<u8> {
        return self.token.literal.clone();
    }
    fn statement_node(&self) {}
}

// Expression statement
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
    fn token_literal(&self) -> Vec<u8> {
        return self.token.literal.clone();
    }
    fn statement_node(&self) {}
}
