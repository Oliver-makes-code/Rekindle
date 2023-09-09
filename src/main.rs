#![feature(pattern, decl_macro)]

use token::{string::StringCursor, Token};

pub mod token;

fn main() {
    let mut cursor = StringCursor::from(include_str!("../test.rk"));
    let mut tokens = vec![];
    while let Some(token) = Token::from(&mut cursor) {
        tokens.push(token);
    }
}
