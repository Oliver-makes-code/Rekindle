use crate::{
    cursor::StringCursor,
    location::{Location, Locational},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Ident(String),
    Keyword(Keyword),
    Number(String),
    String(String, char),
    Symbol(Symbol),
    Whitespace(char),
    Comment(String),
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenError {
    UnclosedString,
    MultiDottedNumber,
    UnknownChar(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    Get,
    Global,
    If,
    Is,
    Impl,
    Import,
    Let,
    Loop,
    Mut,
    Namespace,
    New,
    Object,
    Pub,
    Return,
    Set,
    Trait,
    Typeof,
    When,
    While,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    And,                   // &&
    AndBitwise,            // &
    Arrow,                 // ->
    Assign,                // =
    Asterisk,              // *
    BracketClose(Bracket), // ], }, ), >
    BracketOpen(Bracket),  // [, {, (, <
    Caret,                 // ^
    Colon,                 // :
    Comma,                 // ,
    Decrement,             // --
    DivAssign,             // /=
    Dollar,                // $
    Dot,                   // .
    Equal,                 // ==
    Increment,             // ++
    Minus,                 // -
    MinusAssign,           // -=
    MultAssign,            // *=
    Not,                   // !
    NotEqual,              // !=
    Or,                    // ||
    Percent,               // %
    Pipe,                  // |
    Plus,                  // +
    PlusAssign,            // +=
    Semicolon,             // ;
    Xor,                   // ^^
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bracket {
    Angle,  // <>
    Curly,  // {}
    Paren,  // ()
    Square, // []
}

impl Token {
    pub fn from_cursor(cursor: &mut StringCursor) -> Locational<Result<Self, TokenError>> {
        let start_pos = cursor.idx;
        if cursor.idx >= cursor.string.len() {
            return Locational::from(start_pos, start_pos, Ok(Self::Eof));
        }
        let char = cursor.current().unwrap();

        if char.is_whitespace() {
            Self::extract_whitespace(cursor, char);
            return Locational::from(start_pos, cursor.idx, Ok(Self::Whitespace(char)));
        }

        if char == '/' {
            cursor.advance();
            let char = cursor.current();
            if let Some(char) = char && (char == '/' || char == '*') {
                cursor.advance();
                return Locational::from(start_pos, cursor.idx, Ok(Self::Comment(Self::extract_comment(cursor, char == '*'))));
            } else {
                cursor.idx = start_pos;
            }
        }

        if let Some(symbol) = Symbol::from_cursor(cursor) {
            return Locational::from(start_pos, cursor.idx, Ok(Token::Symbol(symbol)));
        }

        if char.is_alphabetic() || char == '_' {
            let ident = Self::extract_ident(cursor);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(keyword) = Keyword::from_str(&ident) {
                return Locational::from_loc(loc, Ok(Self::Keyword(keyword)));
            }
            return Locational::from_loc(loc, Ok(Self::Ident(ident)));
        }

        if char.is_numeric() {
            let num = Self::extract_number(cursor);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(num) = num {
                return Locational::from_loc(loc, Ok(Self::Number(num)));
            }
            return Locational::from_loc(loc, Err(TokenError::MultiDottedNumber));
        }

        if char == '\'' || char == '"' {
            let str = Self::extract_string(cursor, char);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(str) = str {
                return Locational::from_loc(loc, Ok(Self::String(str, char)));
            }
            return Locational::from_loc(loc, Err(TokenError::UnclosedString));
        }

        Locational::from(start_pos, cursor.idx, Err(TokenError::UnknownChar(char)))
    }

    fn extract_string(cursor: &mut StringCursor, closing_char: char) -> Option<String> {
        cursor.advance();
        let mut str = "".to_string();
        let mut closed = false;
        while let Some(char) = cursor.current() {
            cursor.advance();
            if char == closing_char {
                closed = true;
                break;
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

    fn extract_comment(cursor: &mut StringCursor, multiline: bool) -> String {
        let mut comment = "".to_string();

        while let Some(char) = cursor.current() {
            cursor.advance();
            comment.push(char);

            if !multiline && comment.ends_with('\n') {
                comment.pop();
                break;
            }
            if multiline && comment.ends_with("*/") {
                comment.pop();
                comment.pop();
                break;
            }
        }

        return comment;
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
            "get" => Some(Self::Get),
            "global" => Some(Self::Global),
            "if" => Some(Self::If),
            "is" => Some(Self::Is),
            "impl" => Some(Self::Impl),
            "import" => Some(Self::Import),
            "let" => Some(Self::Let),
            "loop" => Some(Self::Loop),
            "mut" => Some(Self::Mut),
            "namespace" => Some(Self::Namespace),
            "new" => Some(Self::New),
            "object" => Some(Self::Object),
            "pub" => Some(Self::Pub),
            "return" => Some(Self::Return),
            "set" => Some(Self::Set),
            "trait" => Some(Self::Trait),
            "typeof" => Some(Self::Typeof),
            "when" => Some(Self::When),
            "while" => Some(Self::While),
            _ => None,
        }
    }
}

impl Symbol {
    pub fn from_cursor(cursor: &mut StringCursor) -> Option<Self> {
        let char = cursor.current()?;

        cursor.advance();

        if let Some(next) = cursor.current() {
            let mut str = char.to_string();
            str.push(next);
            if let Some(symbol) = Self::from_str(&str) {
                cursor.advance();
                return Some(symbol);
            }
        }

        if let Some(symbol) = Self::from_char(char) {
            return Some(symbol);
        }

        cursor.idx = cursor.idx - 1;

        None
    }

    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "&&" => Some(Self::And),
            "->" => Some(Self::Arrow),
            "--" => Some(Self::Decrement),
            "/=" => Some(Self::DivAssign),
            "==" => Some(Self::Equal),
            "++" => Some(Self::Increment),
            "-=" => Some(Self::MinusAssign),
            "*=" => Some(Self::MultAssign),
            "!=" => Some(Self::NotEqual),
            "||" => Some(Self::Or),
            "+=" => Some(Self::PlusAssign),
            "^^" => Some(Self::Xor),
            _ => None,
        }
    }

    pub fn from_char(char: char) -> Option<Self> {
        if let Some(bracket) = Bracket::from_opening_char(char) {
            return Some(Self::BracketOpen(bracket));
        }
        if let Some(bracket) = Bracket::from_closing_char(char) {
            return Some(Self::BracketClose(bracket));
        }

        match char {
            '&' => Some(Self::AndBitwise),
            '*' => Some(Self::Asterisk),
            '^' => Some(Self::Caret),
            ':' => Some(Self::Colon),
            ',' => Some(Self::Comma),
            '$' => Some(Self::Dollar),
            '.' => Some(Self::Dot),
            '=' => Some(Self::Assign),
            '-' => Some(Self::Minus),
            '!' => Some(Self::Not),
            '%' => Some(Self::Percent),
            '|' => Some(Self::Pipe),
            '+' => Some(Self::Plus),
            ';' => Some(Self::Semicolon),
            _ => None,
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
