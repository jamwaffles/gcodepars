use nom::*;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
enum Value {
	Int(i32),
	Float(f32),
}

#[derive(Debug)]
enum Entity {
	Comment(String),
	Word((char, Value))
}

named!(parse_float<Value>, do_parse!(
	sign: opt!(tag!("-")) >>
	bef: opt!(digit) >>
	tag!(".") >>
	aft: opt!(digit) >>
	({
		let a = match sign {
			Some(sign) => str::from_utf8(sign).unwrap(),
			None => ""
		};

		let b = match bef {
			Some(bef) => str::from_utf8(bef).unwrap(),
			None => "0"
		};

		let c = match aft {
			Some(aft) => str::from_utf8(aft).unwrap(),
			None => "0"
		};

		let parsed = format!("{}{}.{}", a, b, c).parse::<f32>().unwrap();

		Value::Float(parsed)
	})
));

named!(parse_int<Value>, do_parse!(
	sign: opt!(tag!("-")) >>
	num: digit >>
	({
		let a = match sign {
			Some(sign) => str::from_utf8(sign).unwrap(),
			None => ""
		};

		let b = str::from_utf8(num).unwrap();

		let parsed = format!("{}{}", a, b).parse::<i32>().unwrap();

		Value::Int(parsed)
	})
));

// Any letter that is not `N` followed by a real value
named!(parse_word<&[u8], Entity>, do_parse!(
	letter: map!(one_of!("ABCDEFGHIJKLMNOPRSTUVWXYZabcdefghijklmnoprstuvwxyz"), |s| s.to_ascii_uppercase()) >>
	value: alt!(parse_float | parse_int) >>
	({
		Entity::Word((letter, value))
	})
));

named!(parse_comment<&[u8], Entity>, do_parse!(
	tag!("(") >>
	text: map_res!(
		map_res!(take_until!(")"), str::from_utf8),
		FromStr::from_str
	) >>
	tag!(")") >>
	(Entity::Comment(text))
));

// named!(parse_numbered_variable, parse_int);
// named!(parse_local_variable, delimited("<", text, ">") );
// named!(parse_global_variable, delimited("<_", text, ">"));

// // Global vars must be parsed first because of the leading underscore
// named!(parse_variable, "#", then one_of!(parse_numbered_variable | parse_global_variable | parse_local_variable))

named!(parse<&[u8], Vec<Entity>>, ws!(
	many1!(
 		alt!(
			parse_comment |
			parse_word
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
