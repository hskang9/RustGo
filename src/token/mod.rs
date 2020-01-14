
use std::collections::HashMap;

pub type TokenType = &'static [u8];

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: Vec<u8>
}


pub const ILLEGAL: &[u8] = b"ILLEGAL";
pub const EOF: &[u8] = b"EOF";

// Identifiers + literals
pub const  IDENT: &[u8] = b"IDENT";
pub const  INT: &[u8] = b"INT";

// Operators
pub const ASSIGN: &[u8] = b"=";
pub const PLUS: &[u8] = b"+";
pub const MINUS: &[u8] = b"-";
pub const BANG: &[u8] = b"!";
pub const ASTERISK: &[u8] = b"*";
pub const SLASH: &[u8] = b"/";

pub const LT: &[u8] = b"<";
pub const GT: &[u8] = b">";

// Delimiters
pub const COMMA: &[u8] = b",";
pub const SEMICOLON: &[u8] = b";";
pub const LPAREN: &[u8] = b"(";
pub const RPAREN: &[u8] = b")";
pub const LBRACE: &[u8] = b"{";
pub const RBRACE: &[u8] = b"}";

// Logic
pub const EQ: &[u8] = b"==";
pub const NOT_EQ: &[u8] = b"!=";


// Keywords
pub const FUNCTION: &[u8] = b"FUNCITON";
pub const LET: &[u8] = b"LET";
pub const TRUE: &[u8] = b"TRUE";
pub const FALSE: &[u8] = b"FALSE";
pub const IF: &[u8] = b"IF";
pub const ELSE: &[u8] = b"ELSE";
pub const RETURN: &[u8] = b"RETURN";


// Keyword HashMap
lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static [u8], TokenType> = {
        let mut m: HashMap<&'static [u8], TokenType> = HashMap::new();
        m.insert(b"return", RETURN);
        m.insert(b"fn", FUNCTION);
        m.insert(b"let", LET);
        m.insert(b"true", TRUE);
        m.insert(b"false", FALSE);
        m.insert(b"if", IF);
        m.insert(b"else", ELSE);
        m
    };
}

pub fn lookup_ident(ident: &[u8]) -> TokenType {
    if KEYWORDS.contains_key(ident) {
        return KEYWORDS[ident];
    } else {
        return IDENT;
    }
}



