use std::env;

pub fn get_arg() -> String {
	let args: Vec<String> = env::args().collect();

	match &args[..] {
		[_, x] => x.clone(),
		_ => panic!("invalid number of CLI parameters"),
	}
}
