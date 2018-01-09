use nom::*;
use std::str;
use std::str::FromStr;
use nalgebra::{ VectorN, U9 };

type Vector9 = VectorN<f32, U9>;

#[derive(Debug, PartialEq)]
enum MeasurementUnit {
	Imperial,	// G20
	Metric,		// G21
}

// Order taken from here: http://linuxcnc.org/docs/html/gcode/overview.html#sub:numbered-parameters
#[derive(Debug, PartialEq)]
enum Axis {
	X(f32), Y(f32), Z(f32),
	A(f32), B(f32), C(f32),
	U(f32), V(f32), W(f32),
}

#[derive(Debug, PartialEq)]
enum GCode {
	Rapid,			// G0
	Move,			// G1
	CWArc,			// G2
	CCWArc,			// G3
}

#[derive(Debug, PartialEq)]
enum MCode {
	EndProgram,		// M2
	SpindleCW,		// M3
	SpindleCCW,		// M4
	SpindleStop,	// M5
	CoolantMist,	// M7
	CoolantFloor,	// M8
	CoolantOff,		// M9
}

#[derive(Debug, PartialEq)]
enum Token {
	Comment(String),
	MeasurementUnits(MeasurementUnit),
	// RadiusCompensation(f32),
	Feed(i32),
	G(GCode),
	// ToolLengthOffsetIndex(f32),
	// ArcXOffset(f32),
	// ArcYOffset(f32),
	// ArcZOffset(f32),
	// GenericParameter(f32),	// L word
	M(MCode),
	LineNumber(u32),
	// DwellTime(f32),		// P
	// FeedIncrement(f32),
	Radius(f32),
	SpindleSpeed(f32),
	Tool(u32),
	Rapid(Vector9),
	Move(Vector9),
	Unknown(String),
	ProgramEnd,
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

named!(word<&[u8], String>, flat_map!(
	recognize!(preceded!(one_of!("DFGHIJKLMNPQRSTdfghijklmnpqrst"), alt_complete!(recognize!(float) | recognize!(int)))),
	parse_to!(String)
));

fn match_gcode(number: &String) -> Option<GCode> {
	match number.as_str() {
		"0" => Some(GCode::Rapid),
		"1" => Some(GCode::Move),
		"2" => Some(GCode::CWArc),
		"3" => Some(GCode::CCWArc),
		_ => None
	}
}

named!(word_split<&[u8], Token>, do_parse!(
	letter: map!(one_of!("DFGHIJKLMNPQRSTdfghijklmnpqrst"), |s| s.to_ascii_uppercase()) >>
	number: flat_map!(alt_complete!(recognize!(float) | recognize!(int)), parse_to!(String)) >>
	({
		match letter {
			'G' => match match_gcode(&number) {
				Some(code) => Token::G(code),
				None => Token::Unknown(format!("G{}", number)),
			}
			// 'M' => match number {
			// 	_ => Token::Unknown(format!("M{}", number)),
			// },
			'R' => Token::Radius(number.parse::<f32>().unwrap()),
			_ => Token::Unknown(format!("{}{}", letter, number)),
		}
	})
));

named!(axis<&[u8], Axis>, do_parse!(
	axis_letter: map!(one_of!("ABCUVWXYZabcuvwxyz"), |s| s.to_ascii_uppercase()) >>
	value: alt_complete!(recognize!(float) | recognize!(int)) >>
	({
		let value_float = str::from_utf8(value).unwrap().parse::<f32>().unwrap();

		let axis = match axis_letter {
			'A' => Axis::A(value_float),
			'B' => Axis::B(value_float),
			'C' => Axis::C(value_float),
			'U' => Axis::U(value_float),
			'V' => Axis::V(value_float),
			'W' => Axis::W(value_float),
			'X' => Axis::X(value_float),
			'Y' => Axis::Y(value_float),
			'Z' => Axis::Z(value_float),
			_ => panic!("Axis letter {} not recognised", axis_letter),
		};

		axis
	})
));

named!(axes<&[u8], Vector9>, map!(
	many1!(axis),
	|axes| {
		let mut vector: [ f32; 9 ] = [ 0.0; 9 ];

		for axis in axes.iter() {
			match axis {
				// Order taken from here: http://linuxcnc.org/docs/html/gcode/overview.html#sub:numbered-parameters
				&Axis::X(dist) => vector[0] = dist,
				&Axis::Y(dist) => vector[1] = dist,
				&Axis::Z(dist) => vector[2] = dist,
				&Axis::A(dist) => vector[3] = dist,
				&Axis::B(dist) => vector[4] = dist,
				&Axis::C(dist) => vector[5] = dist,
				&Axis::U(dist) => vector[6] = dist,
				&Axis::V(dist) => vector[7] = dist,
				&Axis::W(dist) => vector[8] = dist,
			}
		}

		Vector9::from_column_slice(&vector)
	}
));

named!(comment<Token>, map!(
	flat_map!(delimited!(tag!("("), take_until!(")"), tag!(")")), parse_to!(String)),
	Token::Comment
));
named!(rapid<Token>, preceded!(tag_no_case!("G0"), map!(axes, Token::Rapid)));
named!(linear_move<Token>, preceded!(tag_no_case!("G1"), map!(axes, Token::Move)));
named!(measurement_units<Token>, alt!(
	map!(tag_no_case!("G20"), |_| Token::MeasurementUnits(MeasurementUnit::Imperial)) |
	map!(tag_no_case!("G21"), |_| Token::MeasurementUnits(MeasurementUnit::Metric))
));
named!(feedrate<Token>, preceded!(tag_no_case!("F"), map!(int, Token::Feed)));
named!(program_end<Token>, map!(tag_no_case!("M2"), |_| Token::ProgramEnd));

named!(unknown<Token>, map!(
	flat_map!(
		recognize!(preceded!(alpha, alt_complete!(recognize!(float) | recognize!(int)))),
		parse_to!(String)
	),
	|t| Token::Unknown(t)
));

// named!(parse_numbered_variable, parse_int);
// named!(parse_local_variable, delimited("<", text, ">") );
// named!(parse_global_variable, delimited("<_", text, ">"));

// // Global vars must be parsed first because of the leading underscore
// named!(parse_variable, "#", then one_of!(parse_numbered_variable | parse_global_variable | parse_local_variable))

named!(parse<&[u8], Vec<Token>>, ws!(
	many1!(
		alt!(
			comment
			| rapid
			| linear_move
			| measurement_units
			| feedrate
			| program_end
			| unknown
		)
	)
));

pub fn parse_gcode(input: &[u8]) {
	println!("{}", str::from_utf8(input).unwrap());

	let parsed = parse(input);

	println!("{:?}", parsed);
}

pub fn construct_scope_tree() {
	// Turn "flat" parsed G-code into a scoped tree
	// This is so we can do stuff like "run from line, but set the tool and start the spindle" or whatever
	// It's a bit like an AST in that it holds context information
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
