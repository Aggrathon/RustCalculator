extern crate calc;

use calc::parser::*;

fn test(string: &str, value: f64) {
	let mut p = Parser::new(string);
	let v = match p.parse() {
		ParseResult::Value(v) => v,
		ParseResult::Pair(_, v) => v,
		ParseResult::Error(s) => panic!(s.clone()),
		ParseResult::Ended => panic!("Evaluation ended early"),
		ParseResult::Parsing => panic!("Incomplete evaluation"),
	};
	assert!((value-v).abs() < 0.000001, "{}: {} != {}", string, v, value);
}

fn test_multiple(string: &str, values: &[f64]) {
	let mut p = Parser::new(string);
	for v in values {
		let v2 = match p.parse() {
			ParseResult::Value(v) => v,
			ParseResult::Pair(_, v) => v,
			ParseResult::Error(s) => panic!(s.clone()),
			ParseResult::Ended => panic!("Evaluation ended early"),
			ParseResult::Parsing => panic!("Incomplete evaluation"),
		};
		assert!((v-v2).abs() < 0.000001, "{}: {} != {}", string, v2, v);
	}
}

fn test_fail(string: &str) {
	let mut p = Parser::new(string);
	match p.parse() {
		ParseResult::Value(v) => panic!("Should not return a valid answer ({} != {})", string, v),
		ParseResult::Pair(_, v) => panic!("Should not return a valid answer ({} != {})", string, v),
		ParseResult::Error(_) => return,
		ParseResult::Ended => return,
		ParseResult::Parsing => return,
	}
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
	test("atan 5", 1.373400766945016);
	test("asin 1", 1.5707963267948966);
	test("acos 1", 0.0);
	test("atan2(2,3)", 0.5880026035475675);
}

#[test]
fn func_fail() {
	test_fail("ln(2,e) - log(2)");
	test_fail("sqrt(-1)");
	test_fail("asin 10");
	test_fail("acos 10");
	test_fail("atan2 3");
}

#[test]
fn comp_1() {
	test("((2 * 3 - 6)**0.5 + 3.212^2) / 10.0", 1.0316944);
	test("cos cos (cos cos cos pi )", 0.7934803587425656);
	test("((((5+2))*3)+((((((7)))*4))))", 49.0);
	test("log((cos 3 - 5 +7*25), 2*e-e)-ln((cos 3 - 5 +7*25))", 0.0);
	test("|5|", 5.0);
	test("|-5|", 5.0);
	test("|10-5-5*2|", 5.0);
}

#[test]
fn no_mul() {
	test("2pi-2*pi", 0.0);
	test("2cos pi", -2.0);
	test("2(pi)-2*pi", 0.0);
	test("e pi-e*pi", 0.0);
	test("2 -5", -3.0);
}

#[test]
fn multiple() {
	test("x = 5, x*x/5", 5.0);
	test("5*5/5", 5.0);
	test_multiple("x = 5, x*x/5", &[5.0, 5.0]);
	test_multiple("(5 + 5 + 5)/3, sqrt(5*5)", &[5.0, 5.0]);
	test_multiple("x = 5, x*x/5", &[5.0, 5.0]);
	test_multiple("x = 5, y = x*x/5", &[5.0, 5.0]);
	test_multiple("x = 5, y = x*x/5, 5, 5, x*5 / y", &[5.0, 5.0, 5.0, 5.0, 5.0]);
}