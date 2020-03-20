use sfml::window::{mouse::Button, Event};

use toalib::vec::Vec2f;
use toalib::command::Command;

use crate::app::App;
use crate::menu::MenuCommand;

impl App {
	pub fn handle_event(&mut self, e: Event) {
		match e {
			Event::Closed => self.window.close(),
			Event::MouseButtonPressed { button: Button::Left, x, y } => self.handle_mouse_press((x as f32, y as f32).into()),
			_ => {},
		}
	}

	fn handle_mouse_press(&mut self, p: Vec2f) {
		if let Some(w) = self.generate_widgets().iter().rfind(|w| w.collides(p)) {
			w.on_click.iter().for_each(|c| self.handle_menu_command(c) )
		} else {
			// TODO update self.cursor
			panic!("TODO")
		}
	}

	fn handle_menu_command(&mut self, c: &MenuCommand) {
		match c {
			MenuCommand::NextTurn => self.send_command(Command::NextTurn),
		}
	}
}
