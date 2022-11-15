use std::error::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Lines, BufReader};

pub struct Assembler {
	pub labels: HashMap<String, u32>,
	pub variables: HashMap<String, u16>
}

impl Assembler {
	pub fn new() -> Assembler {
		Assembler {
			labels: HashMap::new(),
			variables: HashMap::new()
		}
	}

	pub fn assemble(&mut self, lines: Lines<BufReader<File>>) -> Result<(), Box<dyn Error>> {
		let mut code_address = 0;
		for line in lines {
			let line = line?;
			println!("{code_address:0>4X}: {line}");
	
			// Strip out comments.
			let mut line = match line.find(";") {
				Some(pos) => &line[..pos],
				None => line.as_str()
			};
	
			// Strip off X or Y index.
			line = match line.find(",") {
				Some(pos) => &line[..pos],
				None => line
			};
	
			line = line.trim();
	
			// We have either an assignment, a directive, a label, or an instruction.

			// Is it an assignment?
			if let Some(assign_index) = line.find("=") {
				let name = &line[..assign_index];
				let expression = &line[(assign_index+1)..];
				let value = Self::evaluate_expression(expression);
				self.variables.insert(name.to_string(), value);
				
				continue;
			}

			// Is it a directive?
			if line.starts_with(".") {
				// not implemented yet
				continue;
			}

			// Is it a label?
			if let Some(colon) = line.find(":") {
				let label = &line[..colon];
				self.labels.insert(label.to_string(), code_address);
				
				// A label may be followed by an instruction, so maybe keep going.
				line = &line[colon+1..];
			}

			// We better have an instruction now.
			if line.len() < 3 {
				continue;
			}
	
			// All instructions are one byte for the instruction, possibly more for a parameter.
			code_address = code_address + 1;
			let _instruction = &line[..3];
			let expression = &line[3..];
			let parameter = Self::evaluate_expression(expression);
			code_address = code_address + if parameter > 0xFF { 2 } else { 1 };
		}	

		Ok(())
	}

	fn evaluate_expression(_expression: &str) -> u16 {
		0
	}	
}
