use crate::errors::err::RuntimeErr;
use crate::runtime::interpreter::Visitor;

use super::{expressions::{Expr, Literal}, tokens::Token};

pub trait Stmt {
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr>;
}

pub struct Dec {
    name: Token,
    expr: Box<dyn Expr>,
}

pub struct Print {
    pub expr: Box<dyn Expr>,
}

pub struct ExprStmt {
    pub expr: Box<dyn Expr>,
}

impl Dec {
    pub fn new(name: Token, expr: Box<dyn Expr>) -> Self {
        Dec { name, expr }
    }
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

impl Stmt for Dec{
    fn accept(&self, p: &dyn Visitor<Literal>) -> Result<Literal, RuntimeErr> {
        p.visit_declaration(self)
    }
}
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
