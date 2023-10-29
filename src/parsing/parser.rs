use super::tokens::{Token, TokenType};
use super::expressions::*;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn equality(&mut self) -> Box<dyn Expr> {
        let mut expr = self.comparison();

        while self.matching(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
    }

    fn matching(&mut self, types: Vec<TokenType>) -> bool {
        for t in types.iter(){
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
        let mut expr = self.term();
        let token_types = vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL
        ];

        while self.matching(token_types) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
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
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn term(&mut self) -> Box<dyn Expr> {
        let mut expr = self.factor();
        let token_types = vec![
            TokenType::MINUS,
            TokenType::PLUS
        ];

        while self.matching(token_types) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut expr = self.unary();
        let token_types = vec![
            TokenType::SLASH,
            TokenType::STAR
        ];

        while self.matching(token_types) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        let token_types = vec![
            TokenType::BANG,
            TokenType::MINUS
        ];
        if self.matching(token_types) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Unary::new(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expr> {
        if self.matching(vec![TokenType::FALSE]) {
            return Box::new(Literal::Bool(false));
        }
        if self.matching(vec![TokenType::TRUE]) {
            return Box::new(Literal::Bool(true));
        }
        if self.matching(vec![TokenType::NIL]) {
            return Box::new(Literal::Nil);
        }
        if self.matching(vec![TokenType::NUMBER]) {
            let val = self.previous().int.unwrap();
            return Box::new(Literal::Int(val));
        }
        if self.matching(vec![TokenType::STRING]) {
            let s = self.previous().string.unwrap();
            return Box::new(Literal::S(s));
        }
        if self.matching(vec![TokenType::LEFT_PAREN]) {
           let expr = self.expression(); 
           self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.".to_owned());
           return Box::new(Grouping::new(expr));
        }
        Box::new(Literal::Nil)
    }

    fn consume(&mut self, variant: TokenType, msg: String) -> Result<Token, String> {
        if self.check(variant) {
            return Ok(self.advance());
        } 
        Err(msg) 
    }
}
