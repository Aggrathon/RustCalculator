use natural_constants::physics;

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Operator(Operator),
    Number(f64),
    Unknown,
    Text(&'a str),
    Function(Function),
    Comma,
    Lparen,
    Rparen,
    Equals,
    Bar,
    END,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Token::Number(x) => write!(f, "Value: {}", x),
            Token::Text(ref s) => write!(f, "Text: {}", s),
            Token::END => write!(f, "END"),
            Token::Unknown => write!(f, "Unknown"),
            Token::Comma => write!(f, "Symbol: ,"),
            Token::Lparen => write!(f, "Symbol: ("),
            Token::Rparen => write!(f, "Symbol: )"),
            Token::Equals => write!(f, "Symbol: ="),
            Token::Bar => write!(f, "Symbol: |"),
            Token::Function(ref s) => write!(f, "Function: {}", s),
            Token::Operator(ref s) => write!(f, "Operator: {}", s),
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
    Acos,
    Asin,
    Atan,
    Atan2,
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
            Function::Asin => write!(f, "asin"),
            Function::Acos => write!(f, "acos"),
            Function::Atan => write!(f, "atan"),
            Function::Atan2 => write!(f, "atan2"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Power,
    Factorial,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Operator::Addition => write!(f, "+"),
            Operator::Subtraction => write!(f, "-"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
            Operator::Power => write!(f, "^"),
            Operator::Factorial => write!(f, "!"),
        }
    }
}

pub struct Scanner<'a> {
    string: &'a str,
    iterator: std::iter::Peekable<std::str::CharIndices<'a>>,
    index_current: usize,
    index_next: usize,
    token_current: Token<'a>,
    token_next: Token<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(string: &'a str) -> Scanner<'a> {
        let mut sc = Scanner {
            string: string,
            iterator: string.char_indices().peekable(),
            index_current: 0,
            index_next: 0,
            token_current: Token::Unknown,
            token_next: Token::Unknown,
        };
        sc.next();
        sc
    }

    pub fn print_pos(&self) -> String {
        let i = if self.index_current > 0 {
            self.index_current - 1
        } else {
            0
        };
        format!(
            "{}\nPosition: {}\n{}\n{:4$}^",
            self.token_current, self.index_current, self.string, "", i
        )
    }

    pub fn next(self: &mut Self) -> &'a Token {
        self.index_current = self.index_next;
        self.token_current = self.token_next;
        self.token_next = self.get_next_token();
        &self.token_current
    }

    #[allow(dead_code)]
    pub fn current(&self) -> &'a Token {
        &self.token_current
    }

    pub fn peek(&self) -> &'a Token {
        &self.token_next
    }


    fn get_next_token(&mut self) -> Token<'a> {
        let oc = match self.iterator.next() {
            Option::None => return Token::END,
            Option::Some(c) => c,
        };
        self.index_next = oc.0;
        match oc.1 {
            '+' => Token::Operator(Operator::Addition),
            '-' => Token::Operator(Operator::Subtraction),
            '/' => Token::Operator(Operator::Division),
            '^' => Token::Operator(Operator::Power),
            '!' => Token::Operator(Operator::Factorial),
            ',' | ';' => Token::Comma,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '[' => Token::Lparen,
            ']' => Token::Rparen,
            '=' => Token::Equals,
            '|' => Token::Bar,
            '*' => {
                // ** == ^
                match self.iterator.peek() {
                    Option::None => return Token::Operator(Operator::Multiplication),
                    Option::Some(d) => match d.1 {
                        '*' => {
                            self.iterator.next();
                            Token::Operator(Operator::Power)
                        }
                        _ => Token::Operator(Operator::Multiplication),
                    },
                }
            }
            _ => {
                let mut end = oc.0;
                if oc.1.is_numeric() || oc.1 == '.' {
                    loop {
                        match self.iterator.peek() {
                            Option::None => break,
                            Option::Some(d) => {
                                if d.1.is_numeric() || d.1 == '.' || d.1 == 'E' {
                                    end = oc.0;
                                } else {
                                    break;
                                }
                            }
                        };
                        self.iterator.next();
                    }
                    match self.string[self.index_next..(end + 1)].parse::<f64>() {
                        Result::Ok(n) => Token::Number(n),
                        _ => Token::Unknown,
                    }
                } else if oc.1.is_alphabetic() {
                    loop {
                        match self.iterator.peek() {
                            Option::None => break,
                            Option::Some(d) => {
                                if d.1.is_alphanumeric() || d.1 == '_' {
                                    end = oc.0;
                                } else {
                                    break;
                                }
                            }
                        };
                        self.iterator.next();
                    }
                    Scanner::parse_text(&self.string[self.index_next..(end + 1)])
                } else {
                    self.get_next_token()
                }
            }
        }
    }

    fn parse_text(s: &'a str) -> Token<'a> {
        let l = s.to_lowercase();
        match l.as_ref() {
            //Constants
            "e" => Token::Number(std::f64::consts::E),
            "pi" => Token::Number(std::f64::consts::PI),
            "tau" => Token::Number(std::f64::consts::PI * 2.0),
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
            "asin" => Token::Function(Function::Asin),
            "acos" => Token::Function(Function::Acos),
            "atan" => Token::Function(Function::Atan),
            "atan2" => Token::Function(Function::Atan2),
            "arcsin" => Token::Function(Function::Asin),
            "arccos" => Token::Function(Function::Acos),
            "arctan" => Token::Function(Function::Atan),
            "arctan2" => Token::Function(Function::Atan2),
            //Unknown
            _ => Token::Text(s),
        }
    }
}
