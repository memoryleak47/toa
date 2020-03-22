mod design;
mod widget;

use toalib::command::Command;
use toalib::packet::ClientToServerPacket;
use toalib::vec::Pos;

pub use widget::*;

use crate::app::App;

#[derive(Clone)]
pub enum MenuCommand {
	StateChange(MenuState),
	Command(Command),
	Cursor(Pos),
}

#[derive(Clone)]
pub enum ItemChoiceMode {
	Attack,
//	Drop,
	//Take,
}

#[derive(Clone)]
pub enum MenuState {
	Normal,
	ItemChoice(ItemChoiceMode),
	Attack(Option<usize>),
	//DropItem, // TODO this needs some item
}

impl MenuState {
	pub fn new() -> MenuState {
		MenuState::Normal
	}
}

impl App {
	pub fn render_menu(&mut self) {
		for w in self.generate_widgets() {
			self.draw_widget(w);
		}
	}

	pub fn apply_menu_commands(&mut self, mut cs: Vec<MenuCommand>) {
		if !self.pending.is_empty() {
			return; // while pending, inputs are ignored!
			// this is a simple way to prevent a big class of bugs
		}

		while !cs.is_empty() {
			match cs.remove(0) {
				MenuCommand::Command(c) => {
					if !self.world.is_valid_command(self.player_id, &c) {
						println!("your command was invalid!");
						return;
					}

					let p = ClientToServerPacket::Command(c);
					self.stream.send(p);
					self.pending = cs;
					return;
				},
				MenuCommand::StateChange(s) => {
					self.menu_state = s;
				}
				MenuCommand::Cursor(c) => {
					self.cursor = c;
				},
			}
		}
	}
}
