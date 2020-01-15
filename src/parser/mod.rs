use crate::ast::*;
use crate::lexer::*;
use crate::token::*;
use iota::iota;
use std::collections::HashMap;

iota!{
    const LOWEST: u8 = 1;
    ,EQUALS
    ,LESSGREATER
    ,SUM
    ,PRODUCT
    ,PREFIX
    ,CALL
}

type PrefixParseFn = fn() -> Box<dyn Expression>;
type InfixParseFn = fn(Box<dyn Expression>) -> Box<dyn Expression>;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            lexer: l,
            cur_token: None,
            peek_token: None,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new()
        };
        let callback: PrefixParseFn = p.parse_identifier;
        p.register_prefix(IDENT, *&callback);
        p.next_token();
        p.next_token();
        return p;
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    pub fn register_prefix(&mut self, token: TokenType, prefix_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token, prefix_fn);
    }

    pub fn register_infix(&mut self, token: TokenType, infix_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token, infix_fn);
    }

    pub fn parse_identifier(&self) -> Box<dyn Expression> {
        return Box::new(Identifier{token: self.cur_token.clone(), value: self.cur_token.clone().unwrap().literal});
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token.clone().unwrap().r#type != EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        return program;
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.clone().unwrap().r#type {
            LET => {
                let stmt = self.parse_let_statement();
                if stmt.is_some() {
                    Some(Box::new(stmt.unwrap()))
                } else {
                    None
                }
            }
            RETURN => {
                let stmt = self.parse_return_statement();
                if stmt.is_some() {
                    Some(Box::new(stmt.unwrap()))
                } else {
                    None
                }
            }
            _ => {
                let stmt = self.parse_expression_statement();
                if stmt.is_some() {
                    Some(Box::new(stmt.unwrap()))
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone().unwrap(),
            name: None,
            value: None,
        };

        if !self.expect_peek(IDENT) {
            return None;
        }

        stmt.name = Some(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.clone().unwrap().literal,
        });

        if !self.expect_peek(ASSIGN) {
            return None;
        }

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.is_cur_token(SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }

    pub fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone().unwrap(),
            return_value: None,
        };

        self.next_token();

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.is_cur_token(SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }

    pub fn parse_expression_statement(&self) -> Option<ExpressionStatement> {
        let stmt = ExpressionStatement{
            token: self.cur_token.clone().unwrap(),
            expression: self.parse_expression(LOWEST)
        };

        if self.is_peek_token(SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }

    
    pub fn parse_expression(&self, precedence: u8) -> Option<Box<dyn Expression>> {
        if self.prefix_parse_fns.contains_key(self.cur_token.clone().unwrap().r#type) {
            return None;
        }
        let prefix = self.prefix_parse_fns[self.cur_token.clone().unwrap().r#type];
        let left_exp = prefix();
        return Some(left_exp);
    }
    

    pub fn is_cur_token(&self, t: TokenType) -> bool {
        return self.cur_token.clone().unwrap().r#type == t;
    }

    pub fn is_peek_token(&self, t: TokenType) -> bool {
        return self.peek_token.clone().unwrap().r#type == t;
    }

    pub fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.is_peek_token(t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    pub fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t,
            self.peek_token.clone().unwrap().r#type
        );
        self.errors.push(msg);
    }
}
