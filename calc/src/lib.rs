
#[derive(PartialEq)]
enum Token {
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Power,
	Number(f64),
	END,
	Unknown,
	Text(String),
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
		}
    }
}

struct Parser {
	string: String,
	iterator: std::iter::Peekable<std::str::Chars<'static>>,
	index: usize,
}

impl Parser {

	fn from(string:String) -> Parser {
		//This unsafe storage of the char iterator requires that the string is never changed
		let iter = unsafe { std::mem::transmute(string.chars().peekable()) };
		Parser{string: string, iterator: iter, index: 0}
	}

	#[allow(dead_code)]
	fn print_pos(&self) {
		println!("Position: {}", self.index);
		println!("{}", self.string);
		println!("{:1$}^", "", self.index-1);
	}

	fn get_next_token(&mut self) -> Token {
		self.index+= 1;
		let oc = match self.iterator.next() {
			Option::None => return Token::END,
			Option::Some(c) => c,
		};
		match oc {
			'+' => Token::Addition,
			'-' => Token::Subtraction,
			'/' => Token::Division,
			'^' => Token::Power,
			'*' => {	// ** == ^
				let od = match self.iterator.peek() {
					Option::None => return Token::Multiplication,
					Option::Some(d) => d,
				}.clone();
				match od {
					'*' => {
						self.iterator.next();
						self.index+= 1;
						Token::Power
					},
					_ => Token::Multiplication
				}
			},
			_ => {
				let mut s = oc.to_string();
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
						self.index+= 1;
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
						self.index+= 1;
					}
					Token::Text(s)
				}
				else {
					self.get_next_token()
				}
			},
		}
	}
}

pub fn calculate(input: String) {
	let mut parser = Parser::from(input);
	loop {
		let t = parser.get_next_token();
		println!("{}", t);
		parser.print_pos();
		if t == Token::END {
			break;
		}
	}
}