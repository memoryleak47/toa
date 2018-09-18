extern crate toalib;

use std::net::{TcpListener, SocketAddr, TcpStream};
use std::io::stdin;

use toalib::world::World;

const PORT: u32 = 4242;

struct Slot {
	stream: TcpStream,
	addr: SocketAddr, // TODO add team
}

fn main() {
	let bind_string = format!("127.0.0.1:{}", PORT);
    let listener = TcpListener::bind(&*bind_string).expect("Could not bind TcpListener");
	listener.set_nonblocking(true).expect("Could not set non-blocking");

	let mut slots = Vec::new();

	let handle_command = |c: &str| {
		match c {
			"go" => return true,
			_ => {}, // TODO add more commands
		};
		return false;
	};

	// lobby
	loop {
		// add new connections
		if let Ok((stream, addr)) = listener.accept() {
			let slot = Slot { stream, addr };
			slots.push(slot);
			println!("a new player joined");
		}

		// enter commands
		println!("enter command");

		let mut s = String::new();
		stdin().read_line(&mut s).unwrap(); // TODO make this non-blocking
		if handle_command((&*s).trim()) { break; }
	}

	// game
	let mut w = World::gen();
	loop {
		println!("running!");
		// run!
	}
}
