use serde::{Serialize, de::DeserializeOwned};

use crate::world::World;
use crate::command::Command;
use crate::team::PlayerID;

#[derive(Serialize, Deserialize)]
pub enum ServerToClientPacket {
	Init {
		world: World,
		your_id: PlayerID
	},
	Command {
		command: Command,
		author_id: PlayerID,
	}
}

#[derive(Serialize, Deserialize)]
pub enum ClientToServerPacket {
	Command(Command),
}

pub trait Packet: Serialize + DeserializeOwned {}

impl Packet for ServerToClientPacket {}
impl Packet for ClientToServerPacket {}
