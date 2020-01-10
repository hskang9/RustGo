use crate::lexer::*;
use crate::token::*;
use crate::ast::*;


#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>
}

impl Parser {
    pub fn new(&mut self, l: Lexer) -> Parser {
        let mut p = Parser{
            lexer: l,
            cur_token: None,
            peek_token: None,
            errors: vec!{}
        };
        p.next_token();
        p.next_token();
        return p;
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    // TODO: return Program<T>
    pub fn parse_program(&mut self) -> Option<Program>  {
        let mut program = Program{
            statements: vec!{}
        };

        while self.cur_token.clone().unwrap().r#type != EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        return Some(program);
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.clone().unwrap().r#type {
            LET => {
                let a = self.parse_let_statement();
                Some(Box::new(a.unwrap()))
            },
            _ => None
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement{
            token: self.cur_token.clone().unwrap(),
            name: None,
            value: None
        };


        stmt.name = Some(Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.clone().unwrap().literal
        });

        if !self.expect_peek(IDENT) {
            return None;
        }

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.is_cur_token(SEMICOLON) {
            self.next_token();
        }

        
        return Some(stmt);
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
            return false;
        }
    }

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    pub fn peek_error(&mut self, t: TokenType) {
        let msg = format!("expected next token to be {:?}, got {:?} instead", t, self.peek_token.clone().unwrap().r#type);
        self.errors.push(msg);
    }
}