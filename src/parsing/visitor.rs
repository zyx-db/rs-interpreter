use crate::errors::err::RuntimeErr;

use super::expressions::{Binary, Grouping, Literal, Unary, Expr, Value};
use super::tokens::TokenType;

pub trait Visitor<T>{
    fn visit_binary(&self, b: &Binary) -> Result<T, RuntimeErr>;
    fn visit_grouping(&self, g: &Grouping) -> Result<T, RuntimeErr>;
    fn visit_literal(&self, l: &Literal) -> Result<T, RuntimeErr>;
    fn visit_unary(&self, u: &Unary) -> Result<T, RuntimeErr>;
}

pub struct Printer {}

impl Printer{
    pub fn new() -> Self{
        Printer {  }
    }
    pub fn print(&self, e: Box<dyn Expr>) -> Result<String, RuntimeErr>{
        e.accept_s(self)
    }
}

impl Visitor<String> for Printer {
    fn visit_binary(&self, b: &Binary) -> Result<String, RuntimeErr>{
        eprintln!("visiting Binary");
        let op = b.operator.lexeme.clone();
        let left = b.left.accept_s(self);
        let right = b.right.accept_s(self);


        return Ok(format!("{} ( {} {} )", op, left?, right?));
    }
    fn visit_grouping(&self, g: &Grouping) -> Result<String, RuntimeErr>{
        eprintln!("visiting Grouping");
        let val = g.expr.accept_s(self);
        return Ok(format!("({})", val?));
    }
    fn visit_literal(&self, l: &Literal) -> Result<String, RuntimeErr>{
        eprintln!("visiting Literal");
        match &l.val {
            Value::S(s) => {
                let res = s.clone();
                return Ok(res);
            }
            Value::Int(i) => {
                return Ok(format!("{}", i));
            }
            Value::Bool(b) => {
                return Ok(format!("{}", b));
            }
            _ => {return Ok("None".to_owned());}
        }
    }
    fn visit_unary(&self, u: &Unary) -> Result<String, RuntimeErr>{
        eprintln!("visiting Unary");
        let op = u.operator.lexeme.clone();
        let right = u.right.accept_s(self);
        return Ok(format!("({} {})", op, right?));
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {  }
    }

    pub fn evaluate(&self, e: Box<dyn Expr>) -> Result<Literal, RuntimeErr> {
        e.accept_l(self)
    }

    fn is_truthy(&self, expr: &Value) -> bool {
        match expr {
           Value::Nil => {return false;} 
           Value::Bool(res) => {return *res;}
           _ => {return true;}
        }
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_binary(&self, b: &Binary) -> Result<Literal, RuntimeErr> {
        let left = b.left.accept_l(self);
        let right = b.right.accept_l(self);

        if left.is_err(){
            return Err(left.err().unwrap());
        }

        if right.is_err(){
            return Err(right.err().unwrap());
        }

        match (&b.operator.variant, left?.val, right?.val) {
            // arithmetic
            (TokenType::PLUS, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Int(x + y)
                    )
                );
            }
            (TokenType::SLASH, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Int(x / y)
                    )
                );
            }
            (TokenType::MINUS, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Int(x - y)
                    )
                );
            }
            (TokenType::STAR, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Int(x * y)
                    )
                );
            }

            // string concatenation
            (TokenType::PLUS, Value::S(x), Value::S(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::S(x.clone() + &y.clone())
                    )
                );
            }

            // comparison
            (TokenType::GREATER, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Bool(x > y)
                    )
                );
            }
            (TokenType::GREATER_EQUAL, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Bool(x >= y)
                    )
                );
            }
            (TokenType::LESS, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Bool(x < y)
                    )
                );
            }
            (TokenType::LESS_EQUAL, Value::Int(x), Value::Int(y)) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Bool(x <= y)
                    )
                );
            }

            // equality
            (TokenType::EQUAL_EQUAL, x, y) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Bool(x == y)
                    )
                );
            }
            (TokenType::BANG_EQUAL, x, y) => {
                return Ok(
                    Literal::new(
                        b.operator.clone(),
                        Value::Bool(!(x == y))
                    )
                );
            }
            
            // invalid types
            // (TokenType::PLUS, _, _) => {}
            // (TokenType::MINUS, _, _) => {}
            // (TokenType::STAR, _, _) => {}
            // (TokenType::SLASH, _, _) => {}

            // (TokenType::GREATER, _, _) => {}
            // (TokenType::GREATER_EQUAL, _, _) => {}
            // (TokenType::LESS, _, _) => {}
            // (TokenType::LESS_EQUAL, _, _) => {}

            // error?
            (_, _ ,_) => {
                return Err(
                    RuntimeErr::new(
                        "invalid arguments to binary operation".to_owned(),
                        b.operator.clone()
                    )
                );
            } 
        }
    }
    fn visit_grouping(&self, g: &Grouping) -> Result<Literal, RuntimeErr> {
        g.expr.accept_l(self)
    } 
    fn visit_literal(&self, l: &Literal) -> Result<Literal, RuntimeErr> {
        Ok(l.clone())
    }
    fn visit_unary(&self, u: &Unary) -> Result<Literal, RuntimeErr> {
        let possible = u.right.accept_l(self);
        if possible.is_err(){
            let e = possible.err().unwrap();
            return Err(e);
        }

        let right = possible?;
        match (&u.operator.variant, right.val) {
            (TokenType::MINUS, Value::Int(i)) => {
                return Ok(
                    Literal::new(
                        u.operator.clone(),
                        Value::Int(-i)
                    )
                );
            }
            // (TokenType::MINUS, _) => {
            //     return 
            // }
            (TokenType::BANG, e) => {
                return Ok(
                    Literal::new(
                        u.operator.clone(),
                        Value::Bool(!(self.is_truthy(&e)))
                    )
                );
            }
            (_,  _) => {
                return Ok(
                    Literal::new(
                        u.operator.clone(),
                        Value::Nil
                    )
                );
            }
        }
    }
}
