use std::fmt::Debug;

use self::string::{cursor_match, SrcStr, StringCursor};

pub mod string;

#[derive(Clone)]
pub struct Token {
    pub value: TokenType,
    pub src: SrcStr,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Comment,
    Identifier,
    Keyword(Keyword),
    Number,
    String,
    Symbol(Symbol),
    Whitespace,
    UnknownChar,
    Eof
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    As,
    Class,
    Defer,
    Else,
    Enum,
    For,
    Fun,
    If,
    In,
    Impl,
    Import,
    Let,
    Loop,
    Mut,
    Namespace,
    New,
    Object,
    Pub,
    This,
    ThisUpper,
    Trait,
    Type,
    When,
    While,
}

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    And,                   // &&
    AndAssign,             // &=
    AndBitwise,            // &
    Arrow,                 // ->
    Assign,                // =
    BracketOpen(Bracket),  // [ { (
    BracketClose(Bracket), // ] } )
    Colon,                 // :
    ColonQuestion,         // :?
    Comma,                 // ,
    Decrement,             // --
    Div,                   // /
    DivAssign,             // /=
    Dollar,                // $
    Dot,                   // .
    Equal,                 // ==
    Greater,               // >
    GreaterEqual,          // >=
    Increment,             // ++
    Less,                  // <
    LessEqual,             // <=
    Minus,                 // -
    MinusAssign,           // -=
    Mult,                  // *
    MultAssign,            // *=
    Not,                   // !
    NotEqual,              // !=
    Or,                    // ||
    OrAssign,              // |=
    OrBitwise,             // |
    Plus,                  // +
    PlusAssign,            // +=
    Question,              // ?
    Semicolon,             // ;
    ShiftLeft,             // <<
    ShiftLeftAssign,       // <<=
    ShiftRight,            // >>
    ShiftRightAssign,      // >>=
    Xor,                   // ^^
    XorAssign,             // ^=
    XorBitwise,            // ^
}

#[derive(Debug, Clone, Copy)]
pub enum Bracket {
    Curly,
    Round,
    Square,
}

fn digit(c: char) -> bool {
    c.is_digit(10)
}

fn spaced_digit(c: char) -> bool {
    digit(c) || c == '_'
}

fn ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn ident_cont(c: char) -> bool {
    ident_start(c) || digit(c) || c == '-' || c == '$'
}

impl Token {
    pub fn from(cursor: &mut StringCursor) -> Option<Self> {
        if cursor.expect_func(char::is_whitespace) {
            cursor.consume_func(char::is_whitespace);

            return Some(Self {
                value: TokenType::Whitespace,
                src: cursor.fold(),
            });
        }

        if cursor.is("//") {
            cursor.consume_until("\n", false);
            return Some(Self {
                value: TokenType::Comment,
                src: cursor.fold(),
            });
        }

        if cursor.is("/*") {
            cursor.consume_until("*/", false)?;
            return Some(Self {
                value: TokenType::Comment,
                src: cursor.fold(),
            });
        }

        if cursor.expect_func(ident_start) {
            cursor.consume_func(ident_cont);

            let str = cursor.fold();

            if let Some(keyword) = Keyword::from(&str) {
                return Some(Self {
                    value: TokenType::Keyword(keyword),
                    src: str,
                });
            }

            return Some(Self {
                value: TokenType::Identifier,
                src: str,
            });
        }

        if cursor.expect_any("\"") {
            cursor.consume_until("\"", true)?;

            return Some(Self {
                value: TokenType::String,
                src: cursor.fold(),
            });
        }

        if cursor.expect_func(digit) {
            cursor.consume_func(spaced_digit);

            if cursor.expect_any(".") {
                cursor.consume_func(spaced_digit);
            }

            return Some(Self {
                value: TokenType::Number,
                src: cursor.fold(),
            });
        }

        if let Some(symbol) = Symbol::from(cursor) {
            return Some(Self {
                value: TokenType::Symbol(symbol),
                src: cursor.fold(),
            });
        }

        let Some(_) = cursor.next() else {
            cursor.last();
            return Some(Self {
                value: TokenType::Eof,
                src: cursor.fold()
            })
        };
        Some(Self {
            value: TokenType::UnknownChar,
            src: cursor.fold()
        })
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Token").field(&self.value).field(&self.src).finish()
    }
}

impl Keyword {
    fn from(str: &str) -> Option<Self> {
        match str {
            "as" => Some(Self::As),
            "class" => Some(Self::Class),
            "defer" => Some(Self::Defer),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "for" => Some(Self::For),
            "fun" => Some(Self::Fun),
            "if" => Some(Self::If),
            "in" => Some(Self::In),
            "impl" => Some(Self::Impl),
            "import" => Some(Self::Import),
            "let" => Some(Self::Let),
            "loop" => Some(Self::Loop),
            "mut" => Some(Self::Mut),
            "namespace" => Some(Self::Namespace),
            "new" => Some(Self::New),
            "object" => Some(Self::Object),
            "pub" => Some(Self::Pub),
            "this" => Some(Self::This),
            "This" => Some(Self::ThisUpper),
            "trait" => Some(Self::Trait),
            "type" => Some(Self::Type),
            "when" => Some(Self::When),
            "while" => Some(Self::While),
            _ => None,
        }
    }
}

impl Symbol {
    fn from(cursor: &mut StringCursor) -> Option<Self> {
        cursor_match!(
            cursor;
            "&&" => Self::And,
            "&=" => Self::AndAssign,
            "->" => Self::Arrow,
            ":?" => Self::ColonQuestion,
            "--" => Self::Minus,
            "/=" => Self::DivAssign,
            "==" => Self::Equal,
            "<=" => Self::GreaterEqual,
            "++" => Self::Increment,
            ">=" => Self::LessEqual,
            "-=" => Self::MinusAssign,
            "*=" => Self::MultAssign,
            "!=" => Self::NotEqual,
            "||" => Self::Or,
            "|=" => Self::OrAssign,
            "+=" => Self::PlusAssign,
            "<<=" => Self::ShiftLeftAssign,
            "<<" => Self::ShiftLeft,
            ">>=" => Self::ShiftRightAssign,
            ">>" => Self::ShiftRight,
            "^^" => Self::Xor,
            "^=" => Self::XorBitwise,
            _ => Self::from_char
        );
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '&' => Some(Self::AndBitwise),
            '=' => Some(Self::Assign),
            '{' => Some(Self::BracketOpen(Bracket::Curly)),
            '(' => Some(Self::BracketOpen(Bracket::Round)),
            '[' => Some(Self::BracketOpen(Bracket::Square)),
            '}' => Some(Self::BracketClose(Bracket::Curly)),
            ')' => Some(Self::BracketClose(Bracket::Round)),
            ']' => Some(Self::BracketClose(Bracket::Square)),
            ':' => Some(Self::Colon),
            ',' => Some(Self::Comma),
            '/' => Some(Self::Div),
            '$' => Some(Self::Dollar),
            '.' => Some(Self::Dot),
            '>' => Some(Self::Greater),
            '<' => Some(Self::Less),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Mult),
            '!' => Some(Self::Not),
            '|' => Some(Self::OrBitwise),
            '+' => Some(Self::Plus),
            '?' => Some(Self::Question),
            ';' => Some(Self::Semicolon),
            '^' => Some(Self::XorBitwise),
            _ => None,
        }
    }
}
