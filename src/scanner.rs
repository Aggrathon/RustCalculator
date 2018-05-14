extern crate natural_constants;
extern crate rand;

use std;
use self::natural_constants::physics;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Power,
	Factorial,
	Number(f64),
	END,
	Unknown,
	Text(String),
	Function(Function),
	Comma,
	Lparen,
	Rparen,
	Equals,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
			Token::Addition => write!(f, "Operator: +"),
			Token::Subtraction => write!(f, "Operator: -"),
			Token::Multiplication => write!(f, "Operator: *"),
			Token::Division => write!(f, "Operator: /"),
			Token::Power => write!(f, "Operator: ^"),
			Token::Number(x) => write!(f, "Value: {}", x),
			Token::Text(ref s) => write!(f, "Text: {}", s),
			Token::END => write!(f, "END"),
			Token::Unknown => write!(f, "Unknown"),
			Token::Factorial => write!(f, "Operator: !"),
			Token::Comma => write!(f, "Symbol: ,"),
			Token::Lparen => write!(f, "Symbol: ("),
			Token::Rparen => write!(f, "Symbol: )"),
			Token::Equals => write!(f, "Symbol: ="),
			Token::Function(ref s) => write!(f, "Function: {}", s),
		}
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Function {
	Log,
	Ln,
	Abs,
	Cos,
	Sin,
	Tan,
	Sqrt,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
			Function::Abs => write!(f, "abs"),
			Function::Ln => write!(f, "ln"),
			Function::Log => write!(f, "log"),
			Function::Cos => write!(f, "cos"),
			Function::Sin => write!(f, "sin"),
			Function::Tan => write!(f, "tan"),
			Function::Sqrt => write!(f, "sqrt"),
		}
	}
}

pub struct Scanner {
	string: String,
	iterator: std::iter::Peekable<std::str::Chars<'static>>,
	index_current: usize,
	index_next: usize,
	token_current: Token,
	token_next: Token,
}

impl Scanner {

	pub fn new(string:String) -> Scanner {
		//This unsafe storage of the char iterator requires that the string is never changed
		let iter = unsafe { std::mem::transmute(string.chars().peekable()) };
		let mut sc = Scanner {
			string: string,
			iterator: iter,
			index_current: 0,
			index_next: 0,
			token_current: Token::Unknown,
			token_next: Token::Unknown
		};
		sc.next();
		sc
	}

	pub fn print_pos(&self) -> String {
		let i = if self.index_current > 0 { self.index_current-1 } else { 0 };
		format!("{}\nPosition: {}\n{}\n{:4$}^", self.token_current, self.index_current, self.string, "", i)
	}

	pub fn next(&mut self) -> &Token {
		self.index_current = self.index_next;
		std::mem::swap(&mut self.token_current, &mut self.token_next);
		self.token_next = self.get_next_token();
		&self.token_current
	}

	#[allow(dead_code)]
	pub fn current(&self) -> &Token {
		&self.token_current
	}

	#[allow(dead_code)]
	pub fn peek(&self) -> &Token {
		&self.token_next
	}

	pub fn has_ended(&self) -> bool {
		match self.token_current {
			Token::END => true,
			_ => false,
		}
	}

	fn get_next_token(&mut self) -> Token {
		self.index_next += 1;
		let oc = match self.iterator.next() {
			Option::None => return Token::END,
			Option::Some(c) => c,
		};
		match oc {
			'+' => Token::Addition,
			'-' => Token::Subtraction,
			'/' => Token::Division,
			'^' => Token::Power,
			'!' => Token::Factorial,
			',' => Token::Comma,
			'(' => Token::Lparen,
			')' => Token::Rparen,
			'[' => Token::Lparen,
			']' => Token::Rparen,
			'=' => Token::Equals,
			'*' => {	// ** == ^
				let od = match self.iterator.peek() {
					Option::None => return Token::Multiplication,
					Option::Some(d) => d,
				}.clone();
				match od {
					'*' => {
						self.iterator.next();
						self.index_next += 1;
						Token::Power
					},
					_ => Token::Multiplication
				}
			},
			_ => {
				let mut s = String::with_capacity(8);
				s.push(oc);
				if oc.is_numeric() || oc == '.' {
					loop {
						match self.iterator.peek() {
							Option::None => break,
							Option::Some(d) => { 
								if d.is_numeric() || *d == '.' || *d == 'E' {
									s.push(*d);
								} else { break; }
							},
						};
						self.iterator.next();
						self.index_next += 1;
					}
					match s.parse::<f64>() {
						Result::Ok(n) => Token::Number(n),
						_ => Token::Unknown,
					}
				}
				else if oc.is_alphabetic() {
					loop {
						match self.iterator.peek() {
							Option::None => break,
							Option::Some(d) => { 
								if d.is_alphanumeric() || *d == '_' {
									s.push(*d);
								} else { break; }
							},
						};
						self.iterator.next();
						self.index_next += 1;
					}
					Scanner::parse_text(s)
				}
				else {
					self.get_next_token()
				}
			},
		}
	}

	fn parse_text(s: String) -> Token {
		let l = s.to_lowercase();
		match l.as_ref() {
			//Constants
			"e" => Token::Number(std::f64::consts::E),
			"pi" => Token::Number(std::f64::consts::PI),
			"tau" => Token::Number(std::f64::consts::PI*2.0),
			"sqrt2" => Token::Number(std::f64::consts::SQRT_2),
			"deg2rad" => Token::Number(std::f64::consts::PI / 180.0),
			"rad2deg" => Token::Number(std::f64::consts::FRAC_1_PI * 180.0),
			"epsilon" => Token::Number(std::f64::EPSILON),
			"c" => Token::Number(physics::speed_of_light_vac),
			"rnd" => Token::Number(rand::random::<f64>()),
			//Functions
			"ln" => Token::Function(Function::Ln),
			"log" => Token::Function(Function::Log),
			"sin" => Token::Function(Function::Sin),
			"cos" => Token::Function(Function::Cos),
			"tan" => Token::Function(Function::Tan),
			"abs" => Token::Function(Function::Abs),
			"sqrt" => Token::Function(Function::Sqrt),
			//Unknown
			_ => Token::Text(s),
		}
	}
}