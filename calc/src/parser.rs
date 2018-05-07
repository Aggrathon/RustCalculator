
use scanner::Token;
use scanner::Scanner;
use scanner::Function;

/*
___Pattern table___
Expr
	Term Expr'
Expr'
	+ Term Expr'
	- Term Expr'
	empty
Term
	Factor Term'
Term'
	* Factor Term'
	/ Factor Term'
	f/(/n=>Factor Term
	empty
Factor
	Func Factor'
Factor'
	^ Func Factor'
	! Factor'
	empty
Func
	f1 Func
	f2 (Expression , Expression)
	Value
Value
	( Expression )
	- Number
	Number
*/


fn print_error(scanner: &mut Scanner, error:String) -> f64 {
	scanner.next();
	eprintln!("Syntax Error ({})", error);
	scanner.print_pos();
	panic!();
}

fn expect(scanner: &mut Scanner, t2: Token) {
	if *scanner.peek() == t2 {
		scanner.next();
	}
	else {
		print_error(scanner, format!("Expected {}", t2));
	}
}

pub fn parse(scanner: &mut Scanner) -> f64 {
	let v = expr(scanner);
	match scanner.peek().clone() {
		Token::END => v,
		_ => {
			print_error(scanner, String::from("Unexpected symbol"));
			v
		}
	}
}

fn expr(scanner: &mut Scanner) -> f64 {
	let v = term(scanner);
	expr_(scanner, v)
}

fn expr_(scanner: &mut Scanner, v: f64) -> f64 {
	match  scanner.peek().clone() {
		Token::Addition => {
			scanner.next();
			let v = v + term(scanner);
			expr_(scanner, v)
		},
		Token::Subtraction => {
			scanner.next();
			let v = v - term(scanner);
			expr_(scanner, v)
		},
		_ => v
	}
}

fn term(scanner: &mut Scanner) -> f64 {
	let v = factor(scanner);
	term_(scanner, v)
}

fn term_(scanner: &mut Scanner, v: f64) -> f64 {
	match  scanner.peek().clone() {
		Token::Multiplication =>  {
			scanner.next();
			let v = v * factor(scanner);
			term_(scanner, v)
		},
		Token::Division => {
			scanner.next();
			let v = v / factor(scanner);
			term_(scanner, v)
		},
		Token::Function(_) | Token::Lparen | Token::Number(_) => {
			let v = v * factor(scanner);
			term_(scanner, v)
		}
		_ => v,
	}
}

fn factor(scanner: &mut Scanner) -> f64 {
	let v = func(scanner);
	factor_(scanner, v)
}

fn factor_(scanner: &mut Scanner, v: f64) -> f64 {
	match scanner.peek().clone() {
		Token::Power => {
			scanner.next();
			let v = v.powf(func(scanner));
			factor_(scanner, v)
		},
		Token::Factorial => {
			scanner.next();
			let mut r: f64 = 1.0;
			let mut v: f64 = v.floor();
			while v > 1.0 {
				r *= v;
				v -= 1.0;
			}
			factor_(scanner, r)
		}
		_ => v
	}
}

fn func(scanner: &mut Scanner) -> f64 {
	match scanner.peek().clone() {
		Token::Function(ref f) =>  {
			scanner.next();
			match *f {
				Function::Log => {
					expect(scanner, Token::Lparen);
					let v1 = expr(scanner);
					expect(scanner, Token::Comma);
					let v2 = expr(scanner);
					expect(scanner, Token::Rparen);
					v1.log(v2)
				},
				_ => {
					let v = func(scanner);
					match *f {
						Function::Ln => v.ln(),
						Function::Abs => v.abs(),
						Function::Sqrt => v.sqrt(),
						Function::Cos => v.cos(),
						Function::Sin => v.sin(),
						Function::Tan => v.tan(),
						_ => print_error(scanner, String::from("Too few function arguments")),
					}
				}
			}
		},
		_ => value(scanner)
	}
}

fn value(scanner: &mut Scanner) -> f64 {
	match scanner.peek().clone() {
		Token::Number(x) => { scanner.next(); x },
		Token::Subtraction => { scanner.next(); -value(scanner) },
		Token::Lparen => { 
			scanner.next();
			let v = expr(scanner);
			expect(scanner, Token::Rparen);
			v
		},
		_ => print_error(scanner, String::from("Expected a number or parenthesis")),
	}
}