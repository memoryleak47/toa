use crate::world::World;
use crate::command::Command;

pub enum ServerToClientPacket {
	World(World),
}

pub enum ClientToServerPacket {
	Command(Command),
}
