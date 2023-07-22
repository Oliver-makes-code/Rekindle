#![feature(
    iter_advance_by,
    let_chains
)]

use cursor::StringCursor;
use token::Token;

mod cursor;
mod token;

fn main() {
    let mut cursor = StringCursor::from("Hello, world!\n\tThis is fun! 123.45 67.89");
    loop {
        let token = Token::from_cursor(&mut cursor);
        match token {
            Ok(token) => {
                println!("{:?}", token);
                if token == Token::Eof {
                    break
                }
            },
            Err(err) => {
                print!("{:?}", err);
                break
            }
        }
    }
}
