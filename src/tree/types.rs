use crate::{
    cursor::StringCursor,
    token::{Keyword, Token},
    tree::next_non_space,
};

use super::ParseError;

#[derive(Debug)]
pub enum Type {
    Basic {
        name: String,
        parameters: Vec<Type>,
    },
    Fun {
        type_parameters: Vec<Type>,
        value_parameters: Vec<Type>,
        return_type: Box<Type>,
    },
    Meta {
        base_type: Box<Type>,
    },
    Empty,
}

impl Type {
    pub fn from_cursor(cursor: &mut StringCursor) -> Result<Self, ParseError> {
        let token = next_non_space!(cursor);

        if let Token::Keyword(_, Keyword::Typeof) = token {
            return Ok(Self::Meta {
                base_type: Box::new(Self::from_cursor(cursor)?),
            });
        }

        if let Token::Keyword(_, Keyword::Fun) = token {
            return Self::extract_fun(cursor);
        }

        todo!()
    }

    fn extract_fun(cursor: &mut StringCursor) -> Result<Self, ParseError> {
        let token = next_non_space!(cursor);

        todo!()
    }
}
