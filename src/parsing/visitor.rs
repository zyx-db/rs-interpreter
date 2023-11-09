use super::expressions::{Binary, Grouping, Literal, Unary, Expr};
use super::tokens::TokenType;

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

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {  }
    }

    pub fn evaluate(&self, e: Box<dyn Expr>) -> Literal {
        e.accept_l(self)
    }

    fn is_truthy(&self, expr: &Literal) -> bool {
        match expr {
           Literal::Nil => {return false;} 
           Literal::Bool(res) => {return *res;}
           _ => {return true;}
        }
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_binary(&self, b: &Binary) -> Literal {
        let left = b.left.accept_l(self);
        let right = b.right.accept_l(self);

        match (&b.operator.variant, left, right) {
            // arithmetic
            (TokenType::PLUS, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Int(a + b);
            }
            (TokenType::SLASH, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Int(a / b); 
            }
            (TokenType::MINUS, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Int(a - b);
            }
            (TokenType::STAR, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Int(a * b);
            }

            // string concatenation
            (TokenType::PLUS, Literal::S(a), Literal::S(b)) => {
                return Literal::S(a.clone() + &b.clone());
            }

            // comparison
            (TokenType::GREATER, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Bool(a > b);
            }
            (TokenType::GREATER_EQUAL, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Bool(a >= b);
            }
            (TokenType::LESS, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Bool(a < b);
            }
            (TokenType::LESS_EQUAL, Literal::Int(a), Literal::Int(b)) => {
                return Literal::Bool(a <= b);
            }

            // equality
            (TokenType::EQUAL_EQUAL, a, b) => {
                return Literal::Bool(a == b);
            }
            (TokenType::BANG_EQUAL, a, b) => {
                return Literal::Bool(!(a == b));
            }
            
            // error?
            (_, _ ,_) => {
                return Literal::Nil;
            } 
        }
    }
    fn visit_grouping(&self, g: &Grouping) -> Literal {
        g.expr.accept_l(self)
    } 
    fn visit_literal(&self, l: &Literal) -> Literal {
        l.clone()
    }
    fn visit_unary(&self, u: &Unary) -> Literal {
        let right = u.right.accept_l(self);

        match (&u.operator.variant, right) {
            (TokenType::MINUS, Literal::Int(i)) => {
                return Literal::Int(-i);
            }
            // (TokenType::MINUS, _) => {
            //     return 
            // }
            (TokenType::BANG, e) => {
                return Literal::Bool(!self.is_truthy(&e));
            }
            (_,  _) => {
                return Literal::Nil;
            }
        }
    }
}
