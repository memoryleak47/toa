use std::env;

pub fn get_ip() -> String {
	let args: Vec<_> = env::args()
		.collect();

	let str_args: Vec<_> = args.iter()
			.map(|x| &*x)
			.collect();

	match &str_args[..] {
		[_, x] => x.to_string(),
		_ => panic!("invalid number of CLI parameters"),
	}
}
