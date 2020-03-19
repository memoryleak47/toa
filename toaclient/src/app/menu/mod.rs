mod draw_command;
mod menu_command;

pub use draw_command::*;
pub use menu_command::*;

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
	pub fn generate_widgets(&self) -> Vec<Box<dyn Widget>> {
		vec![Box::new(Plane)]
	}

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

pub trait Widget {
	fn get_draw_commands(&self, widget_size: Vec2f) -> Vec<DrawCommand>;
	fn get_position(&self, window_size: Vec2f) -> Vec2f;
	fn get_size(&self, window_size: Vec2f) -> Vec2f;
	fn on_click(&self) -> Option<MenuCommand> { None }
}

pub struct Plane;

impl Widget for Plane {
	fn get_draw_commands(&self, widget_size: Vec2f) -> Vec<DrawCommand> {
		vec![DrawCommand {
			pos: (0.).into(),
			size: widget_size,
			draw_type: Color::rgb(100, 100, 100).into(),
		}]
	}

	fn get_position(&self, _: Vec2f) -> Vec2f {
		(0.).into()
	}

	fn get_size(&self, window_size: Vec2f) -> Vec2f {
		window_size * (0.25, 1.)
	}
}
