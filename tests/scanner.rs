use calc::scanner::*;

#[test]
fn multiplication() {
    let mut sc = Scanner::new(" * ");
    assert_eq!(sc.next(), Token::Multiplication);
    assert_eq!(sc.next(), Token::END);
}

#[test]
fn string() {
    let mut sc = Scanner::new(" asd gfd ");
    assert_eq!(sc.next(), Token::Text("asd"));
    assert_ne!(sc.next(), Token::Text("gfad"));
    assert_eq!(sc.next(), Token::END);
}

#[test]
fn function() {
    let mut sc = Scanner::new(" sin cos ln abs ");
    assert_eq!(sc.next(), Token::Function(Function::Sin));
    assert_eq!(sc.next(), Token::Function(Function::Cos));
    assert_ne!(sc.next(), Token::Function(Function::Log));
    assert_eq!(sc.next(), Token::Function(Function::Abs));
    assert_eq!(sc.next(), Token::END);
}

#[test]
fn random() {
    let num = 20;
    let string: String = std::iter::repeat(" rnd ").take(num).collect();
    let mut sc = Scanner::new(&string);
    let mut ctr = 0;
    loop {
        match sc.next() {
            Token::Number(x) => {
                assert!(x < 1.0);
                assert!(x >= 0.0);
            }
            Token::END => break,
            _ => panic!("Invalid token"),
        }
        ctr += 1;
    }
    assert_eq!(ctr, num);
}

fn close(a: f64, b: f64) -> bool {
    f64::abs(a - b) < a.abs() * 0.0001
}

fn unwrap(t: &Token) -> f64 {
    match *t {
        Token::Number(x) => x,
        _ => panic!("Token is not a number ({})", t),
    }
}

#[test]
fn value() {
    let mut sc = Scanner::new("1 pi 2E2 1.1 1.2");
    assert!(close(unwrap(&sc.next()), 1.0));
    assert!(close(unwrap(&sc.next()), std::f64::consts::PI));
    assert!(close(unwrap(&sc.next()), 200.0));
    assert!(close(unwrap(&sc.next()), 1.1));
    assert!(!close(unwrap(&sc.next()), 1.1));
}

#[test]
fn mix() {
    let mut sc = Scanner::new("1pi(asd!");
    assert!(close(unwrap(&sc.next()), 1.0));
    assert!(close(unwrap(&sc.next()), std::f64::consts::PI));
    assert_eq!(sc.next(), Token::Lparen);
    assert_eq!(sc.next(), Token::Text("asd"));
    assert_eq!(sc.next(), Token::Factorial);
    assert_eq!(sc.next(), Token::END);
}

#[test]
fn peek() {
    let mut sc = Scanner::new("1pi(asd!");
    assert!(close(unwrap(&sc.next()), 1.0));
    assert!(close(unwrap(&sc.peek()), std::f64::consts::PI));
    assert!(close(unwrap(&sc.next()), std::f64::consts::PI));
    assert_eq!(sc.peek(), Token::Lparen);
    assert_eq!(sc.next(), Token::Lparen);
    assert_eq!(sc.peek(), Token::Text("asd"));
    assert_eq!(sc.next(), Token::Text("asd"));
    assert_eq!(sc.peek(), Token::Factorial);
    assert_eq!(sc.next(), Token::Factorial);
    assert_eq!(sc.peek(), Token::END);
    assert_eq!(sc.next(), Token::END);
}

#[test]
fn current() {
    let mut sc = Scanner::new("1pi(asd!");
    assert!(close(unwrap(&sc.next()), 1.0));
    assert!(close(unwrap(&sc.current()), 1.0));
    assert!(close(unwrap(&sc.next()), std::f64::consts::PI));
    assert!(close(unwrap(&sc.current()), std::f64::consts::PI));
    assert_eq!(sc.peek(), Token::Lparen);
    assert_eq!(sc.next(), Token::Lparen);
    assert_eq!(sc.current(), Token::Lparen);
    assert_eq!(sc.peek(), Token::Text("asd"));
    assert_eq!(sc.next(), Token::Text("asd"));
    assert_eq!(sc.current(), Token::Text("asd"));
}

#[test]
fn end() {
    let mut sc = Scanner::new("");
    assert_eq!(sc.peek(), Token::END);
    assert_eq!(sc.next(), Token::END);
    assert_eq!(sc.current(), Token::END);
    assert_eq!(sc.peek(), Token::END);
    assert_eq!(sc.next(), Token::END);
    assert_eq!(sc.current(), Token::END);
}
