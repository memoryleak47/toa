extern crate toalib;

use toalib::world::World;
use toalib::team::Team;
use toalib::packet::{ServerToClientPacket, ClientToServerPacket};

mod pool;
mod net;
mod term;

use self::pool::UserPool;
use self::net::create_listener;
use self::term::{Term, TermCommand};

fn main() {
	let listener = create_listener().expect("Could not create listener");

	let mut user_pool = UserPool::new();
	let mut term = Term::new();

	// lobby
	loop {
		// add new connections
		if let Ok((stream, addr)) = listener.accept() {
			user_pool.add(Team::Red, stream, addr);
			println!("a new player joined");
		}

		match term.fetch_command() {
			Some(TermCommand::Go) => break,
			None => continue,
		}
	}

	let mut w = World::gen(user_pool.get_player_pool().clone());

	user_pool.broadcast(|id| {
		ServerToClientPacket::Init {
			world: w.clone(),
			your_id: id,
		}
	});

	// game
	loop {
		for (id, packet) in user_pool.receive_packets().into_iter() {
			let command = match packet {
				ClientToServerPacket::Command(c) => c,
			};

			if w.checked_exec(id, &command) {
				user_pool.broadcast(|_| {
					ServerToClientPacket::Command {
						command: command.clone(),
						author_id: id,
					}
				});
			}
		}
	}
}
