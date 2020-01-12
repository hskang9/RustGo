use crate::token::*;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        return l;
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn new_token(&mut self, r#type: TokenType, literal: String) -> Token {
        Token { r#type, literal }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ch: char = self.ch.clone();
        let tok = match ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    let token_literal = [ch.clone().to_string(), ch.clone().to_string()].concat();
                    self.new_token(EQ, token_literal)
                } else {
                    self.new_token(ASSIGN, self.ch.to_string())
                }
            }
            ';' => self.new_token(SEMICOLON, self.ch.to_string()),
            '(' => self.new_token(LPAREN, self.ch.to_string()),
            ')' => self.new_token(RPAREN, self.ch.to_string()),
            ',' => self.new_token(COMMA, self.ch.to_string()),
            '+' => self.new_token(PLUS, self.ch.to_string()),
            '-' => self.new_token(MINUS, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    let token_literal = [ch.clone().to_string(), '='.to_string()].concat();
                    self.new_token(NOT_EQ, token_literal)
                } else {
                    self.new_token(BANG, self.ch.to_string())
                }
            }
            '/' => self.new_token(SLASH, self.ch.to_string()),
            '*' => self.new_token(ASTERISK, self.ch.to_string()),
            '<' => self.new_token(LT, self.ch.to_string()),
            '>' => self.new_token(GT, self.ch.to_string()),
            '{' => self.new_token(LBRACE, self.ch.to_string()),
            '}' => self.new_token(RBRACE, self.ch.to_string()),
            '\0' => self.new_token(EOF, self.ch.to_string()),
            _ => {
                if is_letter(ch.clone()) {
                    let token_literal = self.read_identifier().to_string();
                    self.position += token_literal.len();
                    return self.new_token(lookup_ident(&token_literal), token_literal);
                } else if is_digit(ch.clone()) {
                    let token_literal = self.read_number().to_string();
                    return self.new_token(INT, token_literal);
                } else {
                    return self.new_token(ILLEGAL, ch.to_string());
                }
            }
        };
        self.read_char();
        return tok;
    }
    pub fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        return &self.input[position..self.position];
    }

    pub fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char()
        }
        &self.input[position..self.position]
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

pub fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
