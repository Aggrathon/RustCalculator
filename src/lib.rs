pub mod parser;
pub mod scanner;

use parser::ParseResult;
use parser::Parser;

pub fn calculate_single(input: String) -> f64 {
    let mut parser = Parser::new(input);
    match *parser.parse() {
        ParseResult::Value(v) => v,
        ParseResult::Pair(_, v) => v,
        _ => 0.0,
    }
}

pub fn calculate_print(input: String) {
    let mut p = Parser::new(input);
    loop {
        match p.parse() {
            ParseResult::Error(s) => {
                println!("{}", s);
                break;
            }
            ParseResult::Value(v) => {
                println!("{}", v);
            }
            ParseResult::Pair(k, v) => {
                println!("{} = {}", k, v);
            }
            ParseResult::Parsing => {
                println!("Still Parsing");
            }
            ParseResult::Ended => {
                break;
            }
        }
    }
}
