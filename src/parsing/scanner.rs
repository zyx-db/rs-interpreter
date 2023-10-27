use std::collections::HashMap;

use super::tokens::{Token, TokenType};
use super::keywords::keywords_map;
use crate::errors::err;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
    has_error: bool,
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
            keywords: keywords_map()
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ()> {
        self.has_error = false;
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
            if self.has_error {
                return Err(());
            }
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_owned(), self.line));
        return Ok(self.tokens.clone());
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn scan_token(&mut self) {
        let c = self.source.as_bytes()[self.current as usize] as char;
        self.current += 1;
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token = {
                    if self.match_expected_char('=') {
                        TokenType::BANG_EQUAL
                    } else {
                        TokenType::BANG
                    }
                };
                self.add_token(token);
            }
            '=' => {
                let token = {
                    if self.match_expected_char('=') {
                        TokenType::EQUAL_EQUAL
                    } else {
                        TokenType::EQUAL
                    }
                };
                self.add_token(token);
            }
            '<' => {
                let token = {
                    if self.match_expected_char('=') {
                        TokenType::LESS_EQUAL
                    } else {
                        TokenType::LESS
                    }
                };
                self.add_token(token);
            }
            '>' => {
                let token = {
                    if self.match_expected_char('=') {
                        TokenType::GREATER_EQUAL
                    } else {
                        TokenType::GREATER
                    }
                };
                self.add_token(token);
            }
            '/' => {
                if self.match_expected_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            _ => {
                if Scanner::is_digit(c){
                    self.number();
                }
                else if Scanner::is_alpha(c){
                    self.identifier();
                }
                else {
                    err::error(self.line, &"Unexpected character.".to_owned());
                    self.has_error = true;
                }
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            err::error(self.line, &"Unterminated string.".to_owned());
            self.has_error = true;
            return;
        }

        self.advance();

        let value = self.source[(self.start + 1) as usize..(self.current - 1) as usize].to_owned();
        self.add_string_token(TokenType::STRING, value);
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {self.advance();}

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()){
            self.advance();

            while Scanner::is_digit(self.peek()) {
               self.advance(); 
            }
        }

        let string_value = self.source[self.start as usize .. self.current as usize].to_owned();
        let value: f64 = string_value.parse().unwrap();

        self.add_int_token(TokenType::NUMBER, value)
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()){
            self.advance();
        } 

        let text = self.source[self.start as usize .. self.current as usize].to_owned();
        if self.keywords.contains_key(&text){
            let token_t = self.keywords.get(&text).unwrap();
            self.add_token(token_t.clone());
        }
        else {
            self.add_token(TokenType::IDENTIFIER);
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current as usize] as char
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) as usize >= self.source.len(){
            return '\0';
        }
        self.source.as_bytes()[(self.current + 1) as usize] as char
    }

    fn advance(&mut self) -> char {
        let res = self.source.as_bytes()[self.current as usize] as char;
        self.current += 1;
        res
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self.source[self.start as usize..self.current as usize].to_owned();
        self.tokens.push(Token::new(token, text, self.line));
    }

    fn add_string_token(&mut self, token: TokenType, string: String) {
        let text = self.source[self.start as usize..self.current as usize].to_owned();
        self.tokens
            .push(Token::new_str(token, text, self.line, string));
    }

    fn add_int_token(&mut self, token: TokenType, int: f64) {
        let text = self.source[self.start as usize..self.current as usize].to_owned();
        self.tokens
            .push(Token::new_int(token, text, self.line, int));
    }

    fn match_expected_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.as_bytes()[self.current as usize] as char;
        if c != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9' 
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }
}
