mod commands;
mod treebuilder;
mod tokeniser;

use self::tokeniser::from_bytes;
use self::treebuilder::{ from_tokens, ProgramTree };

#[derive(Debug)]
pub struct Program {
	pub tree: ProgramTree,
}

pub fn parse_bytes(input: &[u8]) -> Program {
	let tokens = from_bytes(&input);
	let tree = from_tokens(&tokens).first().unwrap();

	Program {
		tree: *tree,
	}
}
