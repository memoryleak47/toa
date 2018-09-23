extern crate toalib;

use toalib::world::World;
use toalib::team::{Team, PlayerPool};
use toalib::packet::{ServerToClientPacket, ClientToServerPacket};

mod pool;
mod net;
mod term;

use self::pool::NetPool;
use self::net::create_listener;
use self::term::{Term, TermCommand};

fn main() {
	let listener = create_listener().expect("Could not create listener");

	let mut net_pool = NetPool::new();
	let mut player_pool = PlayerPool::new();
	let mut term = Term::new();

	// lobby
	loop {
		// add new connections
		if let Ok((stream, addr)) = listener.accept() {
			let id = player_pool.add(Team::Red);
			net_pool.add(id, stream, addr);
			println!("a new player joined");
		}

		match term.fetch_command() {
			Some(TermCommand::Go) => break,
			None => continue,
		}
	}

	let mut w = World::gen(player_pool);

	net_pool.broadcast(|id| {
		ServerToClientPacket::Init {
			world: w.clone(),
			your_id: id,
		}
	});

	// game
	loop {
		for (id, packet) in net_pool.receive_packets().into_iter() {
			let command = match packet {
				ClientToServerPacket::Command(c) => c,
			};

			if w.checked_exec(id, &command) {
				net_pool.broadcast(|_| {
					ServerToClientPacket::Command {
						command: command.clone(),
						author_id: id,
					}
				});
			}
		}
	}
}
