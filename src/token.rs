use std::{str::Chars, char, cell::OnceCell, iter::Enumerate};

use regex::Regex;

use crate::cursor::StringCursor;

#[derive(Debug)]
pub enum Token {
    Ident(Location, String),
    Keyword(Location, Keyword),
    Symbol(Location, Symbol),
    Whitespace(Location),
}

#[derive(Debug)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum Keyword {
    As,
    Break,
    Class,
    Continue,
    Defer,
    Enum,
    For,
    From,
    Fun,
    Global,
    If,
    Impl,
    Import,
    Let,
    Loop,
    Mut,
    Pub,
    Return,
    Trait,
    Typeof,
    When,
    While,
}

#[derive(Debug)]
pub enum Symbol {
    And,             // &
    Asterisk,               // *
    BracketClose(Bracket),  // ], }, ), >
    BracketOpen(Bracket),   // [, {, (, <
    Caret,                  // ^
    Colon,                  // :
    Comma,                  // ,
    Dot,                    // .
    Equal,                  // =
    Minus,                  // -
    Not,                    // !
    Pipe,                   // |
    Plus,                   // +
}

#[derive(Debug)]
pub enum Bracket {
    Angle,  // <>
    Curly,  // {}
    Paren,  // ()
    Square, // []
}

impl Token {
    pub fn from_cursor(cursor: &mut StringCursor) -> Option<Self> {
        let start_pos = cursor.idx;
        let char = cursor.current()?;

        if char.is_whitespace() {
            Self::extract_whitespace(cursor);
            return Some(Self::Whitespace(Location::new(start_pos, cursor.idx)))
        }

        if let Some(symbol) = Symbol::from_char(char) {
            cursor.advance();
            return Some(Token::Symbol(Location::new(start_pos, cursor.idx), symbol))
        }

        if char.is_alphabetic() || char == '_' {
            let ident = Self::extract_ident(cursor, char);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(keyword) = Keyword::from_str(&ident) {
                return Some(Self::Keyword(loc, keyword))
            }
            return Some(Self::Ident(loc, ident))
        }

        None
    }

    fn extract_whitespace(cursor: &mut StringCursor) {
        while let Some(char) = cursor.current() && char.is_whitespace() {
            cursor.advance();
        }
    }

    fn extract_ident(cursor: &mut StringCursor, char: char) -> String {
        let mut str = char.to_string();
        cursor.advance();

        while let Some(char) = cursor.current() && (char.is_alphanumeric() || char == '_') {
            str.push(char);
            cursor.advance();
        }

        str
    }
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Keyword {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "as" => Some(Self::As),
            "break" => Some(Self::Break),
            "class" => Some(Self::Class),
            "continue" => Some(Self::Continue),
            "defer" => Some(Self::Defer),
            "enum" => Some(Self::Enum),
            "for" => Some(Self::For),
            "from" => Some(Self::From),
            "fun" => Some(Self::Fun),
            "global" => Some(Self::Global),
            "if" => Some(Self::If),
            "impl" => Some(Self::Impl),
            "import" => Some(Self::Import),
            "let" => Some(Self::Let),
            "loop" => Some(Self::Loop),
            "mut" => Some(Self::Mut),
            "pub" => Some(Self::Pub),
            "return" => Some(Self::Return),
            "trait" => Some(Self::Trait),
            "typeof" => Some(Self::Typeof),
            "when" => Some(Self::When),
            "while" => Some(Self::While),
            _ => None
        }
    }
}

impl Symbol {
    pub fn from_char(char: char) -> Option<Self> {
        if let Some(bracket) = Bracket::from_opening_char(char) {
            return Some(Self::BracketOpen(bracket))
        }
        if let Some(bracket) = Bracket::from_closing_char(char) {
            return Some(Self::BracketClose(bracket))
        }

        match char {
            '&' => Some(Self::And),
            '*' => Some(Self::Asterisk),
            '^' => Some(Self::Caret),
            ':' => Some(Self::Colon),
            ',' => Some(Self::Comma),
            '.' => Some(Self::Dot),
            '=' => Some(Self::Equal),
            '-' => Some(Self::Minus),
            '!' => Some(Self::Not),
            '|' => Some(Self::Pipe),
            '+' => Some(Self::Plus),
            _ => None
        }
    }
}

impl Bracket {
    pub fn from_opening_char(char: char) -> Option<Self> {
        match char {
            '<' => Some(Self::Angle),
            '{' => Some(Self::Curly),
            '(' => Some(Self::Paren),
            '[' => Some(Self::Square),
            _ => None,
        }
    }

    pub fn from_closing_char(char: char) -> Option<Self> {
        match char {
            '>' => Some(Self::Angle),
            '}' => Some(Self::Curly),
            ')' => Some(Self::Paren),
            ']' => Some(Self::Square),
            _ => None,
        }
    }

    pub fn from_char(char: char) -> Option<Self> {
        match char {
            '<' | '>' => Some(Self::Angle),
            '{' | '}' => Some(Self::Curly),
            '(' | ')' => Some(Self::Paren),
            '[' | ']' => Some(Self::Square),
            _ => None,
        }
    }
}
