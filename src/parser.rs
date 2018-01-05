use nom::*;
use std::str;
use std::str::FromStr;

// Duh
// named!(parse_float);
// named!(parse_int);

// // A letter `N` followed by a real value
// named!(parse_linenumber);

// // Any letter that is not `N` followed by a real value
// named!(parse_word);

// named!(parse_comment, delimited!("(", comment, ")"));
named!(parse_comment<&[u8], String>, do_parse!(
	tag!("(") >>
	text: take_until!(")") >>
	tag!(")") >>
	(FromStr::from_str(str::from_utf8(text).unwrap()).unwrap())
));

// named!(parse_numbered_variable, parse_int);
// named!(parse_local_variable, delimited("<", text, ">") );
// named!(parse_global_variable, delimited("<_", text, ">"));

// // Global vars must be parsed first because of the leading underscore
// named!(parse_variable, "#", then one_of!(parse_numbered_variable | parse_global_variable | parse_local_variable))

named!(parse<&[u8], Vec<String>>, many0!(
	ws!(
		alt!(
			parse_comment
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
