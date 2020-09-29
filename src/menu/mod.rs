mod design;
pub use design::*;

mod widget;
pub use widget::*;

use crate::*;

#[derive(Clone)]
pub enum MenuCommand {
	StateChange(MenuState),
	Command(Command),
	Cursor(Pos),
}

#[derive(Clone)]
pub enum MenuState {
	Normal,
	ExecItem,
	Attack(Option<usize>),
	DropItem(HashSet<usize>),
	TakeItem,
	Build,
	Craft,
}

impl App {
	pub fn render_menu(&mut self) {
		for w in self.generate_widgets() {
			self.draw_widget(w);
		}
	}

	pub fn reset_menu(&mut self) {
		self.menu_state = MenuState::Normal;
		self.msg = String::new();
	}

	pub fn apply_menu_commands(&mut self, mut cs: Vec<MenuCommand>) {
		if !self.pending.is_empty() {
			return; // while pending, inputs are ignored!
			// this is a simple way to prevent a big class of bugs
		}

		while !cs.is_empty() {
			match cs.remove(0) {
				MenuCommand::Command(c) => {
					if let Err(s) = self.world.is_valid_command(self.player_id, &c) {
						self.send_msg(format!("Invalid Cmd: {}", s));
						break;
					}

					let p = ClientToServerPacket::Command(c);
					self.stream.send(p);
					self.pending = cs;
					break;
				},
				MenuCommand::StateChange(s) => {
					self.menu_state = s;
				}
				MenuCommand::Cursor(c) => {
					self.cursor = c;
					self.menu_state = MenuState::Normal;
				},
			}
		}
		self.reset_menu_state_if_unselected();
	}

	// maybe there is a better way of doing this
	fn reset_menu_state_if_unselected(&mut self) {
		if self.world.unitmap.get(self.cursor).is_none() {
			self.menu_state = MenuState::Normal;
		}
	}

	pub fn send_msg(&mut self, msg: String) {
		println!("{}", &msg);
		self.msg = msg;
	}
}
