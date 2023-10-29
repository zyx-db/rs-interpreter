use super::tokens::{Token, TokenType};
use super::expressions::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Option<Box<dyn Expr>> {
       return self.expression();
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Option<Box<dyn Expr>> {
        return self.equality();
    }

    fn equality(&mut self) -> Option<Box<dyn Expr>> {
        let val = self.comparison();

        let mut expr = val.unwrap();

        while self.matching(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let tmp_right = self.comparison();

            let right = tmp_right.unwrap();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Some(expr)
    }

    fn matching(&mut self, types: &Vec<TokenType>) -> bool {
        for t in types.iter(){
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn comparison(&mut self) -> Option<Box<dyn Expr>> {
        let val = self.term();
        let mut expr = val.unwrap();
        let token_types = vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL
        ];

        while self.matching(&token_types) {
            let operator = self.previous();
            let tmp_right = self.term();

            let right = tmp_right.unwrap();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Some(expr)
    }

    fn check(&mut self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().variant == t
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().variant == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn term(&mut self) -> Option<Box<dyn Expr>> {
        let val = self.factor();

        let mut expr = val.unwrap();
        let token_types = vec![
            TokenType::MINUS,
            TokenType::PLUS
        ];

        while self.matching(&token_types) {
            let operator = self.previous();
            let tmp_right = self.factor();

            let right = tmp_right.unwrap();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Some(expr)
    }

    fn factor(&mut self) -> Option<Box<dyn Expr>> {
        let val = self.unary(); 

        let mut expr = val.unwrap();
        let token_types = vec![
            TokenType::SLASH,
            TokenType::STAR
        ];

        while self.matching(&token_types) {
            let operator = self.previous();
            let tmp_right = self.unary();

            let right = tmp_right.unwrap();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Some(expr)
    }

    fn unary(&mut self) -> Option<Box<dyn Expr>> {
        let token_types = vec![
            TokenType::BANG,
            TokenType::MINUS
        ];
        if self.matching(&token_types) {
            let operator = self.previous();
            if let Some(right) = self.unary(){
                return Some(Box::new(Unary::new(operator, right)));
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<Box<dyn Expr>> {
        if self.matching(&vec![TokenType::FALSE]) {
            return Some(Box::new(Literal::Bool(false)));
        }
        if self.matching(&vec![TokenType::TRUE]) {
            return Some(Box::new(Literal::Bool(true)));
        }
        if self.matching(&vec![TokenType::NIL]) {
            return Some(Box::new(Literal::Nil));
        }
        if self.matching(&vec![TokenType::NUMBER]) {
            let val = self.previous().int.unwrap();
            return Some(Box::new(Literal::Int(val)));
        }
        if self.matching(&vec![TokenType::STRING]) {
            let s = self.previous().string.unwrap();
            return Some(Box::new(Literal::S(s)));
        }
        if self.matching(&vec![TokenType::LEFT_PAREN]) {
           let expr = self.expression().unwrap(); 
           self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.".to_owned());
           return Some(Box::new(Grouping::new(expr)));
        }
        None
    }

    fn consume(&mut self, variant: TokenType, msg: String) -> Option<Token> {
        if self.check(variant) {
            return Some(self.advance());
        } 
        None
    }
}
