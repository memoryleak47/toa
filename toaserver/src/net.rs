use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read, ErrorKind};

use toalib::packet::{ClientToServerPacket, ServerToClientPacket};

const PORT: u32 = 4242;

pub fn try_receiving_packet(stream: &mut TcpStream) -> Option<Result<ClientToServerPacket, String>> {
	let mut string = String::new();
	match stream.read_to_string(&mut string)
			.map_err(|x| x.kind()) {
		Ok(_) => {},
		Err(ErrorKind::WouldBlock) => return None,
		Err(x) => return Some(Err(format!("{:?}", x))),
	}
	Some(
		ClientToServerPacket::from_str(&*string)
			.map_err(|x| x.to_string())
	)
}

pub fn send_packet(packet: ServerToClientPacket, stream: &mut TcpStream) {
	let s = packet.to_string().unwrap();
	stream.write(s.as_bytes()).unwrap();
}

pub fn create_listener() -> Result<TcpListener, String> {
	let bind_string = format!("127.0.0.1:{}", PORT);
    let listener = TcpListener::bind(&*bind_string)
		.map_err(|x| x.to_string())?;
	listener.set_nonblocking(true)
		.map_err(|x| x.to_string())?;
	Ok(listener)
}
