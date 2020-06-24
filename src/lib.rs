pub mod parser;
pub mod scanner;

use parser::Parser;

pub fn calculate<'a>(input: &'a str) -> Parser<'a> {
    Parser::new(input)
}

pub fn calculate_print(input: &str) {
    for res in Parser::new(input) {
        match res {
            Result::Err(s) => {
                println!("{}", s);
                break;
            }
            Result::Ok(v) => {
                println!("{}", v);
            }
        }
    }
}
