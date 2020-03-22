use sfml::window::{mouse::Button, Event};

use toalib::vec::{Vec2f, Direction};
use toalib::command::{UnitCommand, Command};

use crate::app::App;
use crate::menu::MenuCommand;

impl App {
	pub fn handle_event(&mut self, e: Event) {
		match e {
			Event::Closed => self.window.close(),
			Event::MouseButtonReleased { button: b, x, y } => self.handle_mouse_press((x as f32, y as f32).into(), b),
			Event::MouseWheelScrolled { delta, .. } => { self.tilesize += delta; }
			_ => {},
		}
	}

	fn handle_mouse_press(&mut self, p: Vec2f, b: Button) {
		if let Some(w) = self.generate_widgets().iter_mut().rfind(|w| w.collides(p)) {
			if let Button::Left = b {
				self.apply_menu_commands(w.on_click.clone());
			}
		} else {
			let halfscreen = self.window_size() / 2.;
			if let Some(p) = ((p-halfscreen) / self.tilesize - self.focus_position).to_i().to_pos() {
				if let Button::Left = b {
					self.apply_menu_commands(vec![ MenuCommand::Cursor(p) ]);
				}
				if let Button::Right = b {
					if let Some(d) = [Direction::Left, Direction::Right, Direction::Up, Direction::Down].iter()
								.find(|&d| self.cursor.map(|x| x + **d) == Some(p)) {
						self.apply_menu_commands(vec![
							MenuCommand::Command(Command::UnitCommand { command: UnitCommand::Move(*d), pos: self.cursor }),
							MenuCommand::Cursor(p)
						]);
					}
				}
			}
		}
	}
}
