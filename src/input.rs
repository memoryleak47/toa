use std::collections::HashMap;
use sfml::window::Key;

use misc::Direction;

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

	pub fn are_pressed_mod(&self, keys: &[Key], modulo: u32) -> bool {
		keys.iter()
			.map(|x| &self.keymap[x])
			.all(|x| x.pressed)
		&&
		keys.iter()
			.map(|x| &self.keymap[x])
			.map(|x| x.time)
			.min()
			.filter(|m| m % modulo == 0)
			.is_some()
	}

	fn tick_keys(&mut self) {
		let mut keymap = HashMap::new();

		for key in get_all_keys().iter() {
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

	for key in get_all_keys().iter() {
		keymap.insert(*key, KeyState { time: 0, pressed: key.is_pressed() });
	}

	keymap
}

fn get_all_keys() -> Vec<Key> {
	let mut v = Vec::new();

	for x in (Key::Unknown as i32)..(Key::Count as i32) {
		v.push(unsafe { ::std::mem::transmute(x) });
	}

	v
}
