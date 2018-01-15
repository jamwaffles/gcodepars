use nom::*;

use parser::commands;
use parser::commands::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector9 {
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
pub enum Token {
	Comment(String),
	Unknown(String),
	Command(Command),
	Move(Vector9),
}

named!(float<f32>, flat_map!(
	recognize!(
		preceded!(
			opt!(one_of!("+-")),
			alt_complete!(
				delimited!(digit, tag!("."), opt!(digit)) |
				delimited!(opt!(digit), tag!("."), digit)
			)
		)
	),
	parse_to!(f32)
));

named!(int<i32>, flat_map!(
	recognize!(
		preceded!(opt!(one_of!("+-")), digit)
	),
	parse_to!(i32)
));

named!(number<f32>, flat_map!(
	alt_complete!(recognize!(float) | recognize!(int)),
	parse_to!(f32)
));

named!(offsetcommand<Command>, alt_complete!(
	map!(tag_no_case!("G54"), |_| Command::Offset(commands::Offset::G54))
));

named!(motioncommand<Command>, alt_complete!(
	map!(tag_no_case!("G0"), |_| Command::Motion(Motion::Rapid))
	| map!(tag_no_case!("G1"), |_| Command::Motion(Motion::Linear))
));

named!(unitscommand<Command>, alt_complete!(
	map!(tag_no_case!("G20"), |_| Command::Units(Units::Imperial))
	| map!(tag_no_case!("G21"), |_| Command::Units(Units::Metric))
));

named!(stopcommand<Command>, alt_complete!(
	map!(tag_no_case!("M2"), |_| Command::Stop(Stop::End))
));

named!(command<Token>, map!(
	alt!(
		motioncommand
		| offsetcommand
		| unitscommand
		| stopcommand
	),
	|g| Token::Command(g)
));

named!(parse_move<Token>, ws!(do_parse!(
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
)));

named!(comment<Token>, map!(
	flat_map!(delimited!(tag!("("), take_until!(")"), tag!(")")), parse_to!(String)),
	Token::Comment
));

named!(unknown<Token>, map!(
	flat_map!(
		recognize!(preceded!(alpha, number)),
		parse_to!(String)
	),
	|t| Token::Unknown(t)
));

named!(token<Token>, alt_complete!(
	comment
	| command
	| parse_move
	| unknown
));

named!(tokens<Vec<Token>>, ws!(many1!(token)));

pub fn from_bytes(input: &[u8]) -> Vec<Token> {
	let (_, parsed) = tokens(input).unwrap();

	parsed
}
