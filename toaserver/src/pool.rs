use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};

use toalib::team::PlayerID;

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
}
