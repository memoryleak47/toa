use sfml::window::{mouse::Button, Event};

use toalib::vec::Vec2f;

use crate::app::App;

impl App {
	pub fn handle_event(&mut self, e: Event) {
		match e {
			Event::Closed => self.window.close(),
			Event::MouseButtonPressed { button: Button::Middle, x, y } => {
				self.window_grab = Some((x as f32, y as f32).into());
			},
			Event::MouseMoved { x, y } => {
				if self.window_grab.is_some() {
					let v = Vec2f::new(x as f32, y as f32);
					self.focus_position += (self.window_grab.unwrap() - v) / self.tilesize;
					self.window_grab = Some(v);
				}
			},
			Event::MouseButtonReleased { button: Button::Middle, x, y } => {
				if self.window_grab.is_some() {
					let v = Vec2f::new(x as f32, y as f32);
					self.focus_position += (self.window_grab.unwrap() - v) / self.tilesize;
					self.window_grab = None;
				} else { println!("warning: window_grab is None but middle mouse button was pressed"); }
			},
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
				self.on_tile_click(p, b);
			}
		}
	}
}
