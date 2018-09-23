use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};

use toalib::team::{Team, PlayerPool, PlayerID};
use toalib::packet::{ServerToClientPacket, ClientToServerPacket};

use crate::net::{send_packet, try_receiving_packet};

struct Entry {
	stream: TcpStream,
	addr: SocketAddr,
}

pub struct UserPool {
	users: HashMap<PlayerID, Entry>,
	player_pool: PlayerPool,
}

impl UserPool {
	pub fn new() -> UserPool {
		UserPool {
			users: HashMap::new(),
			player_pool: PlayerPool::new(),
		}
	}

	pub fn add(&mut self, team: Team, stream: TcpStream, addr: SocketAddr) {
		stream.set_nonblocking(true).unwrap();

		let player_id = self.player_pool.add(team);
		let entry = Entry { stream, addr };
		self.users.insert(player_id, entry);
	}

	pub fn send(&mut self, id: PlayerID, p: ServerToClientPacket) {
		send_packet(p, &mut self.users.get_mut(&id).unwrap().stream);
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
		for (&id, entry) in self.users.iter_mut() {	
			if let Some(x) = try_receiving_packet(&mut entry.stream) {
				v.push((id, x.unwrap()));
			}
		}
		v
	}

	pub fn get_player_pool(&self) -> &PlayerPool {
		&self.player_pool
	}
}
