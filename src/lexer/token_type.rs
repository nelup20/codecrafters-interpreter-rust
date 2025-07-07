
pub enum TokenType {
    LeftParen,
    RightParen
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            TokenType::LeftParen => "LEFT_PAREN ( null",
            TokenType::RightParen => "RIGHT_PAREN ) null"
        }
    }
}