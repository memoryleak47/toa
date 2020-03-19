mod draw_command;
mod menu_command;
mod design;
mod widget;

pub use draw_command::*;
pub use menu_command::*;
pub use widget::*;

use sfml::graphics::Color;

use toalib::vec::Vec2f;

use crate::app::App;

pub enum MenuState {
	Normal,
}

impl MenuState {
	pub fn new() -> MenuState {
		MenuState::Normal
	}
}

impl App {
	pub fn render_menu(&mut self) {
		let mut draw_commands = vec![];
		for w in self.generate_widgets() {
			let widget_size = w.get_size(self.window_size());
			draw_commands.extend(w.get_draw_commands(widget_size));
		}
		for c in draw_commands {
			self.execute_draw_command(c);
		}
	}
}

