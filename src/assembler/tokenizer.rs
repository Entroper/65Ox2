use std::io::{Error, ErrorKind};
use std::vec::Vec;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Plus,
	Minus,
	Asterisk,
	Slash,
	LeftParen,
	RightParen,
    Equal,
	Dot,
	Colon,
	Pound,
	Comma,
    Number(u16),
	Identifier(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
	let mut output = Vec::new();
	let mut chars = input.chars().peekable();
	while let Some(c) = chars.next() {
		let token = match c {
			' ' | '\t' => continue,
			'+' => Token::Plus,
			'-' => Token::Minus,
			'*' => Token::Asterisk,
			'/' => Token::Slash,
			'(' => Token::LeftParen,
			')' => Token::RightParen,
			'=' => Token::Equal,
			'.' => Token::Dot,
			':' => Token::Colon,
			'#' => Token::Pound,
			',' => Token::Comma,
			'0'..='9' => {
				let mut s = String::from(c);
				loop {
					match chars.peek() {
						Some(c) if c.is_digit(10) => {
							s.push(*c);
							chars.next();
						}
						_ => break
					}
				}
				match s.parse::<u16>() {
					Ok(n) => Token::Number(n),
					Err(_) => return Err(Error::new(ErrorKind::InvalidInput, format!("Could not parse number {s}")))
				}
			},
			'$' => {
				let mut s = String::new();
				loop {
					match chars.peek() {
						Some(c) if c.is_digit(16) => {
							s.push(*c);
							chars.next();
						}
						_ => break
					}
				}
				match u16::from_str_radix(s.as_str(), 16) {
					Ok(n) => Token::Number(n),
					Err(_) => return Err(Error::new(ErrorKind::InvalidInput, format!("Could not parse number ${s}")))
				}
			},
			'a'..='z' | 'A'..='Z' | '_' | '@' => {
				let mut s = String::from(c);
				loop {
					match chars.peek() {
						Some(c) if c.is_alphanumeric() || *c == '_' => {
							s.push(*c);
							chars.next();
						}
						_ => break
					}
				}
				Token::Identifier(s)
			},
			_ => return Err(Error::new(ErrorKind::InvalidInput, format!("Unexpected character {c}")))
		};

		output.push(token);
	}

	Ok(output)
}
