use std::io::stdin;

pub fn read_stdin() -> String {
	let mut s = String::new();
	stdin().read_line(&mut s).unwrap(); // TODO make this non-blocking
	s
}
