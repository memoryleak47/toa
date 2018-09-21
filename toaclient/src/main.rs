extern crate toalib;
extern crate sfml;

mod net;

use std::net::TcpStream;
use std::env;
use std::io::Read;

use toalib::packet::ServerToClientPacket;

use self::net::try_receiving_packet;

fn main() {
	let args: Vec<_> = env::args()
			.collect();
	let str_args: Vec<_> = args.iter()
			.map(|x| &*x)
			.collect();
	let ip = match &str_args[..] {
		[_, x] => x,
		_ => panic!("invalid number of CLI parameters"),
	};

	let mut stream = TcpStream::connect(&*ip).unwrap();

	let mut init_string = String::new();
	stream.read_to_string(&mut init_string).unwrap();

	let (mut world, my_id) = match ServerToClientPacket::from_str(&*init_string).unwrap() {
		ServerToClientPacket::Init { world, your_id } => (world, your_id),
		_ => panic!("got command packet while still in lobby!"),
	};

	loop {
		if let Some(x) = try_receiving_packet(&mut stream) {
			let (author_id, command) = match x.unwrap() {
				ServerToClientPacket::Command { author_id, command, } => (author_id, command),
				_ => panic!("got init packet while already running!"),
			};

			assert!(world.checked_exec(author_id, &command));
		}

		// TODO tick

		// TODO maybe send packet
	}

}
