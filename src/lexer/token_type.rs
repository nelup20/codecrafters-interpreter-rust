
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            TokenType::LeftParen => "LEFT_PAREN ( null",
            TokenType::RightParen => "RIGHT_PAREN ) null",
            TokenType::LeftBrace => "LEFT_BRACE { null",
            TokenType::RightBrace => "RIGHT_BRACE } null",
        }
    }
}