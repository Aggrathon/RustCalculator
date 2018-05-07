
pub mod scanner;
pub mod parser;
use scanner::Scanner;

pub fn calculate(input: String) -> f64 {
	let mut scanner = Scanner::new(input);
	parser::parse(&mut scanner)
}

pub fn calculate_print(input: String) {
	println!("{}", calculate(input));
}