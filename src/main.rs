use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = env::args();
	if args.len() != 2 {
		return Err("Usage: assembler <input file>".into());
	}

	let filename = args.nth(1).unwrap();
	let file = File::open(filename)?;
	let reader = BufReader::new(file);

	let mut code_address = 0;
	let mut labels = HashMap::new();
	for line in reader.lines() {
		let line = line?;
		// Strip out comments.
		let line = match line.find(";") {
			Some(pos) => &line[..pos],
			None => line.as_str()
		};

		// Split on whitespace.
		let tokens = &mut line.split_whitespace();
		let next = tokens.next();
		if next.is_none() {
			continue;
		}

		// Do we have a label?
		let mut instruction = next.unwrap();
		if instruction.ends_with(":") {
			labels.insert(instruction.to_string(), code_address);
			let next = tokens.next();
			if next.is_none() {
				continue;
			}
			instruction = next.unwrap();
		}

		if instruction == "LDA" {
			code_address = code_address + 1;
		}
	}

	for label in labels.keys() {
		println!("{} {}", label, labels.get(label).unwrap());
	}

	Ok(())
}
