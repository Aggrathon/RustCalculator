
mod scanner;
use scanner::Scanner;

pub fn calculate(input: String) {
	let mut scanner = Scanner::from(input);
	while !scanner.has_ended() {
		scanner.next();
		scanner.print_pos();
	}
}