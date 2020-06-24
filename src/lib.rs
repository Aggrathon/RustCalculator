pub mod parser;
pub mod scanner;

use parser::Parser;

pub fn calculate_single(input: &str) -> Result<f64, String> {
    let mut p = Parser::new(input);
    match p.parse()? {
        Some(v) => Result::Ok(v),
        None => Result::Err(String::from("Nothing to calculate")),
    }
}

pub fn calculate_print(input: &str) {
    let mut p = Parser::new(input);
    loop {
        let res = p.parse();
        match res {
            Result::Err(s) => {
                println!("{}", s);
                break;
            }
            Result::Ok(Option::Some(v)) => {
                println!("{}", v);
            }
            Result::Ok(Option::None) => {
                break;
            }
        }
    }
}
