use serde::{de::DeserializeOwned, Serialize};

use crate::*;

// An Init-Packet is sent to end the lobby phase, and start the game,
// A Command-Packet is sent to inform, that a command has been accepted
// A DeclineCommand-Packet is sent to inform the client, that it's last Command is not allowed!

#[derive(Serialize, Deserialize)]
pub enum ServerToClientPacket {
    Init {
        world: World,
        your_id: PlayerID,
    },
    Command {
        command: Command,
        author_id: PlayerID,
    },
    DeclineCommand,
}

#[derive(Serialize, Deserialize)]
pub enum ClientToServerPacket {
    Command(Command),
}

pub trait Packet: Serialize + DeserializeOwned {}

impl Packet for ServerToClientPacket {}
impl Packet for ClientToServerPacket {}
