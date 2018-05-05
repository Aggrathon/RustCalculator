extern crate calc;
use std::env;


fn main() {
	let combine: String = combine_args();
    calc::calculate(combine);
}

fn combine_args() -> (String) {
	let args: Vec<String> = env::args().collect();
	return args[1..].join(" ");
}