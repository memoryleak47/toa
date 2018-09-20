use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};

use toalib::team::PlayerID;
use toalib::packet::ServerToClientPacket;

use crate::net::send_packet;

pub struct NetPool {
	players: HashMap<PlayerID, (TcpStream, SocketAddr)>,
}

impl NetPool {
	pub fn new() -> NetPool {
		NetPool {
			players: HashMap::new(),
		}
	}

	pub fn add(&mut self, player: PlayerID, stream: TcpStream, addr: SocketAddr) {
		self.players.insert(player, (stream, addr));
	}

	pub fn send(&mut self, id: PlayerID, p: ServerToClientPacket) {
		send_packet(p, &mut self.players.get_mut(&id).unwrap().0);
	}

	pub fn broadcast<F>(&mut self, f: F) where F: Fn(PlayerID) -> ServerToClientPacket {
		let v: Vec<PlayerID> = self.players.keys()
				.cloned()
				.collect();
		for &x in v.iter() {
			self.send(x, f(x));
		}
	}
}
