use nom::*;
use std::str;
use std::slice;
// use nalgebra::{ VectorN, U9 };

use ::commands::*;

// type Vector9 = VectorN<f32, U9>;

#[derive(Debug, PartialEq)]
enum MeasurementUnits {
	Imperial,	// G20
	Metric,		// G21
}

// Order taken from here: http://linuxcnc.org/docs/html/gcode/overview.html#sub:numbered-parameters
#[derive(Debug, PartialEq, Clone)]
enum Axis {
	X(f32), Y(f32), Z(f32),
	A(f32), B(f32), C(f32),
	U(f32), V(f32), W(f32),
}

// #[derive(Debug, PartialEq)]
// enum GCode {
// 	Rapid,			// G0
// 	Move,			// G1
// 	CWArc,			// G2
// 	CCWArc,			// G3
// }

// #[derive(Debug, PartialEq)]
// enum MCode {
// 	EndProgram,		// M2
// 	SpindleCW,		// M3
// 	SpindleCCW,		// M4
// 	SpindleStop,	// M5
// 	CoolantMist,	// M7
// 	CoolantFloor,	// M8
// 	CoolantOff,		// M9
// }

// type Vector9 = [ Option<f32>; 9 ];
#[derive(Debug, PartialEq, Clone)]
struct Vector9 {
	x: Option<f32>,
	y: Option<f32>,
	z: Option<f32>,
	a: Option<f32>,
	b: Option<f32>,
	c: Option<f32>,
	u: Option<f32>,
	v: Option<f32>,
	w: Option<f32>,
	r: Option<f32>,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
	Comment(String),
	Word(String),
	Axis(Axis),
	Radius(f32),
	Feed(f32),
	Tool(i32),
	SpindleSpeed(i32),
	Unknown(String),

	Command(Command),
	Move(Vector9),
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
	tokens: Vec<Token>,
	linenumber: u32,
}

named!(float<f32>, do_parse!(
	sign: opt!(one_of!("+-")) >>
	num: recognize!(
		alt_complete!(
			delimited!(digit, tag!("."), opt!(digit)) |
			delimited!(opt!(digit), tag!("."), digit)
		)
	) >>
	({
		let parsed = str::from_utf8(num).unwrap().parse::<f32>().unwrap();

		match sign {
			Some('-') => parsed * -1.0,
			_ => parsed,
		}
	})
));

named!(int<i32>, do_parse!(
	sign: opt!(tag!("-")) >>
	num: digit >>
	({
		let a = match sign {
			Some(sign) => str::from_utf8(sign).unwrap(),
			None => ""
		};

		let b = str::from_utf8(num).unwrap();

		format!("{}{}", a, b).parse::<i32>().unwrap()
	})
));

named!(number<f32>, flat_map!(
	alt_complete!(recognize!(float) | recognize!(int)),
	parse_to!(f32)
));

named!(word<&[u8], Token>, map!(
	flat_map!(
		recognize!(preceded!(one_of!("DFGHIJKLMNPQRSTdfghijklmnpqrst"), recognize!(number))),
		parse_to!(String)
	),
	|w| Token::Word(w)
));

named!(gmcode<String>, map!(
	flat_map!(
		complete!(
			recognize!(
				preceded!(one_of!("GMgm"), number)
			)
		),
		parse_to!(String)
	),
	|w| w.to_uppercase()
));

named!(motioncommand<Command>, alt_complete!(
	map!(tag_no_case!("G0"), |_| Command::Motion(Motion::Rapid))
	| map!(tag_no_case!("G1"), |_| Command::Motion(Motion::Linear))
));

named!(unitscommand<Command>, alt_complete!(
	map!(tag_no_case!("G20"), |_| Command::Units(Units::Imperial))
	| map!(tag_no_case!("G21"), |_| Command::Units(Units::Metric))
));

named!(stopcommand<Command>, map!(
	gmcode,
	|word| match word.as_str() {
		"M2" => Command::Stop(Stop::End),
		_ => Command::Unknown(word),
	}
));

named!(command<Token>, map!(
	alt!(
		motioncommand
		| unitscommand
		| stopcommand
	),
	|g| Token::Command(g)
));

named!(axis<&[u8], Token>, do_parse!(
	axis_letter: map!(one_of!("ABCUVWXYZabcuvwxyz"), |s| s.to_ascii_uppercase()) >>
	value: number >>
	({
		let axis = match axis_letter {
			'A' => Axis::A(value),
			'B' => Axis::B(value),
			'C' => Axis::C(value),
			'U' => Axis::U(value),
			'V' => Axis::V(value),
			'W' => Axis::W(value),
			'X' => Axis::X(value),
			'Y' => Axis::Y(value),
			'Z' => Axis::Z(value),
			_ => panic!("Axis letter {} not recognised", axis_letter),
		};

		Token::Axis(axis)
	})
));

named!(parse_move<Token>, do_parse!(
	x: opt!(complete!(preceded!(tag_no_case!("X"), number))) >>
	y: opt!(complete!(preceded!(tag_no_case!("Y"), number))) >>
	z: opt!(complete!(preceded!(tag_no_case!("Z"), number))) >>
	a: opt!(complete!(preceded!(tag_no_case!("A"), number))) >>
	b: opt!(complete!(preceded!(tag_no_case!("B"), number))) >>
	c: opt!(complete!(preceded!(tag_no_case!("C"), number))) >>
	u: opt!(complete!(preceded!(tag_no_case!("U"), number))) >>
	v: opt!(complete!(preceded!(tag_no_case!("V"), number))) >>
	w: opt!(complete!(preceded!(tag_no_case!("W"), number))) >>
	r: opt!(complete!(preceded!(tag_no_case!("R"), number))) >>
	(Token::Move(Vector9 { x, y, z, a, b, c, u, v, w, r }))
));

named!(comment<Token>, map!(
	flat_map!(delimited!(tag!("("), take_until!(")"), tag!(")")), parse_to!(String)),
	Token::Comment
));
// named!(rapid<Token>, preceded!(tag_no_case!("G0"), map!(axes, Token::Rapid)));
// named!(linear_move<Token>, preceded!(tag_no_case!("G1"), map!(axes, Token::Move)));
// named!(measurement_units<Token>, alt!(
// 	map!(tag_no_case!("G20"), |_| Token::MeasurementUnits(MeasurementUnit::Imperial)) |
// 	map!(tag_no_case!("G21"), |_| Token::MeasurementUnits(MeasurementUnit::Metric))
// ));
named!(feed<Token>, preceded!(tag_no_case!("F"), map!(number, Token::Feed)));
named!(spindlespeed<Token>, preceded!(tag_no_case!("S"), map!(int, Token::SpindleSpeed)));
named!(tool<Token>, preceded!(tag_no_case!("T"), map!(int, Token::Tool)));
named!(radius<Token>, preceded!(tag_no_case!("R"), map!(number, Token::Radius)));
// named!(program_end<Token>, map!(tag_no_case!("M2"), |_| Token::ProgramEnd));

named!(unknown<Token>, map!(
	flat_map!(
		recognize!(preceded!(alpha, number)),
		parse_to!(String)
	),
	|t| Token::Unknown(t)
));

// named!(parse_numbered_variable, parse_int);
// named!(parse_local_variable, delimited("<", text, ">") );
// named!(parse_global_variable, delimited("<_", text, ">"));

// // Global vars must be parsed first because of the leading underscore
// named!(parse_variable, "#", then one_of!(parse_numbered_variable | parse_global_variable | parse_local_variable))

named!(line<Vec<Token>>, flat_map!(
	recognize!(take_until_and_consume!("\n")),
	many0!(
		ws!(alt_complete!(
			comment
			| parse_move
			| command
			// | feed
			// | tool
			// | radius
			// | spindlespeed
			// | word
			// | axis
			| unknown
		))
	)
));

named!(parse<Vec<Line>>, map!(
	many1!(line),
	|lines| lines.into_iter().enumerate().map(|(i, l)| Line {
		tokens: l,
		linenumber: i as u32
	}).collect::<Vec<Line>>()
));

#[derive(Debug, PartialEq)]
struct Context {
	units: Option<Units>,
	motion: Option<Motion>,
	moves: Vec<Vector9>,
	child_context: Option<Box<Context>>,
}

// If a line contains a command that would overwrite a set group command in the current context, create a new context (i.e. recurse)
// If a line contains no axes (just commands), add to the current context's commands
// If a line contains both an axis and anything else (command, feed, etc), create a new context (i.e. recurse)
// Otherwise, append the found axes to the current context's moves
fn create_tree_from_tokens <'a>(mut tokens: &mut slice::Iter<'a, Token>) -> Context {
	let mut context = Context {
		units: None,
		motion: None,
		moves: Vec::new(),
		child_context: None
	};

	while let Some(token) = tokens.peekable().next() {
		let mut should_recurse = false;
		// let mut current_move

		match token {
			&Token::Command(ref w) => match w {
				&Command::Motion(ref m) => if context.motion.is_none() { context.motion = Some(m.clone()) } else { should_recurse = true },
				&Command::Units(ref u) => if context.units.is_none() { context.units = Some(u.clone()) } else { should_recurse = true },

				_ => ()
			},

			&Token::Axis(ref a) => {

			},

			// &Token::Axis(ref a) => {
			// 	let next_axes: Vec<&Axis> = tokens.clone().take_while(|t| {
			// 		match **t {
			// 			Token::Axis(_) => true,
			// 			_ => false,
			// 		}
			// 	})
			// 	.filter_map(|t| match t {
			// 		&Token::Axis(ref a) => Some(a),
			// 		_ => None,
			// 	})
			// 	.collect();

			// 	println!("Current: {:?}, next: {:?}", a, next_axes);
			// },

			_ => ()
		}

		if should_recurse {
			context.child_context = Some(Box::new(create_tree_from_tokens(&mut tokens)));
		}
	}

	context
}

// pub fn construct_scope_tree() {
// 	// Turn "flat" parsed G-code into a scoped tree
// 	// This is so we can do stuff like "run from line, but set the tool and start the spindle" or whatever
// 	// It's a bit like an AST in that it holds context information
// }

pub fn parse_gcode(input: &[u8]) {
	println!("{}", str::from_utf8(input).unwrap());

	let (_, lines) = parse(input).unwrap();

	let all: Vec<Token> = lines.into_iter().flat_map(|l| l.tokens).collect();

	println!("\nAll {:?}", all);

	let tree = create_tree_from_tokens(&mut all.iter());

	println!("\n\nTree: {:?}", tree);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_parses_utf8_comments() {
		let comment_to_parse = "(Good 👍 stuff 👌)".as_bytes();
		let parsed = "Good 👍 stuff 👌".to_string();

		assert_eq!(comment(comment_to_parse), Ok(("".as_bytes(), Token::Comment(parsed))));
	}

	#[test]
	fn it_parses_floats() {
		assert_eq!(float("123.456".as_bytes()), Ok(("".as_bytes(), 123.456)));
		assert_eq!(float("0.123".as_bytes()), Ok(("".as_bytes(), 0.123)));
		assert_eq!(float("123.0".as_bytes()), Ok(("".as_bytes(), 123.0)));
		assert_eq!(float("1.5".as_bytes()), Ok(("".as_bytes(), 1.5f32)));
		// FIXME
		// assert_eq!(float("123.".as_bytes()), Ok(("".as_bytes(), 123.0)));
		assert_eq!(float(".123".as_bytes()), Ok(("".as_bytes(), 0.123)));
		assert_eq!(float("+.123".as_bytes()), Ok(("".as_bytes(), 0.123)));
		assert_eq!(float("+1.123".as_bytes()), Ok(("".as_bytes(), 1.123)));
		assert_eq!(float("-1.123".as_bytes()), Ok(("".as_bytes(), -1.123)));
	}

	#[test]
	fn it_parses_ints() {
		assert_eq!(int("123".as_bytes()), Ok(("".as_bytes(), 123i32)));
		assert_eq!(int("0".as_bytes()), Ok(("".as_bytes(), 0i32)));
		assert_eq!(int("400000".as_bytes()), Ok(("".as_bytes(), 400000i32)));
	}

	#[test]
	fn it_parses_words() {
		assert_eq!(word("g90".as_bytes()), Ok(("".as_bytes(), "g90".to_string())));
		assert_eq!(word("g0".as_bytes()), Ok(("".as_bytes(), "g0".to_string())));
		assert_eq!(word("G90.1".as_bytes()), Ok(("".as_bytes(), "G90.1".to_string())));
		assert_eq!(word("g90.1".as_bytes()), Ok(("".as_bytes(), "g90.1".to_string())));
	}

	#[test]
	fn it_parses_axes() {
		assert_eq!(axis("x1".as_bytes()), Ok(("".as_bytes(), Axis::X(1.0f32))));
		assert_eq!(axis("Y1.5".as_bytes()), Ok(("".as_bytes(), Axis::Y(1.5f32))));
		assert_eq!(axis("Z.5".as_bytes()), Ok(("".as_bytes(), Axis::Z(0.5f32))));
	}
}
