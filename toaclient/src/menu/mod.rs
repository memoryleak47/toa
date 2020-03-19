mod menu_command;
mod design;
mod widget;

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
		for w in self.generate_widgets() {
			self.draw_widget(w);
		}
	}
}
