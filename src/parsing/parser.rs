use crate::errors::err::error;

use super::statements::*;
use super::tokens::{Token, TokenType};
use super::expressions::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Option<Vec<Box<dyn Stmt>>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let cur = self.declaration();
            if cur.is_some() {
               statements.push(cur.unwrap());
            }
        }

        return Some(statements)
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn declaration(&mut self) -> Option<Box<dyn Stmt>> {
        if self.matching(&vec![TokenType::IDENTIFIER]){
            let res = self.var_declaration();
            if res.is_none() {
                self.synchronize();
                return None;
            }
            return res;
        }

        let res = self.statement();
        if res.is_none() {
            self.synchronize();
            return None;
        }
        res
    }

    fn var_declaration(&mut self) -> Option<Box<dyn Stmt>> {
        let name = self.consume(TokenType::IDENTIFIER, "expect var name".to_owned());
        if name.is_none() {
            return None;
        }

        let mut init: Option<Box<dyn Expr>> = None;
        if self.matching(&vec![TokenType::EQUAL]) {
            init = self.expression(); 
        }

        if init.is_none() {
            return None;
        }

        self.consume(TokenType::SEMICOLON, "expecting ';' after variable declaration".to_owned());
        return Some(Box::new(Dec::new(name.unwrap(), init.unwrap())));
    }

    fn statement(&mut self) -> Option<Box<dyn Stmt>> {
        if self.matching(&vec![TokenType::PRINT]) {
            return self.print_statement();
        }
        self.expression_stmt()
    }

    fn expression(&mut self) -> Option<Box<dyn Expr>> {
        return self.equality();
    }

    fn equality(&mut self) -> Option<Box<dyn Expr>> {
        let val = self.comparison();
        if val.is_none() {return None;}

        let mut expr = val.unwrap();

        while self.matching(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let tmp_right = self.comparison();
            if tmp_right.is_none(){ return None;}

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
        if val.is_none() {return None;}

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
            if tmp_right.is_none(){ return None;}

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
        if val.is_none() {return None;}

        let mut expr = val.unwrap();
        let token_types = vec![
            TokenType::MINUS,
            TokenType::PLUS
        ];

        while self.matching(&token_types) {
            let operator = self.previous();
            let tmp_right = self.factor();
            if tmp_right.is_none() {return None;}

            let right = tmp_right.unwrap();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Some(expr)
    }

    fn factor(&mut self) -> Option<Box<dyn Expr>> {
        let val = self.unary(); 
        if val.is_none(){ return None;}

        let mut expr = val.unwrap();
        let token_types = vec![
            TokenType::SLASH,
            TokenType::STAR
        ];

        while self.matching(&token_types) {
            let operator = self.previous();
            let tmp_right = self.unary();
            if tmp_right.is_none(){return None;}

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
            return Some(Box::new(
                    Literal::new(
                        self.previous(),
                        Value::Bool(false)
                    )
                ));
        }
        if self.matching(&vec![TokenType::TRUE]) {
            return Some(Box::new(
                    Literal::new(
                        self.previous(),
                        Value::Bool(true)
                        )
                    ));
        }
        if self.matching(&vec![TokenType::NIL]) {
            return Some(Box::new(
                    Literal::new(
                        self.previous(),
                        Value::Nil
                        )
                    ));
        }
        if self.matching(&vec![TokenType::NUMBER]) {
            let val = self.previous().int.unwrap();
            return Some(Box::new(
                    Literal::new(
                        self.previous(),
                        Value::Int(val)
                        )
                    ));
        }
        if self.matching(&vec![TokenType::STRING]) {
            let s = self.previous().string.unwrap();
            return Some(Box::new(
                    Literal::new(
                        self.previous(),
                        Value::S(s)
                        )
                    ));
        }
        if self.matching(&vec![TokenType::LEFT_PAREN]) {
           let val = self.expression(); 
           if val.is_none(){ return None; }

           let expr = val.unwrap();
           self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.".to_owned());
           return Some(Box::new(Grouping::new(expr)));
        }

        None
    }

    fn print_statement(&mut self) -> Option<Box<dyn Stmt>> {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "expecting ';' after expr.".to_owned());
    
        if expr.is_some() {
            return Some(Box::new(Print::new(expr.unwrap())));
        }

        None
    }

    fn expression_stmt(&mut self) -> Option<Box<dyn Stmt>> {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "expecting ';' after expr".to_owned());

        if expr.is_some() {
            return Some(Box::new(ExprStmt::new(expr.unwrap())));
        }

        None
    }

    fn consume(&mut self, variant: TokenType, msg: String) -> Option<Token> {
        if self.check(variant) {
            return Some(self.advance());
        } 
        error(self.peek().line, &msg);
        None
    }

    fn synchronize(&mut self){
        self.advance();
        while !self.is_at_end() {
            if self.previous().variant == TokenType::SEMICOLON{
                return;
            }
            
            match self.peek().variant{
                TokenType::CLASS => {return;}
                TokenType::DEF => {return;}
                TokenType::VAR => {return;}
                TokenType::FOR=> {return;}
                TokenType::IF => {return;}
                TokenType::WHILE => {return;}
                TokenType::PRINT => {return;}
                TokenType::RETURN => {return;}
                _ => {}
            }

            self.advance();
        }
    }
}
