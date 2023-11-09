use super::super::parsing::tokens::Token;

pub fn error(line: u32, message: &String) {
    report(line, &"".to_owned(), message);
}

fn report(line: u32, location: &String, message: &String) {
    eprintln!("[Line {}] ERROR{}: {}", line, location, message);
}

#[derive(Debug)]
pub struct RuntimeErr {
    message: String,
    token: Token,
}

impl RuntimeErr {
    pub fn default() -> Self{
        RuntimeErr { message: "ERR".to_owned(), token: Token { variant: crate::parsing::tokens::TokenType::BANG, lexeme: "!".to_owned(), string: None, int: None, line: 1 } }
    }
}
