mod scanner;
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
	empty
Factor
	Value Factor'
	Function Func'
Func'
	(Expression Func'')
	Factor
Func''
	, Expression
	empty
Factor'
	^ Value Factor'
	! Factor'
	empty
Value
	( Expression )
	- Number
	Number
*/


pub struct Parser {
	scanner: Scanner,
}

impl Parser {
	pub fn new(scanner:Scanner) -> Parser {
		Parser(scanner)
	}
}