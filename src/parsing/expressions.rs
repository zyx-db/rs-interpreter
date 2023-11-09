use crate::errors::err::RuntimeErr;

use super::tokens::Token;
use super::visitor::Visitor;

pub trait Expr {
    fn accept_s(&self, p: &dyn Visitor<String>) -> Result<String, RuntimeErr>;
    fn accept_l(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr>;
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
pub enum Literal {
    S(String),
    Int(f64),
    Bool(bool),
    Nil
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => {
                a == b
            }
            (Literal::S(a), Literal::S(b)) => {
                a == b
            }
            (Literal::Bool(a), Literal::Bool(b)) => {
                a == b
            }
            (Literal::Nil, Literal::Nil) => {true}
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
    fn accept_s(&self, p: &dyn Visitor<String>) -> Result<String, RuntimeErr> {
        return p.visit_binary(self);
    }
    fn accept_l(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        return p.visit_binary(self);
    }
}
impl Expr for Grouping{
    fn accept_s(&self, p: &dyn Visitor<String>) -> Result<String, RuntimeErr> {
        return p.visit_grouping(self);
    }
    fn accept_l(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        return p.visit_grouping(self);
    }
}
impl Expr for Literal{
    fn accept_s(&self, p: &dyn Visitor<String>) -> Result<String, RuntimeErr> {
        return p.visit_literal(self);
    }
    fn accept_l(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        return p.visit_literal(self);
    }
}
impl Expr for Unary{
    fn accept_s(&self, p: &dyn Visitor<String>) -> Result<String, RuntimeErr> {
        return p.visit_unary(self);
    }
    fn accept_l(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        p.visit_unary(self)
    }
}
