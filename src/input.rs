use std::collections::HashMap;
use sfml::window::Key;

use misc::Direction;

static KEYS: [Key; 13] = [Key::W, Key::A, Key::S, Key::D, Key::N, Key::Return, Key::Escape, Key::M, Key::F, Key::LControl, Key::RControl, Key::U, Key::J];
const MOVE_WAIT_TIME: u32 = 7;

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

	pub fn move_direction(&self) -> Option<Direction> {
		if self.is_pressed_mod(Key::W, MOVE_WAIT_TIME) { Some(Direction::Up) }
		else if self.is_pressed_mod(Key::A, MOVE_WAIT_TIME) { Some(Direction::Left) }
		else if self.is_pressed_mod(Key::S, MOVE_WAIT_TIME) { Some(Direction::Down) }
		else if self.is_pressed_mod(Key::D, MOVE_WAIT_TIME) { Some(Direction::Right) }
		else { None }
	}

}

fn new_keymap() -> HashMap<Key, KeyState> {
	let mut keymap = HashMap::new();

	for key in KEYS.iter() {
		keymap.insert(*key, KeyState { time: 0, pressed: key.is_pressed() });
	}

	keymap
}
