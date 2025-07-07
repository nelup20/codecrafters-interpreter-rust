use crate::lexer::error::LexicalError;
use crate::lexer::reserved_keyword::ReservedKeyword;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use std::io::Write;

pub struct Lexer {
    tokens: Vec<Token>,
    pub has_errors: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            has_errors: false,
        }
    }

    pub fn scan_input(&mut self, input: &str) {
        let mut column = 0;
        let mut line = 1;

        let mut input_chars = input.chars().peekable();
        while let Some(char) = input_chars.next() {
            let mut token_type: Option<TokenType> = None;
            let mut lexical_error: Option<LexicalError> = None;

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
                '\t' | ' ' => {}

                '!' => match input_chars.peek() {
                    Some('=') => {
                        token_type = Some(TokenType::BangEqual);
                        input_chars.next();
                    }
                    _ => token_type = Some(TokenType::Bang),
                },

                '=' => match input_chars.peek() {
                    Some('=') => {
                        token_type = Some(TokenType::DoubleEqual);
                        input_chars.next();
                    }
                    _ => token_type = Some(TokenType::Equal),
                },

                '<' => match input_chars.peek() {
                    Some('=') => {
                        token_type = Some(TokenType::LessThanOrEqual);
                        input_chars.next();
                    }
                    _ => token_type = Some(TokenType::LessThan),
                },

                '>' => match input_chars.peek() {
                    Some('=') => {
                        token_type = Some(TokenType::GreaterThanOrEqual);
                        input_chars.next();
                    }
                    _ => token_type = Some(TokenType::GreaterThan),
                },

                '/' => match input_chars.peek() {
                    Some('/') => {
                        while input_chars.peek().is_some_and(|&char| char != '\n') {
                            input_chars.next();
                        }
                    }
                    _ => token_type = Some(TokenType::Slash),
                },

                '"' => {
                    let mut string_is_terminated = false;
                    let mut literal_value = String::new();

                    while let Some(next_char) = input_chars.next() {
                        match next_char {
                            '"' => {
                                string_is_terminated = true;
                                break;
                            }
                            _ => {
                                literal_value.push(next_char);
                            }
                        }
                    }

                    token_type = Some(TokenType::StringLiteral(literal_value));

                    if !string_is_terminated {
                        lexical_error = Some(LexicalError::UnterminatedString(line));
                    }
                }

                other_char => {
                    if is_char_digit(other_char) {
                        let mut entire_number = String::new();
                        entire_number.push(other_char);

                        while let Some(&next_char) = input_chars.peek() {
                            if is_char_digit(next_char) || next_char == '.' {
                                entire_number.push(input_chars.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        token_type = Some(TokenType::Number(entire_number))
                    } else if is_char_letter_or_underscore(other_char) {
                        let mut entire_identifier = String::new();
                        entire_identifier.push(other_char);

                        while let Some(&next_char) = input_chars.peek() {
                            if is_char_alphanumeric(next_char) {
                                entire_identifier.push(input_chars.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        if ReservedKeyword::is_reserved_keyword(&entire_identifier) {
                            let reserved_keyword = ReservedKeyword::from_str(&entire_identifier);
                            token_type = Some(TokenType::Reserved(reserved_keyword))
                        } else {
                            token_type = Some(TokenType::Identifier(entire_identifier));
                        }
                    } else {
                        token_type = Some(TokenType::InvalidChar);
                        lexical_error = Some(LexicalError::UnexpectedChar(line, other_char));
                    }
                }
            }

            if lexical_error.is_some() {
                self.has_errors = true;
            }

            match token_type {
                None => {}
                Some(token_type) => self.tokens.push(Token {
                    token_type,
                    lexical_error,
                    line,
                    column,
                }),
            }

            column += 1;
        }
    }

    pub fn write_tokens_to_stream(
        &self,
        stdout_stream: &mut dyn Write,
        stderr_stream: &mut dyn Write,
    ) {
        for token in &self.tokens {
            match token.lexical_error {
                Some(_) => {
                    writeln!(stderr_stream, "{}", token.as_string()).unwrap();
                }

                None => {
                    writeln!(stdout_stream, "{}", token.as_string()).unwrap();
                }
            }
        }

        writeln!(stdout_stream, "EOF  null").unwrap();
    }
}

#[inline(always)]
fn is_char_alphanumeric(char: char) -> bool {
    is_char_digit(char) || is_char_letter_or_underscore(char)
}

#[inline(always)]
fn is_char_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

#[inline(always)]
fn is_char_letter_or_underscore(char: char) -> bool {
    is_char_lowercase_letter(char) || is_char_uppercase_letter(char) || char == '_'
}

#[inline(always)]
fn is_char_uppercase_letter(char: char) -> bool {
    char >= 'A' && char <= 'Z'
}

#[inline(always)]
fn is_char_lowercase_letter(char: char) -> bool {
    char >= 'a' && char <= 'z'
}
