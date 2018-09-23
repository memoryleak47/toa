use std::collections::HashMap;

use toalib::team::{Team, PlayerPool, PlayerID};
use toalib::packet::{ServerToClientPacket, ClientToServerPacket};
use toalib::net::Stream;

pub struct UserPool {
	users: HashMap<PlayerID, Stream>,
	player_pool: PlayerPool,
}

impl UserPool {
	pub fn new() -> UserPool {
		UserPool {
			users: HashMap::new(),
			player_pool: PlayerPool::new(),
		}
	}

	pub fn add(&mut self, team: Team, stream: Stream) {
		let player_id = self.player_pool.add(team);
		self.users.insert(player_id, stream);
	}

	pub fn send(&mut self, id: PlayerID, p: ServerToClientPacket) {
		self.users.get_mut(&id)
			.unwrap()
			.send(p)
	}

	pub fn broadcast<F>(&mut self, f: F) where F: Fn(PlayerID) -> ServerToClientPacket {
		let v: Vec<PlayerID> = self.users.keys()
				.cloned()
				.collect();

		for &x in v.iter() {
			self.send(x, f(x));
		}
	}

	pub fn receive_packets(&mut self) -> Vec<(PlayerID, ClientToServerPacket)> {
		let mut v = Vec::new();
		for (&id, stream) in self.users.iter_mut() {	
			match stream.receive_nonblocking::<ClientToServerPacket>() {
				Some(x) => v.push((id, x)),
				None => {},
			}
		}
		v
	}

	pub fn get_player_pool(&self) -> &PlayerPool {
		&self.player_pool
	}
}
