use sfml::window::Key;

static KEYS: [Key; 10] = [Key::W, Key::A, Key::S, Key::D, Key::N, Key::Return, Key::M, Key::F, Key::LControl, Key::RControl];

pub struct Input {
	pressed_keys: Vec<&'static Key>,
	fresh_pressed_keys: Vec<&'static Key>,
}

impl Input {
	pub fn new() -> Input {
		Input {
			pressed_keys: vec![],
			fresh_pressed_keys: vec![],
		}
	}

	pub fn tick(&mut self) {
		self.tick_keys();
	}

	pub fn is_pressed(&self, key: Key) -> bool {
		self.pressed_keys.contains(&&key)
	}

	pub fn is_fresh_pressed(&self, key: Key) -> bool {
		self.fresh_pressed_keys.contains(&&key)
	}

	fn tick_keys(&mut self) {
		let old_pressed_keys = self.pressed_keys.clone();

		self.pressed_keys = vec![];
		self.fresh_pressed_keys = vec![];

		for key in KEYS.iter() {
			if key.is_pressed() {
				self.pressed_keys.push(key);
				if !old_pressed_keys.contains(&key) {
					self.fresh_pressed_keys.push(key);
				}
			}
		}
	}
}
