use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::thread::{spawn, JoinHandle};
use std::io::{stdin, stdout, Write};

pub enum TermCommand {
	Go
}

pub struct Term {
	_handle: JoinHandle<()>,
	receiver: Receiver<TermCommand>,
	commands: Vec<TermCommand>,
}

fn term_fun(sender: Sender<TermCommand>) {
	let mut s = String::new();
	loop {
		
		stdout().write(b"enter command\n>> ").unwrap();
		stdout().flush().unwrap();

		stdin().read_line(&mut s).unwrap();

		if let Some(x) = Term::parse_command(&*s) {
			sender.send(x).unwrap();
		}
	}
}

impl Term {
	pub fn new() -> Term {
		let (sender, receiver) = channel();

		let _handle = spawn(move || {
			term_fun(sender);
		});

		Term {
			_handle,
			receiver,
			commands: Vec::new(),
		}
	}

	fn tick(&mut self) {
		loop {
			match self.receiver.try_recv() {
				Ok(x) => self.commands.push(x),
				Err(TryRecvError::Empty) => break,
				Err(TryRecvError::Disconnected) => panic!("term error: Sender disconnected!"),
			}
		}
	}

	pub fn fetch_command(&mut self) -> Option<TermCommand> {
		self.tick();

		self.commands.pop()
	}

	fn parse_command(s: &str) -> Option<TermCommand> {
		match s.trim() {
			"go" => Some(TermCommand::Go),
			_ => {
				println!("unknown command!");
				None
			},
		}
	}
}
