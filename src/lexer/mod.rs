use crate::token::*;

#[derive(Debug, Clone)]
pub struct Lexer {
    /// input string of code
    pub input: Vec<u8>,
    /// current read position of lexer
    pub position: usize,
    /// position to read in string
    pub read_position: usize,
    /// next character to read
    pub ch: u8,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: b' ',
        };
        l.read_char();
        return l;
    }

    pub fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            b'\0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn new_token(&mut self, r#type: TokenType, literal: Vec<u8>) -> Token {
        Token { r#type, literal }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ch: u8 = self.ch.clone();
        let tok = match ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    let token_literal = [ch.clone(), ch.clone()];
                    self.new_token(EQ, token_literal.to_vec())
                } else {
                    self.new_token(ASSIGN, [self.ch].to_vec())
                }
            }
            b';' => self.new_token(SEMICOLON, [self.ch].to_vec()),
            b'(' => self.new_token(LPAREN, [self.ch].to_vec()),
            b')' => self.new_token(RPAREN,[self.ch].to_vec()),
            b',' => self.new_token(COMMA, [self.ch].to_vec()),
            b'+' => self.new_token(PLUS, [self.ch].to_vec()),
            b'-' => self.new_token(MINUS, [self.ch].to_vec()),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    let token_literal = [ch.clone(), b'='];
                    self.new_token(NOT_EQ, token_literal.to_vec())
                } else {
                    self.new_token(BANG, [self.ch].to_vec())
                }
            }
            b'/' => self.new_token(SLASH, [self.ch].to_vec()),
            b'*' => self.new_token(ASTERISK, [self.ch].to_vec()),
            b'<' => self.new_token(LT, [self.ch].to_vec()),
            b'>' => self.new_token(GT, [self.ch].to_vec()),
            b'{' => self.new_token(LBRACE, [self.ch].to_vec()),
            b'}' => self.new_token(RBRACE, [self.ch].to_vec()),
            b'\0' => self.new_token(EOF, [self.ch].to_vec()),
            _ => {
                if is_letter(ch.clone()) {
                    let token_literal = self.read_identifier();
                    self.position += token_literal.len();
                    return self.new_token(lookup_ident(&token_literal), token_literal.to_vec());
                } else if is_digit(ch.clone()) {
                    let token_literal = self.read_number();
                    return self.new_token(INT, token_literal.to_vec());
                } else {
                    self.read_char();
                    return self.new_token(ILLEGAL, [ch].to_vec());
                }
            }
        };
        self.read_char();
        return tok;
    }
    pub fn read_identifier(&mut self) -> Vec<u8> {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        return self.input[position..self.position].to_vec();
    }

    pub fn read_number(&mut self) -> Vec<u8> {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char()
        }
        return self.input[position..self.position].to_vec();
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            if self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

pub fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

pub fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}
