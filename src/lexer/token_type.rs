use crate::lexer::reserved_keyword::ReservedKeyword;

#[derive(PartialEq)]
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
    Reserved(ReservedKeyword),
    InvalidChar,
}

impl TokenType {
    pub fn as_string_for_parser(&self) -> String {
        match self {
            TokenType::LeftParen => "(".to_string(),
            TokenType::RightParen => ")".to_string(),
            TokenType::LeftBrace => "{".to_string(),
            TokenType::RightBrace => "}".to_string(),
            TokenType::Star => "*".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Equal => "=".to_string(),
            TokenType::DoubleEqual => "==".to_string(),
            TokenType::Bang => "!".to_string(),
            TokenType::BangEqual => "!=".to_string(),
            TokenType::LessThan => "<".to_string(),
            TokenType::GreaterThan => ">".to_string(),
            TokenType::LessThanOrEqual => "<=".to_string(),
            TokenType::GreaterThanOrEqual => ">=".to_string(),
            TokenType::Slash => "/".to_string(),
            TokenType::Reserved(keyword) => keyword.as_string_for_parser(),
            TokenType::StringLiteral(literal_value) => literal_value.clone(),
            TokenType::Identifier(identifier) => identifier.clone(),
            TokenType::Number(number) => {
                let parsed_number: f64 = number.parse().unwrap();
                format!("{parsed_number:?}")
            }

            TokenType::InvalidChar => "INVALID CHAR".to_string(),
        }
    }

    pub fn as_string_for_lexer(&self) -> String {
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
            TokenType::Reserved(keyword) => keyword.as_string_for_lexer(),
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
