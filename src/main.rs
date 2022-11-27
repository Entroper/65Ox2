use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

mod assembler;

fn main() -> Result<(), Box<dyn Error>> {
	assemble()
}

fn assemble() -> Result<(), Box<dyn Error>> {
	let mut args = env::args();
	if args.len() != 2 {
		return Err("Usage: assembler <input file>".into());
	}

	let filename = args.nth(1).unwrap();
	let file = File::open(filename)?;
	let reader = BufReader::new(file);

	let mut assembler = assembler::Assembler::new();
	assembler.assemble(reader.lines())?;

	for variable in assembler.variables.keys() {
		let value = assembler.variables.get(variable).unwrap();
		println!("{variable} ${value:0>4X}");
	}
	for label in assembler.labels.keys() {
		let value = assembler.labels.get(label).unwrap();
		println!("{label} ${value:0>4X}");
	}

	Ok(())
}
