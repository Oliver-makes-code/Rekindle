#![feature(pattern, decl_macro)]

use std::time::{Instant, Duration};

use token::{string::StringCursor, Token};

pub mod token;

fn run() -> Duration {
    let start = Instant::now();
    let cursor = StringCursor::from(include_str!("../test.rk"));
    let _: Vec<Token> = cursor.into();
    start.elapsed()
}

fn main() {
    let mut avg = run();

    let num_runs = 1_000;

    for _ in 1..num_runs {
        avg += run();
    }
    avg /= num_runs;
    println!("{:?}", avg);
}
