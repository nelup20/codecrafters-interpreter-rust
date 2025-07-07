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
    Slash,
    StringLiteral(String),
    Number(String),
    Identifier(String),
    InvalidChar,
}

impl TokenType {
    pub fn as_string(&self) -> String {
        match self {
            TokenType::LeftParen => "LEFT_PAREN ( null".to_string(),
            TokenType::RightParen => "RIGHT_PAREN ) null".to_string(),
            TokenType::LeftBrace => "LEFT_BRACE { null".to_string(),
            TokenType::RightBrace => "RIGHT_BRACE } null".to_string(),
            TokenType::Star => "STAR * null".to_string(),
            TokenType::Dot => "DOT . null".to_string(),
            TokenType::Comma => "COMMA , null".to_string(),
            TokenType::Plus => "PLUS + null".to_string(),
            TokenType::Minus => "MINUS - null".to_string(),
            TokenType::Semicolon => "SEMICOLON ; null".to_string(),
            TokenType::Equal => "EQUAL = null".to_string(),
            TokenType::DoubleEqual => "EQUAL_EQUAL == null".to_string(),
            TokenType::Bang => "BANG ! null".to_string(),
            TokenType::BangEqual => "BANG_EQUAL != null".to_string(),
            TokenType::LessThan => "LESS < null".to_string(),
            TokenType::GreaterThan => "GREATER > null".to_string(),
            TokenType::LessThanOrEqual => "LESS_EQUAL <= null".to_string(),
            TokenType::GreaterThanOrEqual => "GREATER_EQUAL >= null".to_string(),
            TokenType::Slash => "SLASH / null".to_string(),
            TokenType::Number(number) => {
                let parsed_number: f64 = number.parse().unwrap();
                format!("NUMBER {number} {parsed_number:?}")
            }

            TokenType::StringLiteral(literal_value) => {
                format!("STRING \"{}\" {}", literal_value, literal_value)
            }

            TokenType::Identifier(identifier) => {
                format!("IDENTIFIER {identifier} null")
            }

            TokenType::InvalidChar => "INVALID CHAR".to_string(),
        }
    }
}
