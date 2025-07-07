pub enum LexicalError {
    UnexpectedChar(usize, char),
    UnterminatedString(usize),
}

impl LexicalError {
    pub fn as_string(&self) -> String {
        match self {
            Self::UnexpectedChar(line, invalid_char) => format!(
                "[line {}] Error: Unexpected character: {}",
                line, invalid_char
            ),

            Self::UnterminatedString(line) => {
                format!("[line {}] Error: Unterminated string.", line)
            }
        }
    }
}