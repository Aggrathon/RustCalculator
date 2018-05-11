extern crate calc;

use calc::parser::*;

fn test(string: &str, value: f64) {
	let v = match *Parser::new(string).parse() {
		ParseResult::Value(v) => v,
		ParseResult::Pair(_, v) => v,
		_ => panic!(),
	};
	assert!((value-v).abs() < 0.000001, "{}: {} != {}", string, v, value);
}

#[test]
fn easy() {
	test("2", 2.0);
	test("1+1", 2.0);
	test("2*1", 2.0);
	test("3-1", 2.0);
	test("6/3", 2.0);
	test("2^1", 2.0);
	test("2**1", 2.0);
	test("4!", 24.0);
	test("(2)", 2.0);	
}

#[test]
fn func() {
	test("cos pi", -1.0);
	test("sin pi", 0.0);
	test("tan pi", 0.0);
	test("pi * rad2deg * deg2rad", 3.14159265);
	test("abs -10", 10.0);
	test("ln 2 - log(2,e)", 0.0);
	test("sqrt2 - sqrt 2", 0.0);
}

#[test]
fn comp_1() {
	test("((2 * 3 - 6)**0.5 + 3.212^2) / 10.0", 1.0316944);
	test("cos cos (cos cos cos pi )", 0.7934803587425656);
	test("((((5+2))*3)+((((((7)))*4))))", 49.0);
	test("log((cos 3 - 5 +7*25), 2*e-e)-ln((cos 3 - 5 +7*25))", 0.0)
}

#[test]
fn no_mul() {
	test("2pi-2*pi", 0.0);
	test("2cos pi", -2.0);
	test("2(pi)-2*pi", 0.0);
	test("e pi-e*pi", 0.0);
	test("2 -5", -3.0);
}