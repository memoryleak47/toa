use bincode::{serialize, deserialize};
use serde::{Serialize, de::DeserializeOwned};

use std::iter;
use std::io::{Write, Read, ErrorKind};
use std::net::{TcpStream, TcpListener};

use crate::packet::Packet;

pub struct Stream {
	stream: TcpStream,
	nonblocking: bool,
}

pub struct Listener {
	listener: TcpListener,
	nonblocking: bool,
}

impl Stream {
	fn set_nonblocking(&mut self, nb: bool) {
		if nb != self.nonblocking {
			self.stream.set_nonblocking(nb).unwrap();
			self.nonblocking = nb;
		}
	}

	pub fn connect(ip: &str) -> Stream {
		let stream = TcpStream::connect(ip).unwrap();

		Stream {
			stream,
			nonblocking: false,
		}
	}

	pub fn send<P: Packet>(&mut self, p: P) {
		let bytes = ser(p);
		let len = bytes.len();
		let len_bytes = ser(len as u32);

		self.stream.write_all(&len_bytes[..]).unwrap();
		self.stream.write_all(&bytes[..]).unwrap();
		self.stream.flush().unwrap();
	}

	pub fn receive_blocking<P: Packet>(&mut self) -> P {
		self.set_nonblocking(false);

		let mut len_bytes: [u8; 4] = [0; 4];
		self.stream.read_exact(&mut len_bytes[..]).unwrap();
		let len: u32 = deser(&len_bytes[..]);

		let mut bytes: Vec<u8> = iter::repeat(0u8)
			.take(len as usize)
			.collect();
		self.stream.read_exact(&mut bytes[..]).unwrap();
		deser(&bytes[..])
	}

	pub fn receive_nonblocking<P: Packet>(&mut self) -> Option<P> {
		self.set_nonblocking(true);

		let mut len_bytes: [u8; 4] = [0; 4];
		match self.stream.read_exact(&mut len_bytes[..]) {
			Ok(()) => {},
			Err(ref x) if x.kind() == ErrorKind::WouldBlock => return None,
			Err(x) => Err(x).unwrap()
		}
		let len: u32 = deser(&len_bytes[..]);

		let mut bytes: Vec<u8> = iter::repeat(0u8)
			.take(len as usize)
			.collect();
		match self.stream.read_exact(&mut bytes[..]) {
			Ok(()) => {},
			Err(x) => Err(x).unwrap(),
		}
		Some(deser(&bytes[..]))
	}
}

impl Listener {
	pub fn bind(ip: &str) -> Listener {
		Listener {
			listener: TcpListener::bind(ip).unwrap(),
			nonblocking: false,
		}
	}

	pub fn accept_nonblocking(&mut self) -> Option<Stream> {
		self.set_nonblocking(true);

		match self.listener.accept() {
			Ok((stream, _)) => Some(Stream { stream, nonblocking: false }),
			Err(ref x) if x.kind() == ErrorKind::WouldBlock => return None,
			Err(x) => Err(x).unwrap(),
		}
	}

	pub fn accept_blocking(&mut self) -> Stream {
		self.set_nonblocking(false);

		Stream {
			stream: self.listener.accept().unwrap().0,
			nonblocking: false,
		}
	}

	fn set_nonblocking(&mut self, nb: bool) {
		if nb != self.nonblocking {
			self.listener.set_nonblocking(nb).unwrap();
			self.nonblocking = nb;
		}
	}

}

fn ser<P: Serialize>(p: P) -> Vec<u8> {
	serialize(&p).unwrap()
}

fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
