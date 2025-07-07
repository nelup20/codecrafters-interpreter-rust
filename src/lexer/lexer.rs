use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use std::io::Write;

pub struct Lexer {
    tokens: Vec<Token>,
    has_error: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            has_error: false,
        }
    }

    pub fn scan_input(&mut self, input: &str) {
        let mut column = 0;
        let mut line = 0;

        for char in input.chars() {
            let mut token_type: Option<TokenType> = None;

            match char {
                '(' => token_type = Some(TokenType::LeftParen),
                ')' => token_type = Some(TokenType::RightParen),
                '{' => token_type = Some(TokenType::LeftBrace),
                '}' => token_type = Some(TokenType::RightBrace),
                '*' => token_type = Some(TokenType::Star),
                '+' => token_type = Some(TokenType::Plus),
                '-' => token_type = Some(TokenType::Minus),
                ',' => token_type = Some(TokenType::Comma),
                '.' => token_type = Some(TokenType::Dot),
                ';' => token_type = Some(TokenType::Semicolon),
                '\n' => line += 1,
                _ => {}
            }

            match token_type {
                None => {}
                Some(token_type) => self.tokens.push(Token {
                    token_type,
                    line,
                    column,
                }),
            }

            column += 1;
        }
    }

    pub fn write_tokens_to_stream(&self, output_stream: &mut dyn Write) {
        for token in &self.tokens {
            writeln!(output_stream, "{}", token.as_str());
        }

        writeln!(output_stream, "EOF  null");
    }
}
