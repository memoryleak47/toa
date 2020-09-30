use crate::*;

impl App {
	pub fn tick(&mut self) {
		self.window.set_active(true);

		match self.stream.receive_nonblocking() {
			Some(ServerToClientPacket::Command { author_id, command }) => {
				self.world.checked_exec(author_id, &command).unwrap();
				self.animate(&command);
				if author_id == self.player_id {
					self.command_accepted();
				}
			},
			Some(ServerToClientPacket::DeclineCommand) => self.command_declined(),
			Some(_) => panic!("got wrong packet while running!"),
			None => {},
		}
		#[cfg(feature = "fuzz")] self.fuzz();
		self.tick_animationmap();
	}

	fn command_accepted(&mut self) {
		let mut cs = None;
		mem::swap(&mut self.pending, &mut cs);
		if let Some(x) = cs {
			self.apply_menu_commands(x);
		}
	}

	fn command_declined(&mut self) {
		println!("Your command has been declined!\nMaybe some other player did a move which prevents your move?\nOtherwise this is a bug.");
		self.pending = None;
	}
}
