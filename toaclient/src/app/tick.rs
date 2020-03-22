use crate::app::App;

use toalib::command::Command;
use toalib::packet::ServerToClientPacket;

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
			Some(ServerToClientPacket::DeclineCommand) => self.command_declined(),
			Some(_) => panic!("got wrong packet while running!"),
			None => {},
		}
	}

	fn command_accepted(&mut self) {
		self.apply_menu_commands(&self.pending.clone());
		self.pending = vec![];
	}

	fn command_declined(&mut self) {
		println!("Your command has been declined!\nMaybe some other player did a move which prevents your move?\nOtherwise this is a bug.");
		self.pending = vec![];
	}
}
