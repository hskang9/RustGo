
use std::collections::HashMap;

pub type TokenType = &'static str;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String
}


pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const  IDENT: &str = "IDENT";
pub const  INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Logic
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";


// Keywords
pub const FUNCTION: &str = "FUNCITON";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";


// Keyword HashMap
lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", FUNCTION);
        m.insert("let", LET);
        m.insert("true", TRUE);
        m.insert("false", FALSE);
        m.insert("if", IF);
        m.insert("else", ELSE);
        m.insert("return", RETURN);
        m
    };
}

pub fn lookup_ident(ident: &str) -> TokenType {
    if KEYWORDS.contains_key(ident) {
        return KEYWORDS[ident];
    } else {
        return IDENT;
    }
}



