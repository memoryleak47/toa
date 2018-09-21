use std::net::TcpStream;
use std::io::{Write, Read, ErrorKind};

use toalib::packet::{ClientToServerPacket, ServerToClientPacket};

pub fn try_receiving_packet(stream: &mut TcpStream) -> Option<Result<ServerToClientPacket, String>> {
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
