#![feature(iter_advance_by, let_chains, decl_macro)]

use cursor::StringCursor;
use tree::types::Type;

use crate::token::Token;

pub mod cursor;
pub mod token;
pub mod tree;

fn main() {
    let mut cursor = StringCursor::from("fun(typeof SomeGenericClass(int); int) SomeGenericClass(int)");
    let t = Type::from_cursor(&mut cursor);
    println!("{:#?}", t);

    // let mut cursor = StringCursor::from(include_str!("../test.rk"));
    // loop {
    //     let token = cursor.next_token();

    //     if let Err(err) = token {
    //         println!("{:?}", err);
    //         break;
    //     }

    //     let token = token.unwrap();
    //     println!("{:?}", token);
    //     if let Token::Eof = token {
    //         break;
    //     }
    // }
}
