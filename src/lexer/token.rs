use crate::lexer::error::LexicalError;
use crate::lexer::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexical_error: Option<LexicalError>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn as_string_for_lexer(&self) -> String {
        match &self.lexical_error {
            None => { self.token_type.as_string_for_lexer() }
            Some(error) => { error.as_string() }
        }
    }

    pub fn as_string_for_parser(&self) -> String {
        self.token_type.as_string_for_parser()
    }
}
