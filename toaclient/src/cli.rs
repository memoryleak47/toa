use std::env;

pub fn get_ip() -> String {
	let args: Vec<String> = env::args().collect();

	match &args[..] {
		[_, x] => x.clone(),
		_ => panic!("invalid number of CLI parameters"),
	}
}
