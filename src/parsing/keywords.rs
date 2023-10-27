use std::collections::HashMap;

use super::tokens::TokenType;

pub fn keywords_map() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("and".to_owned(),    TokenType::AND);
    keywords.insert("class".to_owned(),  TokenType::CLASS);
    keywords.insert("else".to_owned(),   TokenType::ELSE);
    keywords.insert("false".to_owned(),  TokenType::FALSE);
    keywords.insert("for".to_owned(),    TokenType::FOR);
    keywords.insert("fun".to_owned(),    TokenType::DEF);
    keywords.insert("if".to_owned(),     TokenType::IF);
    keywords.insert("nil".to_owned(),    TokenType::NIL);
    keywords.insert("or".to_owned(),     TokenType::OR);
    keywords.insert("print".to_owned(),  TokenType::PRINT);
    keywords.insert("return".to_owned(), TokenType::RETURN);
    keywords.insert("super".to_owned(),  TokenType::SUPER);
    keywords.insert("this".to_owned(),   TokenType::THIS);
    keywords.insert("true".to_owned(),   TokenType::TRUE);
    keywords.insert("var".to_owned(),    TokenType::VAR);
    keywords.insert("while".to_owned(),  TokenType::WHILE);
    keywords
}
