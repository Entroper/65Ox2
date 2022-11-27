use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::iter::Peekable;

pub mod tokenizer;
use tokenizer::Token;

pub struct Assembler {
	pub labels: HashMap<String, u16>,
	pub variables: HashMap<String, u16>
}

impl Assembler {
	pub fn new() -> Assembler {
		Assembler {
			labels: HashMap::new(),
			variables: HashMap::new()
		}
	}

	pub fn assemble<T>(&mut self, lines: T) -> Result<(), Error>
	where T: Iterator<Item = Result<String, Error>>
	{
		let mut code_address = 0;
		for line in lines {
			let line = line?;
			println!("{code_address:0>4X}: {line}");
	
			// Strip out comments.
			let line = match line.find(";") {
				Some(pos) => &line[..pos],
				None => line.as_str()
			};

			let tokens = tokenizer::tokenize(line)?;
			if tokens.len() == 0 {
				continue;
			}

			// Is it a directive?
			if let Some(Token::Dot) = tokens.get(0) {
				continue; // not implemented yet
			}
	
			// Is it an assignment?
			if let Some(equal_pos) = tokens.iter().position(|t| *t == Token::Equal) {
				if equal_pos > 1 {
					return Err(Error::new(ErrorKind::InvalidInput, format!("Could not assign value to {:?}", &tokens[..equal_pos])));
				}
				let first = tokens.get(0).unwrap();
				let variable = match first {
					Token::Identifier(name) => name,
					_ => return Err(Error::new(ErrorKind::InvalidInput, format!("Expected variable name on left side of =, got {:?}", first)))
				};
				let rhs = &tokens[equal_pos+1..];
				let value = self.evaluate_expression(&mut rhs.iter().peekable(), 0)?;
				self.variables.insert(variable.to_string(), value);
				continue;
			}

			let mut i = 0;
			// Read any labels present.
			loop {
				if let Some(Token::Identifier(name)) = tokens.get(i) {
					match tokens.get(i+1) {
						Some(Token::Colon) => {
							// Make sure it's not something like BEQ :+ or :-
							if let Some(Token::Plus) | Some(Token::Minus) = tokens.get(i+2) {
								break;
							}
							self.labels.insert(name.to_string(), code_address);
							i += 2;
							continue;
						},
						_ => break
					}
				}
				else {
					break;
				}
			}

			// Sometimes labels are unnamed.
			if let Some(Token::Colon) = tokens.get(i) {
				i += 1;
			}

			if tokens.get(i).is_none() {
				continue;
			}

			// Now read an instruction.
			if let Some(Token::Identifier(_instruction)) = tokens.get(i) {
				// All instructions are one byte for the instruction, possibly more for a parameter.
				code_address = code_address + 1;

				// Sometimes there are no parameters.
				if let None = tokens.get(i+1) {
					continue;
				}

				// We might have a branch to :+ or :- or something.
				if let Some(Token::Colon) = tokens.get(i+1) {
					code_address = code_address + 1;
					continue;
				}

				// There may be an expression, optionally preceded by #, optionally followed by ,
				let expression = match tokens.iter().position(|t| *t == Token::Comma) {
					Some(p) => &tokens[i+1..p],
					None => &tokens[i+1..]
				};
				let expression = match expression.get(0) {
					Some(Token::Pound) => &expression[1..],
					_ => expression
				};
				let parameter = self.evaluate_expression(&mut expression.iter().peekable(), 0)?;
				code_address = code_address + if parameter > 0xFF { 2 } else { 1 };
			}
			else {
				return Err(Error::new(ErrorKind::InvalidInput, format!("Unexpected token {:?}", tokens.get(0).unwrap())));
			}
		}	

		Ok(())
	}

/* Expression evaluation:

	<expression> ::= <term> { (+|-) <expression> }

	<term> ::= <factor> { (*|/) <term> }

	<factor> ::= <value> | ( <expression> )

	<value> ::= <identifier> | <number>
*/

	fn evaluate_expression<'a, T: Iterator<Item = &'a Token>>(&self, expression: &mut Peekable<T>, level: usize) -> Result<u16, Error> {
		let lhs = self.evaluate_term(expression, level)?;
		let operator = expression.next();
		match operator {
			Some(Token::Plus) => Ok(lhs + self.evaluate_expression(expression, level)?),
			Some(Token::Minus) => Ok(lhs - self.evaluate_expression(expression, level)?),
			Some(Token::RightParen) => if level != 0 { Ok(lhs) } else { Err(Error::new(ErrorKind::InvalidInput, "Parentheses mismatch")) },
			Some(_) => Err(Error::new(ErrorKind::InvalidInput, format!("Could not evaluate expression, unexpected token {:?}", operator))),
			None => if level == 0 { Ok(lhs) } else { Err(Error::new(ErrorKind::InvalidInput, "Parentheses mismatch")) }
		}
	}

	fn evaluate_term<'a, T: Iterator<Item = &'a Token>>(&self, expression: &mut Peekable<T>, level: usize) -> Result<u16, Error> {
		let lhs = self.evaluate_factor(expression, level)?;
		let operator = expression.peek();
		match operator {
			Some(Token::Asterisk) => {
				expression.next();
				Ok(lhs * self.evaluate_term(expression, level)?)
			},
			Some(Token::Slash) => {
				expression.next();
				Ok(lhs / self.evaluate_term(expression, level)?)
			},
			Some(Token::RightParen) => {
				if level != 0 { Ok(lhs) } else { Err(Error::new(ErrorKind::InvalidInput, "Parentheses mismatch")) }
			},
			Some(_) => Ok(lhs),
			None => if level == 0 { Ok(lhs) } else { Err(Error::new(ErrorKind::InvalidInput, "Parentheses mismatch")) }
		}
	}

	fn evaluate_factor<'a, T: Iterator<Item = &'a Token>>(&self, expression: &mut Peekable<T>, level: usize) -> Result<u16, Error> {
		let token = expression.next();
		match token {
			Some(Token::Number(n)) => Ok(*n),
			Some(Token::Identifier(name)) =>
				if let Some(n) = self.variables.get(name) { Ok(*n) }
				else if let Some(n) = self.labels.get(name) { Ok(*n) }
				else { Err(Error::new(ErrorKind::InvalidInput, format!("Unknown variable {:?}", name))) },
			Some(Token::LeftParen) => self.evaluate_expression(expression, level+1),
			Some(_) => Err(Error::new(ErrorKind::InvalidInput, format!("Could not evaluate expression, unexpected token {:?}", token))),
			None => Err(Error::new(ErrorKind::InvalidInput, "Incomplete expression"))
		}
	}
}
