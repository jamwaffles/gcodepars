use std::slice;

use parser::commands::*;
use parser::tokeniser::{ Token, Vector9 };

#[derive(Debug)]
pub struct ProgramTree {
	command: Option<Command>,
	children: Option<Box<ProgramTree>>,
	moves: Vec<Vector9>,
}

fn tree_from_tokens <'a>(mut tokens: &mut slice::Iter<'a, Token>) -> ProgramTree {
	let mut context = ProgramTree {
		command: None,
		children: None,
		moves: Vec::new(),
	};

	while let Some(token) = tokens.next() {
		match token {
			&Token::Command(ref c) => {
				context.command = Some(c.clone());
				context.children = Some(Box::new(tree_from_tokens(&mut tokens)));
			},
			&Token::Move(ref m) => context.moves.push(m.clone()),
			_ => ()
		}
	}

	context
}

pub fn from_tokens(tokens: &Vec<Token>) -> ProgramTree {
	tree_from_tokens(&mut tokens.iter())
}
