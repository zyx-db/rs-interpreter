use std::collections::HashMap;
use crate::{parsing::{expressions::Expr, tokens::Token}, errors::err::RuntimeErr};

enum Ret {
    E(Box<dyn Expr>)
}

pub struct Environment {
    values: HashMap<String, Ret>,
}

impl Environment {
    fn define(&mut self, name: String, value: Ret){
        self.values.insert(name, value);
    }

    fn get(&self, name: Token) -> Result<Ret, RuntimeErr> {
        if self.values.contains_key(&name.lexeme){
            return Ok(*self.values.get(&name.lexeme).unwrap());
        }
        Err(RuntimeErr::new("Undefined variable".to_owned(), name))
    }
}
