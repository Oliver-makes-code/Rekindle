use crate::cursor::StringCursor;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Ident(Location, String),
    Keyword(Location, Keyword),
    Number(Location, String),
    String(Location, String, char),
    Symbol(Location, Symbol),
    Whitespace(Location, char),
    Comment(Location, String),
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenError {
    UnclosedString(Location),
    MultiDottedNumber(Location),
    UnknownChar(Location, char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Location {
    pub start: usize,
    pub end: usize,
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
    pub fn get_start_idx(&self) -> Option<usize> {
        match self {
            Self::Eof => None,
            Self::Ident(location, _)
            | Self::Keyword(location, _)
            | Self::Number(location, _)
            | Self::String(location, _, _)
            | Self::Symbol(location, _)
            | Self::Whitespace(location, _)
            | Self::Comment(location, _) => Some(location.start),
        }
    }

    pub fn start_idx_or(&self, default: usize) -> usize {
        self.get_start_idx().unwrap_or(default)
    }

    pub fn from_cursor(cursor: &mut StringCursor) -> Result<Self, TokenError> {
        let start_pos = cursor.idx;
        if cursor.idx >= cursor.string.len() {
            return Ok(Self::Eof);
        }
        let char = cursor.current().unwrap();

        if char.is_whitespace() {
            Self::extract_whitespace(cursor, char);
            return Ok(Self::Whitespace(Location::new(start_pos, cursor.idx), char));
        }

        if char == '/' {
            cursor.advance();
            let char = cursor.current();
            if let Some(char) = char && (char == '/' || char == '*') {
                cursor.advance();
                return Ok(Self::Comment(Location::new(start_pos, cursor.idx), Self::extract_comment(cursor, char == '*')));
            } else {
                cursor.idx = start_pos;
            }
        }

        if let Some(symbol) = Symbol::from_cursor(cursor) {
            return Ok(Token::Symbol(Location::new(start_pos, cursor.idx), symbol));
        }

        if char.is_alphabetic() || char == '_' {
            let ident = Self::extract_ident(cursor);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(keyword) = Keyword::from_str(&ident) {
                return Ok(Self::Keyword(loc, keyword));
            }
            return Ok(Self::Ident(loc, ident));
        }

        if char.is_numeric() {
            let num = Self::extract_number(cursor);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(num) = num {
                return Ok(Self::Number(loc, num));
            }
            return Err(TokenError::MultiDottedNumber(loc));
        }

        if char == '\'' || char == '"' {
            let str = Self::extract_string(cursor, char);
            let loc = Location::new(start_pos, cursor.idx);
            if let Some(str) = str {
                return Ok(Self::String(loc, str, char));
            }
            return Err(TokenError::UnclosedString(loc));
        }

        Err(TokenError::UnknownChar(
            Location::new(start_pos, cursor.idx),
            char,
        ))
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
