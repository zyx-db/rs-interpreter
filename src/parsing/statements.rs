use crate::errors::err::RuntimeErr;

use super::{expressions::{Expr, Literal}, tokens::Token, visitor::Visitor};

pub trait Stmt {
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr>;
}

pub struct Dec {
    expr: Box<dyn Expr>,
}

pub struct Print {
    pub expr: Box<dyn Expr>,
}

pub struct ExprStmt {
    pub expr: Box<dyn Expr>,
}

impl Print{
    pub fn new(expr: Box<dyn Expr>) -> Self {
        Print { expr }
    }
}

impl ExprStmt {
    pub fn new(expr: Box<dyn Expr>) -> Self {
        ExprStmt { expr }
    }
}

// impl Stmt for Dec{
//     fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
//         p.visit_d
//     }
// }
impl Stmt for Print{
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        p.visit_print_stmt(self)
    }
}
impl Stmt for ExprStmt{
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        p.visit_expr_stmt(self)
    }
}
