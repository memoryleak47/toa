use std::net::TcpStream;
use std::io::{Write, Read, ErrorKind};

use toalib::packet::{ClientToServerPacket, ServerToClientPacket};

pub fn try_receiving_packet(stream: &mut TcpStream) -> Option<Result<ServerToClientPacket, String>> {
	if let Err(x) = stream.set_nonblocking(true) {
		return Some(Err(x.to_string()));
	}

	let mut string = String::new();
	match stream.read_to_string(&mut string)
			.map_err(|x| x.kind()) {
		Ok(_) => {},
		Err(ErrorKind::WouldBlock) => return None,
		Err(x) => return Some(Err(format!("{:?}", x))),
	}
	Some(
		ServerToClientPacket::from_str(&*string)
			.map_err(|x| x.to_string())
	)
}

pub fn send_packet(packet: ClientToServerPacket, stream: &mut TcpStream) {
	let s = packet.to_string().unwrap();
	stream.write(s.as_bytes()).unwrap();
}

pub fn wait_for_packet(stream: &mut TcpStream) -> Result<ServerToClientPacket, String> {
	if let Err(x) = stream.set_nonblocking(false) {
		return Err(x.to_string());
	}

	let mut string = String::new();
	stream.read_to_string(&mut string)
		.map_err(|x| x.to_string())?;

	ServerToClientPacket::from_str(&*string)
		.map_err(|x| x.to_string())
}
