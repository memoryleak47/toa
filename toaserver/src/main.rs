extern crate toalib;

use toalib::world::World;
use toalib::team::Team;
use toalib::packet::{ServerToClientPacket, ClientToServerPacket};
use toalib::net::Listener;

mod pool;
mod term;

use self::pool::UserPool;
use self::term::{Term, TermCommand};

fn main() {
	let mut listener = Listener::bind("0.0.0.0:4242");

	let mut user_pool = UserPool::new();
	let mut term = Term::new();

	// lobby
	loop {
		// add new connections
		if let Some(stream) = listener.accept_nonblocking() {
			user_pool.add(Team(0), stream);
			println!("a new player joined");
		}

		match term.fetch_command() {
			Some(TermCommand::Go) => break,
			Some(TermCommand::Status) => {
				println!("Status:\n");
				for player_id in user_pool.get_player_pool().get_player_ids() {
					let team = user_pool.get_player_pool().get_team_of(player_id);
					println!("{}: {:?}", player_id, team);
				}
			},
			Some(TermCommand::ChangeTeam { player_id, team }) => {
				user_pool.get_player_pool_mut().change_team(player_id, team);
			},
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
			} else {
				user_pool.send(id, ServerToClientPacket::DeclineCommand);
			}
		}
		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
