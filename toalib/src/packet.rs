use serde_json;

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


impl ServerToClientPacket {
	pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
		serde_json::to_string(self)
	}

	pub fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
		serde_json::from_str(&s)
	}
}

impl ClientToServerPacket {
	pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
		serde_json::to_string(self)
	}

	pub fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
		serde_json::from_str(&s)
	}
}
