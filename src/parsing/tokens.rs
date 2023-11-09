#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    IDENTIFIER,
    STRING,
    NUMBER,

    AND,
    CLASS,
    ELSE,
    FALSE,
    DEF,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub variant: TokenType,
    pub lexeme: String,
    pub string: Option<String>,
    pub int: Option<f64>,
    pub line: u32,
}

impl Token {
    pub fn new(var: TokenType, lexeme: String, line: u32) -> Self {
        Token {
            variant: var,
            lexeme,
            line,
            string: None,
            int: None,
        }
    }

    pub fn new_int(var: TokenType, lexeme: String, line: u32, int: f64) -> Self {
        Token {
            variant: var,
            lexeme,
            line,
            string: None,
            int: Some(int),
        }
    }

    pub fn new_str(var: TokenType, lexeme: String, line: u32, string: String) -> Self {
        Token {
            variant: var,
            lexeme,
            line,
            string: Some(string),
            int: None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.variant, self.lexeme, self.line)
    }
}
