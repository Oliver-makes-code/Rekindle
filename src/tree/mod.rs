use crate::token::{Token, TokenError};

pub mod classes;
pub mod functions;
pub mod traits;
pub mod types;
pub mod variables;

pub enum ParseError {
    Token(TokenError),
    UnexpectedToken(Token, String),
}

impl From<TokenError> for ParseError {
    fn from(value: TokenError) -> Self {
        Self::Token(value)
    }
}

pub macro next_non_space($cursor: expr) {
    loop {
        let token = $cursor.next_token()?;
        if let crate::token::Token::Comment(_, _) = token {
            continue;
        }
        if let crate::token::Token::Whitespace(_, _) = token {
            continue;
        }
        break token;
    }
}
