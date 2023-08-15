use crate::{
    location::Locational,
    token::{Token, TokenError},
};

#[derive(Debug, PartialEq, Eq)]
pub struct StringCursor {
    pub string: String,
    pub idx: usize,
}

impl StringCursor {
    pub fn from(str: &str) -> Self {
        Self {
            string: str.into(),
            idx: 0,
        }
    }

    pub fn next_token(&mut self) -> Locational<Result<Token, TokenError>> {
        Token::from_cursor(self)
    }

    pub fn current(&self) -> Option<char> {
        self.string.chars().nth(self.idx)
    }

    pub fn advance(&mut self) {
        self.idx += 1;
    }
}
