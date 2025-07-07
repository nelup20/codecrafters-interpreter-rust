
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Equal,
    DoubleEqual,
    Bang,
    BangEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Invalid(char)
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            TokenType::LeftParen => "LEFT_PAREN ( null",
            TokenType::RightParen => "RIGHT_PAREN ) null",
            TokenType::LeftBrace => "LEFT_BRACE { null",
            TokenType::RightBrace => "RIGHT_BRACE } null",
            TokenType::Star => "STAR * null",
            TokenType::Dot => "DOT . null",
            TokenType::Comma => "COMMA , null",
            TokenType::Plus => "PLUS + null",
            TokenType::Minus => "MINUS - null",
            TokenType::Semicolon => "SEMICOLON ; null",
            TokenType::Equal => "EQUAL = null",
            TokenType::DoubleEqual => "EQUAL_EQUAL == null",
            TokenType::Bang => "BANG ! null",
            TokenType::BangEqual => "BANG_EQUAL != null",
            TokenType::LessThan => "LESS < null",
            TokenType::GreaterThan => "GREATER > null",
            TokenType::LessThanOrEqual => "LESS_EQUAL <= null",
            TokenType::GreaterThanOrEqual => "GREATER_EQUAL >= null",
            TokenType::Invalid(_) => "INVALID null",
        }
    }
}