
pub mod scanner;
use scanner::Scanner;

pub fn calculate(input: String) {
	let mut scanner = Scanner::new(input);
	while !scanner.has_ended() {
		scanner.next();
		scanner.print_pos();
	}
}