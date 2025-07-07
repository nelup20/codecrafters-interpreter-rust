use crate::lexer::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn as_string(&self) -> String {
        match self.token_type {
            TokenType::Invalid(invalid_char) => { 
                format!("[line {}] Error: Unexpected character: {}", self.line, invalid_char)
            },
            _ => self.token_type.as_str().to_string(),
        }
    }
}
