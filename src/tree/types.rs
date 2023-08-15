use crate::{
    cursor::StringCursor,
    location::Locational,
    token::{Bracket, Keyword, Symbol, Token},
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
    pub fn from_cursor(cursor: &mut StringCursor) -> Locational<Result<Self, ParseError>> {
        let Locational { loc, t: token } = next_non_space!(cursor);
        let startidx = loc.start;

        match token {
            Token::Keyword(Keyword::Typeof) => {
                let Locational { loc: _, t: base } = Self::from_cursor(cursor)?;
                Locational::from(
                    startidx,
                    cursor.idx,
                    Ok(Type::Meta {
                        base_type: Box::new(base),
                    }),
                )
            }
            Token::Keyword(Keyword::Fun) => {
                let Locational { loc: _, t: fun } = Self::extract_fun(cursor, startidx)?;
                return Locational::from(startidx, cursor.idx, Ok(fun));
            }
            Token::Ident(ident) => {
                let Locational { loc: _, t: basic } =
                    Self::extract_basic(cursor, &ident, startidx)?;
                return Locational::from(startidx, cursor.idx, Ok(basic));
            }
            Token::Eof => {
                return Locational::from(
                    startidx,
                    cursor.idx,
                    Err(ParseError::UnexpectedToken(
                        Token::Eof,
                        "not EOF".to_string(),
                    )),
                )
            }
            _ => {
                cursor.idx = startidx;
                return Locational::from(startidx, startidx, Ok(Self::Empty));
            }
        }
    }

    fn extract_param_list(
        cursor: &mut StringCursor,
        multi_phase: bool,
    ) -> Locational<Result<(Vec<Type>, Vec<Type>), ParseError>> {
        let mut meta_parameters: Vec<Type> = vec![];
        let mut value_parameters: Vec<Type> = vec![];

        let Locational { loc, t: token } = next_non_space!(cursor);

        let startidx = loc.start;

        match token {
            Token::Symbol(Symbol::BracketClose(Bracket::Paren)) => {
                return Locational::from_loc(loc, Ok((meta_parameters, value_parameters)))
            }
            Token::Eof => {
                return Locational::from_loc(
                    loc,
                    Err(ParseError::UnexpectedToken(
                        Token::Eof,
                        "type or `)`".to_string(),
                    )),
                )
            }
            _ => cursor.idx = loc.start,
        }

        while let Locational {
            loc: _,
            t: Ok(param),
        } = Self::from_cursor(cursor)
        {
            match param {
                Self::Empty => {
                    let Locational { loc, t: curr } = cursor.next_token()?;
                    match curr {
                        Token::Symbol(Symbol::BracketClose(Bracket::Paren)) => break,
                        t => {
                            return Locational::from_loc(
                                loc,
                                Err(ParseError::UnexpectedToken(t, "type or `,`".to_string())),
                            )
                        }
                    }
                }
                p => value_parameters.push(p),
            }

            let Locational { loc, t: token } = next_non_space!(cursor);

            match token {
                Token::Symbol(Symbol::Semicolon) => {
                    meta_parameters = value_parameters;
                    value_parameters = vec![];
                }
                Token::Symbol(Symbol::Comma) => continue,
                Token::Symbol(Symbol::BracketClose(Bracket::Paren)) => break,
                t => {
                    return Locational::from_loc(
                        loc,
                        Err(ParseError::UnexpectedToken(
                            t,
                            if multi_phase {
                                "`,`, `;`, `)`"
                            } else {
                                "`,`, `)`"
                            }
                            .to_string(),
                        )),
                    )
                }
            }
        }

        Locational::from(
            startidx,
            cursor.idx,
            Ok((meta_parameters, value_parameters)),
        )
    }

    fn extract_basic(
        cursor: &mut StringCursor,
        ident: &str,
        startidx: usize,
    ) -> Locational<Result<Self, ParseError>> {
        let Locational { loc, t: token } = next_non_space!(cursor);

        match token {
            Token::Symbol(Symbol::BracketOpen(Bracket::Paren)) => {}
            _ => {
                cursor.idx = loc.start;
                return Locational::from(
                    startidx,
                    cursor.idx,
                    Ok(Self::Basic {
                        name: ident.to_string(),
                        parameters: vec![],
                    }),
                );
            }
        }

        let Locational {
            loc: _,
            t: (_, parameters),
        } = Self::extract_param_list(cursor, false)?;

        Locational::from(
            startidx,
            cursor.idx,
            Ok(Self::Basic {
                name: ident.to_string(),
                parameters: parameters,
            }),
        )
    }

    fn extract_fun(
        cursor: &mut StringCursor,
        startidx: usize,
    ) -> Locational<Result<Self, ParseError>> {
        let Locational { loc, t: token } = next_non_space!(cursor);

        match token {
            Token::Symbol(Symbol::BracketOpen(Bracket::Paren)) => {}
            t => {
                return Locational::from_loc(
                    loc,
                    Err(ParseError::UnexpectedToken(t, "`(`".to_string())),
                )
            }
        }

        let Locational {
            loc: _,
            t: (meta_parameters, value_parameters),
        } = Self::extract_param_list(cursor, true)?;

        let Locational {
            loc: _,
            t: return_type,
        } = Self::from_cursor(cursor)?;

        Locational::from(
            startidx,
            cursor.idx,
            Ok(Self::Fun {
                meta_parameters,
                value_parameters,
                return_type: Box::new(return_type),
            }),
        )
    }
}
