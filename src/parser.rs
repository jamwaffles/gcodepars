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
	Word((char, u32)),
}

named!(float<f32>, do_parse!(
	sign: opt!(one_of!("+-")) >>
	num: recognize!(
		alt!(
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

// named!(parse_word<&[u8], (char, Value)>, do_parse!(
// 	letter: map!(one_of!("ABCDEFGHIJKLMNOPRSTUVWXYZabcdefghijklmnoprstuvwxyz"), |s| s.to_ascii_uppercase()) >>
// 	value: alt!(float | int) >>
// 	((letter, value))
// ));

named_args!(parse_word_float (letter: char) <f32>, do_parse!(
	tag_no_case!(letter.to_string().as_bytes()) >>
	number: float >>
	({
		println!("asdsgkjbajkshdgb {}", number);

		123.0
	})
));

// named_with_args!(parse_word_int (letter: char), );

named!(parse_comment<&[u8], String>, do_parse!(
	tag!("(") >>
	text: map_res!(
		map_res!(take_until!(")"), str::from_utf8),
		FromStr::from_str
	) >>
	tag!(")") >>
	(text)
));

// named!(parse_numbered_variable, parse_int);
// named!(parse_local_variable, delimited("<", text, ">") );
// named!(parse_global_variable, delimited("<_", text, ">"));

// // Global vars must be parsed first because of the leading underscore
// named!(parse_variable, "#", then one_of!(parse_numbered_variable | parse_global_variable | parse_local_variable))


// FIXME: somethingsomething https://stackoverflow.com/questions/28931515/how-do-i-implement-fromstr-with-a-concrete-lifetime this maybe? idk

named!(parse<&[u8], Vec<Entity>>, ws!(
	many1!(
		alt!(
			parse_comment => { |c| Entity::Comment(c) } |
			parse_word_float('g') => { |g| Entity::Word(g) }
		)
	)
));

pub fn parse_gcode(input: &[u8]) {
	println!("{}", str::from_utf8(input).unwrap());

	// let parsed = parse(input);

	// println!("{:?}", parsed);
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
		let comment = "(Good 👍 stuff 👌)".as_bytes();
		let parsed = "Good 👍 stuff 👌".to_string();

		assert_eq!(parse_comment(comment), Ok(("".as_bytes(), parsed)));
	}

	#[test]
	fn it_parses_floats() {
		assert_eq!(float("123.456".as_bytes()), Ok(("".as_bytes(), 123.456)));
		assert_eq!(float("0.123".as_bytes()), Ok(("".as_bytes(), 0.123)));
		assert_eq!(float("123.0".as_bytes()), Ok(("".as_bytes(), 123.0)));
		// FIXME
		// assert_eq!(float("123.".as_bytes()), Ok(("".as_bytes(), 123.0)));
		assert_eq!(float(".123".as_bytes()), Ok(("".as_bytes(), 0.123)));
		assert_eq!(float("+.123".as_bytes()), Ok(("".as_bytes(), 0.123)));
		assert_eq!(float("+1.123".as_bytes()), Ok(("".as_bytes(), 1.123)));
		assert_eq!(float("-1.123".as_bytes()), Ok(("".as_bytes(), -1.123)));
	}
}
