use crate::app::App;

use std::mem;

use toalib::command::Command;
use toalib::packet::{ClientToServerPacket, ServerToClientPacket};

impl App {
	pub fn tick(&mut self) {
		self.window.set_active(true);

		match self.stream.receive_nonblocking() {
			Some(ServerToClientPacket::Command { author_id, command }) => {
				assert!(self.world.checked_exec(author_id, &command));
				if author_id == self.player_id {
					self.command_accepted();
				}
			},
			Some(_) => panic!("got wrong packet while running!"),
			None => {},
		}

		self.input.tick(&self.window);

		if let Some(c) = self.fetch_command() {
			let p = ClientToServerPacket::Command(c);
			self.stream.send(p);
		}
	}

	fn command_accepted(&mut self) {
		let mut pending = None;
		mem::swap(&mut pending, &mut self.pending);

		if let Some(x) = pending {
			self.execute_action(x);
		}
	}

	fn fetch_command(&mut self) -> Option<Command> {
		if self.pending.is_none() {
			// in case the cursored unit died
			if self.world.get_unit(self.cursor)
					.filter(|x| x.owner == self.player_id)
					.is_none() {
				self.unit_mode = None;
			}

			let action_infos = self.get_action_infos();

			for info in action_infos.into_iter() {
				if info.is_triggered(&self.input) {
					if let Some(c) = info.action.get_command() {
						if self.world.is_valid_command(self.player_id, &c) {
							self.pending = Some(info.action);
							return Some(c);
						}
					} else {
						self.execute_action(info.action);
					}
				}
			}
		}
		None
	}

}
