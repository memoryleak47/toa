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

#[derive(Debug)]
pub enum NonBlockError {
	Error(String),
	Empty,
}

impl Stream {
	fn set_nonblocking(&mut self, nb: bool) -> Result<(), String> {
		if nb != self.nonblocking {
			self.stream.set_nonblocking(nb)
				.map_err(|x| x.to_string())?;
			self.nonblocking = nb;
		}
		Ok(())
	}

	pub fn connect(ip: &str) -> Result<Stream, String> {
		let stream = TcpStream::connect(ip)
			.map_err(|x| x.to_string())?;

		Ok(Stream {
			stream,
			nonblocking: false,
		})
	}

	pub fn send<P: Packet>(&mut self, p: P) -> Result<(), String> {
		let bytes = ser(p)?;
		let len_bytes = ser(bytes.len() as u32)?;

		self.stream.write(&len_bytes[..])
			.map_err(|x| x.to_string())?;

		self.stream.write(&bytes[..])
			.map_err(|x| x.to_string())
			.map(|_| ())
	}

	pub fn receive_blocking<P: Packet>(&mut self) -> Result<P, String> {
		self.set_nonblocking(false)?;

		let mut len_bytes: [u8; 4] = [0; 4];
		self.stream.read_exact(&mut len_bytes[..])
			.map_err(|x| x.to_string())?;
		let len: u32 = deser(&len_bytes[..])?;

		let mut bytes: Vec<u8> = iter::repeat(0u8)
			.take(len as usize)
			.collect();
		self.stream.read_exact(&mut bytes[..])
			.map_err(|x| x.to_string())?;
		let ret = deser(&bytes[..])?;
		Ok(ret)
	}

	pub fn receive_nonblocking<P: Packet>(&mut self) -> Result<P, NonBlockError> {
		self.set_nonblocking(true)
			.map_err(|x| NonBlockError::Error(x))?;

		let mut len_bytes: [u8; 4] = [0; 4];
		match self.stream.read_exact(&mut len_bytes[..]) {
			Ok(_) => {},
			Err(ref x) if x.kind() == ErrorKind::WouldBlock => return Err(NonBlockError::Empty),
			Err(x) => return Err(NonBlockError::Error(x.to_string())),
		}
		let len: u32 = deser(&len_bytes[..])
			.map_err(|x| NonBlockError::Error(x))?;

		let mut bytes: Vec<u8> = iter::repeat(0u8)
			.take(len as usize)
			.collect();
		self.stream.read_exact(&mut bytes[..])
			.map_err(|x| NonBlockError::Error(x.to_string()))?;
		let ret = deser(&bytes[..])
			.map_err(|x| NonBlockError::Error(x))?;
		Ok(ret)
	}
}

impl Listener {
	pub fn bind(ip: &str) -> Result<Listener, String> {
		Ok(Listener {
			listener: TcpListener::bind(ip).map_err(|x| x.to_string())?,
			nonblocking: false,
		})
	}

	pub fn accept_nonblocking(&mut self) -> Result<Stream, NonBlockError> {
		self.set_nonblocking(true)
			.map_err(|x| NonBlockError::Error(x))?;

		match self.listener.accept() {
			Ok((stream, _)) => Ok(Stream { stream, nonblocking: false }),
			Err(ref x) if x.kind() == ErrorKind::WouldBlock => return Err(NonBlockError::Empty),
			Err(x) => return Err(NonBlockError::Error(x.to_string())),
		}
	}

	pub fn accept_blocking(&mut self) -> Result<Stream, String> {
		self.set_nonblocking(false)?;

		self.listener.accept()
			.map(|(stream, _)| Stream { stream, nonblocking: false })
			.map_err(|x| x.to_string())
	}

	fn set_nonblocking(&mut self, nb: bool) -> Result<(), String> {
		if nb != self.nonblocking {
			self.listener.set_nonblocking(nb)
				.map_err(|x| x.to_string())?;
			self.nonblocking = nb;
		}
		Ok(())
	}

}

fn ser<P: Serialize>(p: P) -> Result<Vec<u8>, String> {
	serialize(&p)
		.map_err(|x| x.to_string())
}

fn deser<P: DeserializeOwned>(bytes: &[u8]) -> Result<P, String> {
	deserialize(bytes)
		.map_err(|x| x.to_string())
}

impl ToString for NonBlockError {
	fn to_string(&self) -> String {
		match self {
			NonBlockError::Error(x) => format!("NonBlockError::Error({})", &x),
			NonBlockError::Empty => format!("NonBlockError::Empty"),
		}
	}
}

