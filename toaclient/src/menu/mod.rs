mod design;
mod widget;

pub use widget::*;

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
