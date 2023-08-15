use crate::{
    cursor::StringCursor,
    token::{Keyword, Token, Symbol, Bracket},
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
        meta_parameters: Vec<Type>,
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

        match token {
            Token::Keyword(_, Keyword::Typeof) =>
                return Ok(Self::Meta {
                    base_type: Box::new(Self::from_cursor(cursor)?),
                }),
            Token::Keyword(_, Keyword::Fun) =>
                return Self::extract_fun(cursor),
            Token::Ident(_, ident) =>
                return Self::extract_basic(cursor, &ident),
            Token::Eof => 
                return Err(ParseError::UnexpectedToken(Token::Eof, "not EOF".to_string())),
            t => {
                cursor.idx = t.get_start_idx().unwrap();
                return Ok(Self::Empty)
            }
        }
    }

    fn extract_param_list(cursor: &mut StringCursor, multi_phase: bool) -> Result<(Vec<Type>, Vec<Type>), ParseError> {
        let mut meta_parameters: Vec<Type> = vec![];
        let mut value_parameters: Vec<Type> = vec![];

        match cursor.next_token()? {
            Token::Symbol(_, Symbol::BracketClose(Bracket::Paren)) => return Ok((meta_parameters, value_parameters)),
            Token::Eof => return Err(ParseError::UnexpectedToken(Token::Eof, "type or `)`".to_string())),
            t => cursor.idx = t.get_start_idx().unwrap()
        }

        while let Ok(param) = Self::from_cursor(cursor) {
            match param {
                Self::Empty => {
                    let curr = cursor.next_token()?;
                    match curr {
                        Token::Symbol(_, Symbol::BracketClose(Bracket::Paren)) => break,
                        t => return Err(ParseError::UnexpectedToken(t, "type or `,`".to_string()))
                    }
                },
                p => value_parameters.push(p)
            }
            
            let token = next_non_space!(cursor);

            match token {
                Token::Symbol(_, Symbol::Semicolon) => {
                    meta_parameters = value_parameters;
                    value_parameters = vec![];
                },
                Token::Symbol(_, Symbol::Comma) => continue,
                Token::Symbol(_, Symbol::BracketClose(Bracket::Paren)) => return Ok((meta_parameters, value_parameters)),
                t => return Err(ParseError::UnexpectedToken(t, if multi_phase { "`,`, `;`, `)`" } else { "`,`, `)`" }.to_string()))
            }
        }
        
        Ok((meta_parameters, value_parameters))
    }

    fn extract_basic(cursor: &mut StringCursor, ident: &str) -> Result<Self, ParseError> {
        let token = next_non_space!(cursor);
        match token {
            Token::Symbol(_, Symbol::BracketOpen(Bracket::Paren)) => {}
            t => {
                if let Some(i) = t.get_start_idx() {
                    cursor.idx = i;
                }
                return Ok(Self::Basic { name: ident.to_string(), parameters: vec![] })
            }
        }

        let (_, parameters) = Self::extract_param_list(cursor, false)?;

        Ok(Self::Basic {
            name: ident.to_string(),
            parameters: parameters
        })
    }

    fn extract_fun(cursor: &mut StringCursor) -> Result<Self, ParseError> {
        let token = next_non_space!(cursor);
        
        match token {
            Token::Symbol(_, Symbol::BracketOpen(Bracket::Paren)) => {}
            t => return Err(ParseError::UnexpectedToken(t, "`(`".to_string()))
        }
        
        let (meta_parameters, value_parameters) = Self::extract_param_list(cursor, true)?;

        let return_type = Self::from_cursor(cursor)?;

        Ok(Self::Fun {
            meta_parameters,
            value_parameters,
            return_type: Box::new(return_type)
        })
    }
}
