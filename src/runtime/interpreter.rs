use crate::errors::err::RuntimeErr;

use crate::parsing::expressions::{Binary, Grouping, Literal, Unary, Expr, Value};
use crate::parsing::statements::{Stmt, Print, ExprStmt};
use crate::parsing::tokens::{TokenType, Token};

pub trait Visitor<T>{
    fn visit_binary(&self, b: &Binary) -> Result<T, RuntimeErr>;
    fn visit_grouping(&self, g: &Grouping) -> Result<T, RuntimeErr>;
    fn visit_literal(&self, l: &Literal) -> Result<T, RuntimeErr>;
    fn visit_unary(&self, u: &Unary) -> Result<T, RuntimeErr>;
    fn visit_expr_stmt(&self, s: &ExprStmt) -> Result<T, RuntimeErr>;
    fn visit_print_stmt(&self, s: &Print) -> Result<T, RuntimeErr>;
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {  }
    }

    pub fn interpret(&mut self, stmts: Vec<Box<dyn Stmt>>) {
        for s in stmts {
            let res = self.execute(s);
            if res.is_err(){
                let e = res.err().unwrap();
                eprintln!("{:?}", e);
                break;
            }
        }
        
    }

    fn evaluate(&self, e: &Box<dyn Expr>) -> Result<Literal, RuntimeErr> {
        e.accept(self)
    }

    fn is_truthy(&self, expr: &Value) -> bool {
        match expr {
           Value::Nil => {return false;} 
           Value::Bool(res) => {return *res;}
           _ => {return true;}
        }
    }

    fn execute(&mut self, stmt: Box<dyn Stmt>) -> Result<Literal, RuntimeErr>{
        stmt.accept(self)
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_binary(&self, b: &Binary) -> Result<Literal, RuntimeErr> {
        let left = b.left.accept(self);
        let right = b.right.accept(self);

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
        g.expr.accept(self)
    } 
    fn visit_literal(&self, l: &Literal) -> Result<Literal, RuntimeErr> {
        Ok(l.clone())
    }
    fn visit_unary(&self, u: &Unary) -> Result<Literal, RuntimeErr> {
        let possible = u.right.accept(self);
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

    fn visit_expr_stmt(&self, s: &ExprStmt) -> Result<Literal, RuntimeErr> {
        self.evaluate(&s.expr);
        return Ok(Literal::default());
    }

    fn visit_print_stmt(&self, s: &Print) -> Result<Literal, RuntimeErr> {
        let value = self.evaluate(&s.expr);
        println!("{:?}", value);
        return Ok(Literal::default());
    }
}
