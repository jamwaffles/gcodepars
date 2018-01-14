use std::slice;
use std::iter;

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
	// children: Option<Box<ProgramTree>>,
	moves: Vec<Vector9>,

	children: Vec<Box<ProgramTree>>,
}

fn tree_from_tokens <'a>(mut tokens: &mut iter::Peekable<slice::Iter<'a, Token>>, level: u32) -> ProgramTree {
	let mut context = ProgramTree {
		command: None,
		children: Vec::new(),
		moves: Vec::new(),
	};
	// let mut last_command = Option<Command>;

	let prefix = (0..level).map(|_| "-").collect::<String>();

	// println!("{}Level: {}", prefix, level);

	while let Some(next) = tokens.peek() {
		// println!("{}Token {:?}", prefix, token);

		let token = tokens.next().unwrap();

		match token {
			&Token::Command(ref c) => {
				println!("{}COMMAND {:?}, current is {:?}", prefix, c, context.command);

				// match context.command {
				// 	Some(_) => {
				// 		return context;
				// 	},
				// 	None => {
				// 		context.command = Some(c.clone());

				// 		context.children = Some(Box::new(tree_from_tokens(&mut tokens, &context, level + 1)));
				// 	}
				// }
				if context.command.is_some() {
					// context.children.push(Box::new(tree_from_tokens(&mut tokens, level + 1)));
					println!("{}Context already has command {:?}", prefix, context.command);
					return context
				} else {
					context.command = Some(c.clone());

				}
			},
			&Token::Move(ref m) => {
				println!("{}MOVE {:?}", prefix, m);
				context.moves.push(m.clone());

				// let foo = tokens.peek();

				// if let Some(next) = foo {
				// 	match (next, &context.command) {
				// 		// Token::Command(_) => if context.command.is_some
				// 		(&&Token::Command(_), &Some(_)) => context.children.push(Box::new(tree_from_tokens(&mut tokens, level + 1))),
				// 		_ => ()
				// 	}
				// }
			},
			_ => ()
		}
	}

	context
}

pub fn from_tokens(tokens: &Vec<Token>) -> ProgramTree {
	// tree_from_tokens(&mut tokens.iter(), &ProgramTree { command: None, children: None, moves: Vec::new() }, 0)
	tree_from_tokens(&mut tokens.iter().peekable(), 0)
}
