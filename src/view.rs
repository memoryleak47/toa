use std::cmp::{min, max};
use sfml::system::{Vector2u, Vector2i, Vector2f};
use sfml::window::Key;

use ::world::MAP_SIZE;

pub struct ViewAction {
	kind: ViewActionKind,
	to: Vector2u,
}

pub enum ViewActionKind {
	Move,
	Fight,
}

pub struct View {
	pub focus_position: Vector2f, // the tile in the center of the screen, in map coordinates
	pub marked_tile: Vector2u,
	pub action: Option<ViewAction>,
}

impl View {
	pub fn new() -> View {
		View {
			focus_position: Vector2f::new(0., 0.),
			marked_tile: Vector2u::new(0, 0),
			action: None
		}
	}

	pub fn move_cursor(&mut self, v: Vector2i) {
		let c: &mut Vector2u = match self.action {
			None => &mut self.marked_tile,
			Some(ref mut x) => &mut x.to,
		};
		*c = Vector2u::new(
			min(MAP_SIZE as u32 - 1, max(0, c.x as i32 + v.x) as u32),
			min(MAP_SIZE as u32 - 1, max(0, c.y as i32 + v.y) as u32)
		);
	}

	pub fn handle_action_keys(&mut self) {
		if Key::M.is_pressed() {
			self.action = Some(ViewAction { to: self.marked_tile.clone(), kind: ViewActionKind::Move })
		}
	}

	pub fn handle_basic_keys(&mut self) {
		if Key::Escape.is_pressed() {
			self.action = None;
		}

		let x = Key::D.is_pressed() as i32 - Key::A.is_pressed() as i32;
		let y = Key::S.is_pressed() as i32 - Key::W.is_pressed() as i32;
		self.move_cursor(Vector2i::new(x, y));
	}
}
