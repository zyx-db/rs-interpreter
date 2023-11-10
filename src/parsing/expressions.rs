use crate::errors::err::RuntimeErr;

use super::tokens::Token;
use super::visitor::Visitor;

pub trait Expr {
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr>;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self{
        Binary { left, operator, right}
    }
}

pub struct Grouping {
    pub expr: Box<dyn Expr>,
}

impl Grouping {
    pub fn new(expr: Box<dyn Expr>) -> Self {
        Grouping { expr }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    S(String),
    Int(f64),
    Bool(bool),
    Nil
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub val: Value,
    token: Token,
}

impl Literal {
    pub fn new(token: Token, val: Value) -> Self {
        Literal { val, token }
    }
    pub fn default() -> Self {
        return Literal::new(
            Token::new(super::tokens::TokenType::NIL, "NIL".to_owned(), 1),
            Value::Nil
        )
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                a == b
            }
            (Value::S(a), Value::S(b)) => {
                a == b
            }
            (Value::Bool(a), Value::Bool(b)) => {
                a == b
            }
            (Value::Nil, Value::Nil) => {true}
            (_, _) => {false}
        } 
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Unary { operator, right }
    }
}

impl Expr for Binary{
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        return p.visit_binary(self);
    }
}
impl Expr for Grouping{
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        return p.visit_grouping(self);
    }
}
impl Expr for Literal{
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        return p.visit_literal(self);
    }
}
impl Expr for Unary{
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        p.visit_unary(self)
    }
}
