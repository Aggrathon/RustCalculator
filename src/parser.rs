use std;

use crate::scanner::{Function, Operator, Scanner, Token};

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

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    lexiographic_table: std::collections::HashMap<&'a str, f64>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            scanner: Scanner::new(input),
            lexiographic_table: std::collections::HashMap::new(),
        }
    }

    pub fn from(input: Scanner<'a>) -> Parser<'a> {
        Parser {
            scanner: input,
            lexiographic_table: std::collections::HashMap::new(),
        }
    }

    fn error(self: &mut Self, error: &str) -> Result<f64, String> {
        Result::Err(format!("Error: {}\n{}", error, self.scanner.print_pos()))
    }

    fn expect(self: &mut Self, t2: Token, reason: &str) -> Result<f64, String> {
        if self.scanner.next() == t2 {
            Result::Ok(0.0)
        } else {
            self.error(reason)
        }
    }

    fn expr(self: &mut Self) -> Result<f64, String> {
        match self.scanner.peek() {
            Token::Comma => {
                self.scanner.next();
                self.expr()
            }
            _ => {
                let v = self.term()?;
                self.expr_(v)
            }
        }
    }

    fn expr_(self: &mut Self, v: f64) -> Result<f64, String> {
        match self.scanner.peek() {
            Token::Operator(Operator::Addition) => {
                self.scanner.next();
                let v = v + self.term()?;
                self.expr_(v)
            }
            Token::Operator(Operator::Subtraction) => {
                self.scanner.next();
                let v = v - self.term()?;
                self.expr_(v)
            }
            _ => Result::Ok(v),
        }
    }

    fn term(self: &mut Self) -> Result<f64, String> {
        let v = self.factor()?;
        self.term_(v)
    }

    fn term_(self: &mut Self, v: f64) -> Result<f64, String> {
        match self.scanner.peek() {
            Token::Operator(Operator::Multiplication) => {
                self.scanner.next();
                let v = v * self.factor()?;
                self.term_(v)
            }
            Token::Operator(Operator::Division) => {
                self.scanner.next();
                let v2 = self.factor()?;
                if v2 == 0.0 {
                    self.error("Division by zero")
                } else {
                    self.term_(v / v2)
                }
            }
            Token::Function(_) | Token::Lparen | Token::Number(_) | Token::Text(_) => {
                let v = v * self.factor()?;
                self.term_(v)
            }
            _ => Result::Ok(v),
        }
    }

    fn factor(self: &mut Self) -> Result<f64, String> {
        let v = self.func()?;
        self.factor_(v)
    }

    fn factor_(self: &mut Self, v: f64) -> Result<f64, String> {
        match self.scanner.peek() {
            Token::Operator(Operator::Power) => {
                self.scanner.next();
                let v = v.powf(self.func()?);
                self.factor_(v)
            }
            Token::Operator(Operator::Factorial) => {
                self.scanner.next();
                if v < 0.0 {
                    self.error("Factorial must be positive")
                } else {
                    let mut r: f64 = 1.0;
                    let mut v: f64 = v.floor();
                    while v > 1.0 {
                        r *= v;
                        v -= 1.0;
                    }
                    self.factor_(r)
                }
            }
            _ => Result::Ok(v),
        }
    }

    fn func(self: &mut Self) -> Result<f64, String> {
        match self.scanner.peek() {
            Token::Function(ref f) => {
                self.scanner.next();
                match *f {
                    Function::Log => {
                        self.expect(Token::Lparen, "Syntax: log(x,y)")?;
                        let v1 = self.expr()?;
                        match self.scanner.peek() {
                            Token::Comma => {
                                self.scanner.next();
                                let v2 = self.expr()?;
                                self.expect(Token::Rparen, "Syntax: log(x,y)")?;
                                Result::Ok(v1.log(v2))
                            }
                            Token::Rparen => {
                                self.scanner.next();
                                Result::Ok(v1.ln())
                            }
                            _ => self.error("Syntax: log(x,y)"),
                        }
                    }
                    Function::Atan2 => {
                        self.expect(Token::Lparen, "Syntax: atan2(y,x)")?;
                        let v1 = self.expr()?;
                        self.expect(Token::Comma, "Syntax: atan2(y,x)")?;
                        let v2 = self.expr()?;
                        self.expect(Token::Rparen, "Syntax: atan2(y,x)")?;
                        Result::Ok(v1.atan2(v2))
                    }
                    Function::Sum => {
                        let mut v: f64 = 0.0;
                        self.expect(Token::Lparen, "Syntax: sum(x,y,...)")?;
                        loop {
                            v += self.expr()?;
                            if self.scanner.peek() == Token::Rparen {
                                self.expect(Token::Rparen, "Syntax: sum(x,y,...)")?;
                                break;
                            }
                            self.expect(Token::Comma, "Syntax: sum(x,y,...)")?;
                        }
                        Result::Ok(v)
                    }
                    Function::Mean => {
                        let mut v: f64 = 0.0;
                        let mut c: f64 = 0.0;
                        self.expect(Token::Lparen, "Syntax: sum(x,y,...)")?;
                        loop {
                            v += self.expr()?;
                            c += 1.0;
                            if self.scanner.peek() == Token::Rparen {
                                self.expect(Token::Rparen, "Syntax: sum(x,y,...)")?;
                                break;
                            }
                            self.expect(Token::Comma, "Syntax: sum(x,y,...)")?;
                        }
                        Result::Ok(v / c)
                    }
                    Function::Product => {
                        let mut v: f64 = 1.0;
                        self.expect(Token::Lparen, "Syntax: sum(x,y,...)")?;
                        loop {
                            v *= self.expr()?;
                            if self.scanner.peek() == Token::Rparen {
                                self.expect(Token::Rparen, "Syntax: sum(x,y,...)")?;
                                break;
                            }
                            self.expect(Token::Comma, "Syntax: sum(x,y,...)")?;
                        }
                        Result::Ok(v)
                    }
                    _ => {
                        let v = self.func()?;
                        match *f {
                            Function::Ln => Result::Ok(v.ln()),
                            Function::Exp => Result::Ok(std::f64::consts::E.powf(v)),
                            Function::Abs => Result::Ok(v.abs()),
                            Function::Sqrt => {
                                if v < 0.0 {
                                    self.error(&format!("Cannot handle negative values ({})", v))
                                } else {
                                    Result::Ok(v.sqrt())
                                }
                            }
                            Function::Cos => Result::Ok(v.cos()),
                            Function::Sin => Result::Ok(v.sin()),
                            Function::Tan => Result::Ok(v.tan()),
                            Function::Asin => {
                                if v <= 1.0 && v >= -1.0 {
                                    Result::Ok(v.asin())
                                } else {
                                    self.error(&format!("Value outside range (-1 <= {} <= 1)", v))
                                }
                            }
                            Function::Acos => {
                                if v <= 1.0 && v >= -1.0 {
                                    Result::Ok(v.acos())
                                } else {
                                    self.error(&format!("Value outside range (-1 <= {} <= 1)", v))
                                }
                            }
                            Function::Atan => Result::Ok(v.atan()),
                            _ => self.error("Unknown function"),
                        }
                    }
                }
            }
            _ => self.value(),
        }
    }

    fn value(self: &mut Self) -> Result<f64, String> {
        match self.scanner.peek() {
            Token::Number(x) => {
                self.scanner.next();
                Result::Ok(x)
            }
            Token::Operator(Operator::Subtraction) => {
                self.scanner.next();
                Result::Ok(-self.value()?)
            }
            Token::Lparen => {
                self.scanner.next();
                let v = self.expr()?;
                self.expect(Token::Rparen, "Expected Right Parenthesis")?;
                Result::Ok(v)
            }
            Token::Bar => {
                self.scanner.next();
                let v = self.expr()?;
                self.expect(Token::Bar, "Expected |")?;
                Result::Ok(v.abs())
            }
            Token::Text(_) => self.id(),
            _ => self.error("Expected a number or parenthesis"),
        }
    }

    fn id(self: &mut Self) -> Result<f64, String> {
        match self.scanner.peek() {
            Token::Text(s) => {
                self.scanner.next();
                match self.scanner.peek() {
                    Token::Equals => {
                        self.scanner.next();
                        let v = self.expr()?;
                        self.lexiographic_table.insert(s, v);
                        Result::Ok(v)
                    }
                    _ => match self.lexiographic_table.get(s) {
                        Option::Some(v) => return Result::Ok(*v),
                        Option::None => self.error("Unknown variable or constant"),
                    },
                }
            }
            _ => self.error("Expexted a name or identifier"),
        }
    }
}

impl std::iter::Iterator for Parser<'_> {
    type Item = Result<f64, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scanner.peek() {
            Token::END => Option::None,
            _ => Option::Some(self.expr()), //TODO: If error, seek until end of expression
        }
    }
}
