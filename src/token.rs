use crate::cursor::StringCursor;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident(Location, String),
    Keyword(Location, Keyword),
    Number(Location, String),
    String(Location, String, char),
    Symbol(Location, Symbol),
    Whitespace(Location, char),
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    UnclosedString(Location),
    MultiDottedNumber(Location),
    UnknownChar(Location, char),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq)]
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
    Is,
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

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    And,                    // &
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

#[derive(Debug, PartialEq, Eq)]
pub enum Bracket {
    Angle,  // <>
    Curly,  // {}
    Paren,  // ()
    Square, // []
}

impl Token {
    pub fn from_cursor(cursor: &mut StringCursor) -> Result<Self, TokenError> {
        let start_pos = cursor.idx;
        if cursor.idx >= cursor.string.len() {
            return Ok(Self::Eof)
        }
        let char = cursor.current().unwrap();

        if char.is_whitespace() {
            Self::extract_whitespace(cursor, char);
            return Ok(Self::Whitespace(Location::new(start_pos, cursor.idx), char))
        }

        if let Some(symbol) = Symbol::from_char(char) {
            cursor.advance();
            return Ok(Token::Symbol(Location::new(start_pos, cursor.idx), symbol))
        }

        if char.is_alphabetic() || char == '_' {
            let ident = Self::extract_ident(cursor);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(keyword) = Keyword::from_str(&ident) {
                return Ok(Self::Keyword(loc, keyword))
            }
            return Ok(Self::Ident(loc, ident))
        }

        if char.is_numeric() {
            let num = Self::extract_number(cursor);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(num) = num {
                return Ok(Self::Number(loc, num))
            }
            return Err(TokenError::MultiDottedNumber(loc))
        }

        if char == '\'' || char == '"' {
            let str = Self::extract_string(cursor, char);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(str) = str {
                return Ok(Self::String(loc, str, char))
            }
            return Err(TokenError::UnclosedString(loc))
        }

        Err(TokenError::UnknownChar(Location::new(start_pos, cursor.idx), char))
    }

    fn extract_string(cursor: &mut StringCursor, closing_char: char) -> Option<String> {
        cursor.advance();
        let mut str = "".to_string();
        let mut closed = false;
        while let Some(char) = cursor.current() {
            cursor.advance();
            if char == closing_char {
                closed = true;
                break
            }
            str.push(char);
            if char == '\\' && let Some(char) = cursor.current() {
                cursor.advance();
                str.push(char);
            }
        }

        if !closed {
            None
        } else {
            Some(str)
        }
    }

    fn extract_whitespace(cursor: &mut StringCursor, whitespace_char: char) {
        while let Some(char) = cursor.current() && char == whitespace_char {
            cursor.advance();
        }
    }

    fn extract_ident(cursor: &mut StringCursor) -> String {
        let mut str = "".to_string();

        while let Some(char) = cursor.current() && (char.is_alphanumeric() || char == '_') {
            str.push(char);
            cursor.advance();
        }

        str
    }

    fn extract_number(cursor: &mut StringCursor) -> Option<String> {
        let mut str = "".to_string();

        while let Some(char) = cursor.current() && (char.is_numeric() || char == '_' || char == '.') {
            if char == '.' && str.contains('.') {
                return None
            }
            str.push(char);
            cursor.advance();
        }

        Some(str)
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
            "is" => Some(Self::Is),
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
}
