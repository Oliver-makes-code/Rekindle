#![feature(
    iter_advance_by,
    let_chains
)]

use cursor::StringCursor;
use token::Token;

mod cursor;
mod token;

fn main() {
    let mut cursor = StringCursor::from("Hello, world!\n This is fun!");
    while let Some(token) = Token::from_cursor(&mut cursor) {
        println!("{:?}", token);
    }
}
