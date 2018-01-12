#[macro_use]
extern crate nom;
extern crate nalgebra;

use std::fs::File;
use std::io::prelude::*;

mod parser;
mod commands;

fn main() {
	let filename = "./data/simpleaf.ngc";

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();

	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	parser::parse_gcode(&contents.as_bytes());
}
