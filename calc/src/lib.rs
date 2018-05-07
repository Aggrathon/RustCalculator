
pub mod scanner;
pub mod parser;
use scanner::Scanner;

pub fn calculate(input: String) -> f64 {
	let mut scanner = Scanner::new(input);
	parser::parse(&mut scanner)
}

pub fn calculate_print(input: String) {
	let handler = std::panic::take_hook();
	std::panic::set_hook(Box::new(|_info| {} ));
	let res = std::panic::catch_unwind(|| { 
		println!("{}", calculate(input));
	});
	if res.is_err() {
		println!("Could not calculate");
	}
	std::panic::set_hook(handler);
}