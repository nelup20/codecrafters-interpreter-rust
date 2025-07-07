use crate::lexer::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn as_str(&self) -> &str {
        self.token_type.as_str()
    }
}
