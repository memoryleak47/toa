use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::thread::{spawn, JoinHandle};
use std::io::{stdin, stdout, Write};

use toalib::team::{PlayerID, Team};

pub enum TermCommand {
	Go(Option<String>),
	Status,
	ChangeTeam {
		player_id: PlayerID,
		team: Team,
	}
}

pub struct Term {
	_handle: JoinHandle<()>,
	receiver: Receiver<TermCommand>,
	commands: Vec<TermCommand>,
}

fn term_fun(sender: Sender<TermCommand>) {
	loop {
		let mut s = String::new();
		
		stdout().write_all(b"enter command\n>> ").unwrap();
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
		let parts: Vec<_> = s.trim()
					.split_whitespace()
					.collect();
		match &parts[..] {
			["go"] => Some(TermCommand::Go(None)),
			["go", x] => Some(TermCommand::Go(Some(x.to_owned().to_string()))),
			["status"] => Some(TermCommand::Status),
			["team", player_id_str, team_str] => {
				let player_id = PlayerID(player_id_str.parse::<usize>().ok()?);
				let team = Team(team_str.parse::<usize>().ok()?);

				Some(TermCommand::ChangeTeam { player_id, team })
			},
			_ => {
				println!("unknown command!");
				None
			},
		}
	}
}
