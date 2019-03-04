mod lexer;
mod parser;
mod interact;

use lexer::{Lexer, token::Tokens};
use parser::Parser;
use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let text = read_file("testv3.CSML".to_owned()).unwrap();
    // let split: Vec<&str> = dbg!(text.split('\n').collect());
    let lex_tokens = Lexer::lex_tokens(text.as_bytes());

    interact::test_json();

    match lex_tokens {
        Ok((_complete, t) ) => {
            let tokens = Tokens::new(&t);
            let (_ ,ret) = dbg!(Parser::parse_tokens(tokens).unwrap());
            for truc in ret.iter()
            {
                println!(" >>>> {:?}\n", truc);
            }
        },
        Err(e) => { println!("{:?}", e) }
    };
}
