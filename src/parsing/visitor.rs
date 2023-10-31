use std::fmt::format;

use super::expressions::{Binary, Grouping, Literal, Unary, Expr};

pub trait Visitor<T>{
    fn visit_binary(&self, b: &Binary) -> T;
    fn visit_grouping(&self, g: &Grouping) -> T;
    fn visit_literal(&self, l: &Literal) -> T;
    fn visit_unary(&self, u: &Unary) -> T;
}

pub struct Printer {}

impl Printer{
    pub fn new() -> Self{
        Printer {  }
    }
    pub fn print(&self, e: Box<dyn Expr>) -> String{
        // "".to_owned()
        e.accept_s(self)
    }
}

impl Visitor<String> for Printer {
    fn visit_binary(&self, b: &Binary) -> String{
        eprintln!("visiting Binary");
        let op = b.operator.lexeme.clone();
        let left = b.left.accept_s(self);
        let right = b.right.accept_s(self);
        return format!("{} {} {}", op, left, right);
    }
    fn visit_grouping(&self, g: &Grouping) -> String{
        eprintln!("visiting Grouping");
        let val = g.expr.accept_s(self);
        return format!("({})", val);
    }
    fn visit_literal(&self, l: &Literal) -> String{
        eprintln!("visiting Literal");
        match l {
            Literal::S(s) => {
                return s.clone();
            }
            Literal::Int(i) => {
                return format!("{}", i);
            }
            Literal::Bool(b) => {
                return format!("{}", b);
            }
            _ => {return "None".to_owned();}
        }
    }
    fn visit_unary(&self, u: &Unary) -> String{
        eprintln!("visiting Unary");
        let op = u.operator.lexeme.clone();
        let right = u.right.accept_s(self);
        return format!("({} {})", op, right);
    }
}
