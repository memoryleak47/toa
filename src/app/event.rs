use crate::*;

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
			Event::KeyPressed { code, .. } => self.handle_hotkey(code),
			_ => {},
		}
	}

	fn handle_mouse_press(&mut self, p: Vec2f, b: Button) {
		if let Some(w) = self.generate_widgets().iter_mut().rfind(|w| w.collides(p)) {
			if let Button::Left = b {
				self.apply_menu_commands(w.on_click.clone());
			}
		} else {
			if let Some(p) = self.window_to_tile_position(p).to_i().to_pos() {
				self.apply_menu_commands(self.on_tile_click(p, b));
			}
		}
	}

	fn window_to_tile_position(&self, p: Vec2f) -> Vec2f {
		let halfscreen = self.window_size() / 2.;
		(p-halfscreen) / self.tilesize + self.focus_position
	}

	pub fn get_world_mouse(&self) -> Vec2f {
		let m = self.window.mouse_position();
		self.window_to_tile_position(Vec2f::new(m.x as f32, m.y as f32))
	}

	#[allow(unused)]
	pub fn fuzz(&mut self) {
		static KEYS: &[Key] = &[Key::A, Key::W, Key::S, Key::D, Key::Q, Key::E, Key::R, Key::X, Key::F, Key::B, Key::I];
		for &key in KEYS {
			if rand::random::<u8>() < 8 {
				self.handle_hotkey(key);
			}
		}
		for w in self.generate_widgets() {
			if rand::random::<u8>() < 8 {
				self.apply_menu_commands(w.on_click);
			}
		}
		for p in Pos::iter_all() {
			let d = (*p - *self.cursor).magnitude_sqr();
			let v = if d < 2 { 50 } else { 8 };
			if rand::random::<u8>() < v {
				self.on_tile_click(p, Button::Left);
			}
		}
	}
}
