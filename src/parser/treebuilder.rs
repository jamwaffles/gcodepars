use std::slice;
use std::iter;
use std::fmt;

use parser::commands::*;
use parser::tokeniser::{ Token, Vector9 };

// #[derive(Debug)]
// enum Action {
// 	Command(Command),
// 	Move(Move),
// }

#[derive(Debug)]
pub struct ProgramTree {
	command: Option<Command>,
	// commands: Vec<Command>,
	// children: Option<Box<ProgramTree>>,
	moves: Vec<Vector9>,

	children: Vec<Box<ProgramTree>>,
	level: u32,
}

fn collect_moves <'a>(mut tokens: &mut iter::Peekable<slice::Iter<'a, Token>>) -> Vec<Vector9> {
	let mut moves = Vec::new();

	while let Some(next) = tokens.next() {
		if let &Token::Move(ref m) = next {
			moves.push(m.clone());
		}

		if let Some(&&Token::Command(_)) = tokens.peek() {
			println!("Collected {} moves", moves.len());

			return moves
		}
	}

	moves
}

fn tree_from_tokens <'a>(mut tokens: &mut iter::Peekable<slice::Iter<'a, Token>>, level: u32) -> ProgramTree {

}

pub fn from_tokens(tokens: &Vec<Token>) -> Vec<ProgramTree> {
	// tree_from_tokens(&mut tokens.iter(), &ProgramTree { command: None, children: None, moves: Vec::new() }, 0)
	tree_from_tokens(&mut tokens.iter().peekable(), 0)
}
