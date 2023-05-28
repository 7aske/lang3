use std::env;
use crate::lexer::Lexer;

mod iterator;
mod lexer;
mod token;
mod util;
mod source;

use crate::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let string = std::fs::read_to_string(&args[1]).expect("Failed to read file");

    let mut lexer =  Lexer::new(&string);
    let mut tokens = Vec::<Token>::new();

    while let Some(res) = lexer.next_token() {
        if res.is_err() {
            let err = res.err().unwrap();
            println!("{}", err);
            break;
        }
        tokens.push(res.unwrap());
    }

    println!("{:?}", tokens);
}
