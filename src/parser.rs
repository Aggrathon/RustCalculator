
use std;

use scanner::Token;
use scanner::Scanner;
use scanner::Function;

/*
___Pattern table___
Expr
	Term Expr'
	, Expr
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
	| Expression |
	- Number
	Number
	Id
Id
	Text
	Text = Expr
*/

pub enum ParseResult {
	Parsing,
	Value(f64),
	Pair(String, f64),
	Error(String),
	Ended,
}

pub struct Parser {
	scanner: Scanner,
	result: ParseResult,
	lexiographic_table: std::collections::HashMap<String, f64>,
}

impl Parser {
	pub fn new<S: Into<String>>(input: S) -> Parser {
		Parser { 
			scanner: Scanner::new(input.into()),
			result: ParseResult::Parsing,
			lexiographic_table: std::collections::HashMap::new(),
		}
	}

	pub fn from(input: Scanner) -> Parser {
		Parser {
			scanner: input,
			result: ParseResult::Parsing,
			lexiographic_table: std::collections::HashMap::new(),
		}
	}

	pub fn parse(self: &mut Parser) -> &ParseResult {
		match self.result {
			ParseResult::Ended | ParseResult::Error(_) => return &self.result,
			_ => {}
		};
		match self.scanner.peek().clone() {
			Token::END => self.result = ParseResult::Ended,
			_ => {
				self.result = ParseResult::Parsing;
				let v = self.expr();
				match self.result {
					ParseResult::Error(_) | ParseResult::Value(_) | ParseResult::Pair(_,_) => {},
					_ => match self.scanner.peek().clone() {
						Token::END | Token::Comma => self.result = ParseResult::Value(v),
						_ => self.error("Unexpected symbol"),
					},
				};
			}
		};
		&self.result
	}

	pub fn parse_all(self: &mut Parser) -> &ParseResult {
		loop {
			match self.parse() {
				ParseResult::Ended | ParseResult::Error(_) => break,
				_ => {},
			};
		}
		&self.result
	}

	pub fn get_result(self: &Parser) -> &ParseResult {
		&self.result
	}

	pub fn get_scanner(self: &Parser) -> &Scanner {
		&self.scanner
	}

	fn error<S: Into<String>>(self: &mut Parser, error:S) {
		match self.result {
			ParseResult::Error(_) => {},
			_ => self.result = ParseResult::Error(format!("Error: {}\n{}", error.into(), self.scanner.print_pos())),
		}
	}

	fn expect<S: Into<String>>(self: &mut Parser, t2: Token, reason: S) {
		if *self.scanner.next() == t2 {
		}
		else {
			self.error(reason);
		}
	}

	fn expr(self: &mut Parser) -> f64 {
		match self.scanner.peek().clone() {
			Token::Text(_) => {
				let v = self.term();
				match self.result {
					ParseResult::Error(_) => v,
					ParseResult::Pair(_, v2) => {
						if v == v2 { v }
						else { self.error("Syntax: id = expression"); v }
					},
					_ => self.expr_(v),
				}
			},
			Token::Comma => { self.scanner.next(); self.expr() },
			_ => {
				let v = self.term();
				self.expr_(v)
			},
		}
	}

	fn expr_(self: &mut Parser, v: f64) -> f64 {
		match self.scanner.peek().clone() {
			Token::Addition => {
				self.scanner.next();
				let v = v + self.term();
				self.expr_(v)
			},
			Token::Subtraction => {
				self.scanner.next();
				let v = v - self.term();
				self.expr_(v)
			},
			_ => v
		}
	}

	fn term(self: &mut Parser) -> f64 {
		let v = self.factor();
		self.term_(v)
	}

	fn term_(self: &mut Parser, v: f64) -> f64 {
		match self.scanner.peek().clone() {
			Token::Multiplication =>  {
				self.scanner.next();
				let v = v * self.factor();
				self.term_(v)
			},
			Token::Division => {
				self.scanner.next();
				let v2 = self.factor();
				if v2 == 0.0 {
					self.error("Division by zero");
					0.0
				}
				else {
					self.term_(v/v2)
				}
			},
			Token::Function(_) | Token::Lparen | Token::Number(_) | Token::Text(_) => {
				let v = v * self.factor();
				self.term_(v)
			}
			_ => v,
		}
	}

	fn factor(self: &mut Parser) -> f64 {
		let v = self.func();
		self.factor_(v)
	}

	fn factor_(self: &mut Parser, v: f64) -> f64 {
		match self.scanner.peek().clone() {
			Token::Power => {
				self.scanner.next();
				let v = v.powf(self.func());
				self.factor_(v)
			},
			Token::Factorial => {
				self.scanner.next();
				let mut r: f64 = 1.0;
				let mut v: f64 = v.floor();
				while v > 1.0 {
					r *= v;
					v -= 1.0;
				}
				self.factor_(r)
			}
			_ => v
		}
	}

	fn func(self: &mut Parser) -> f64 {
		match self.scanner.peek().clone() {
			Token::Function(ref f) =>  {
				self.scanner.next();
				match *f {
					Function::Log => {
						self.expect(Token::Lparen, "Syntax: log(x,y)");
						let v1 = self.expr();
						self.expect(Token::Comma, "Syntax: log(x,y)");
						let v2 = self.expr();
						self.expect(Token::Rparen, "Syntax: log(x,y)");
						v1.log(v2)
					},
					_ => {
						let v = self.func();
						match *f {
							Function::Ln => v.ln(),
							Function::Abs => v.abs(),
							Function::Sqrt => v.sqrt(),
							Function::Cos => v.cos(),
							Function::Sin => v.sin(),
							Function::Tan => v.tan(),
							_ => { self.error(String::from("Unknown function")); v},
						}
					}
				}
			},
			_ => self.value()
		}
	}

	fn value(self: &mut Parser) -> f64 {
		match self.scanner.peek().clone() {
			Token::Number(x) => { self.scanner.next(); x },
			Token::Subtraction => { self.scanner.next(); -self.value() },
			Token::Lparen => { 
				self.scanner.next();
				let v = self.expr();
				self.expect(Token::Rparen, "Expected Right Parenthesis");
				v
			},
			Token::Bar => {
				self.scanner.next();
				let v = self.expr();
				self.expect(Token::Bar, "Expected |");
				v.abs()
			}
			Token::Text(_) => self.id(),
			_ => { self.error(String::from("Expected a number or parenthesis")); 0.0 },
		}
	}

	fn id(self: &mut Parser) -> f64 {
		match self.scanner.peek().clone() {
			Token::Text(s) => {
				self.scanner.next();
				match self.scanner.peek().clone() {
					Token::Equals => {
						self.expect(Token::Equals, "Expected =");
						let v = self.expr();
						self.lexiographic_table.insert(s.clone(), v);
						match self.result {
							ParseResult::Error(_) => v,
							_ => { self.result = ParseResult::Pair(s.clone(), v); v }
						}
					},
					_ => {
						match self.lexiographic_table.get(&s) {
							Option::Some(v) => return *v,
							Option::None => {},
						};
						self.error("Unknown variable or constant");
						0.0
					},
				}
			},
			_ => { self.error("Expexted a name or identifier"); 0.0 },
		}
	}
	
}