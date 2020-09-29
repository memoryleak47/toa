use crate::*;

use once_cell::sync::OnceCell;

static STOP: OnceCell<()> = OnceCell::new();
const STOP_FUZZ_HOTKEY: Key = Key::P;

impl App {
	pub fn fuzz(&mut self) {
		if STOP.get().is_some() { return; }
		// stop-fuzz key
		if STOP_FUZZ_HOTKEY.is_pressed() {
			STOP.set(()).unwrap();
			return;
		}

		static KEYS: &[Key] = &[Key::A, Key::W, Key::S, Key::D, Key::Q, Key::E, Key::R, Key::X, Key::F, Key::B, Key::I];
		for &key in KEYS {
			if rand::random::<u8>() < 8 {
				self.handle_hotkey(key);
			}
		}
		if rand::random::<u8>() < 40 {
			self.handle_hotkey(Key::Return);
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
