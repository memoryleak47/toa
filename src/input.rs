use std::collections::HashMap;
use sfml::window::Key;

static KEYS: [Key; 13] = [Key::W, Key::A, Key::S, Key::D, Key::N, Key::Return, Key::Escape, Key::M, Key::F, Key::LControl, Key::RControl, Key::U, Key::J];

struct KeyState {
	time: u32,
	pressed: bool,
}

pub struct Input {
	keymap: HashMap<Key, KeyState>,
}

impl Input {
	pub fn new() -> Input {
		Input {
			keymap: new_keymap()
		}
	}

	pub fn tick(&mut self) {
		self.tick_keys();
	}

	pub fn is_pressed(&self, key: Key) -> bool {
		self.keymap[&key].pressed
	}

	pub fn is_fresh_pressed(&self, key: Key) -> bool {
		let state = &self.keymap[&key];
		state.pressed && state.time == 0
	}

	pub fn is_pressed_mod(&self, key: Key, modulo: u32) -> bool {
		let state = &self.keymap[&key];
		state.pressed && state.time % modulo == 0
	}

	fn tick_keys(&mut self) {
		let mut keymap = HashMap::new();

		for key in KEYS.iter() {
			let state = &self.keymap[&key];
			if state.pressed == key.is_pressed() {
				keymap.insert(*key, KeyState { time: state.time + 1, pressed: state.pressed });
			} else {
				keymap.insert(*key, KeyState { time: 0, pressed: !state.pressed });
			}
		}

		self.keymap = keymap;
	}
}

fn new_keymap() -> HashMap<Key, KeyState> {
	let mut keymap = HashMap::new();

	for key in KEYS.iter() {
		keymap.insert(*key, KeyState { time: 0, pressed: key.is_pressed() });
	}

	keymap
}
