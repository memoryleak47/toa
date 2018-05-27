use std::cmp::{min, max};
use sfml::system::{Vector2u, Vector2i, Vector2f};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::window::Key;

use ::world::{MAP_SIZE, TILESIZE, TILESIZE_VEC};

const MARKED_TILE_BORDER_SIZE: u8 = 5;
const ACTION_BORDER_SIZE: u8 = 3;

fn MARKED_TILE_COLOR() -> Color { Color::rgb(150, 150, 0) }

pub struct ViewAction {
	kind: ViewActionKind,
	to: Vector2u,
}

pub enum ViewActionKind {
	Move,
	Fight,
}

impl ViewActionKind {
	fn get_marker_color(&self) -> Color {
		match self {
			ViewActionKind::Move => Color::rgb(0, 0, 150),
			ViewActionKind::Fight => Color::rgb(150, 0, 0),
		}
	}
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

		if Key::LControl.is_pressed() || Key::RControl.is_pressed() {
			self.focus_position += Vector2f::new(x as f32, y as f32);
		} else {
			self.move_cursor(Vector2i::new(x, y));
		}
	}

	fn render_marker(&self, window: &mut RenderWindow, color: &Color, size: u8, position: Vector2u) {
		let posf = Vector2f::new(position.x as f32 * TILESIZE, position.y as f32 * TILESIZE);

		let halfscreen = Vector2f::new(window.size().x as f32, window.size().y as f32) / 2.0;

		let mut shape = RectangleShape::new();
		shape.set_fill_color(color);

		// top
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen);
		shape.set_size(Vector2f::new(TILESIZE as f32, size as f32));
		window.draw(&shape);

		// left
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen);
		shape.set_size(Vector2f::new(size as f32, TILESIZE as f32));
		window.draw(&shape);

		// bottom
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen + Vector2f::new(0., TILESIZE - size as f32));
		shape.set_size(Vector2f::new(TILESIZE as f32, size as f32));
		window.draw(&shape);

		// right
		shape.set_position((posf - self.focus_position * TILESIZE) + halfscreen + Vector2f::new(TILESIZE - size as f32, 0.));
		shape.set_size(Vector2f::new(size as f32, TILESIZE as f32));
		window.draw(&shape);
	}

	pub fn render(&self, window: &mut RenderWindow) {
		self.render_marker(window, &MARKED_TILE_COLOR(), MARKED_TILE_BORDER_SIZE, self.marked_tile);
		if let Some(ViewAction { ref kind, ref to }) = self.action {
			self.render_marker(window, &kind.get_marker_color(), ACTION_BORDER_SIZE, to.clone());
		}
	}
}
