use std::cmp::{min, max};
use sfml::system::{Vector2u, Vector2i, Vector2f};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::window::Key;

use input::Input;
use world::World;
use ::world::{MAP_SIZE, TILESIZE, Command};

const MARKED_TILE_BORDER_SIZE: u8 = 5;
const ACTION_BORDER_SIZE: u8 = 3;

#[allow(non_snake_case)]
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
			focus_position: Vector2f::new(MAP_SIZE as f32 / 2., MAP_SIZE as f32  / 2.),
			marked_tile: Vector2u::new(MAP_SIZE as u32 / 2, MAP_SIZE as u32 / 2),
			action: None,
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

	pub fn handle_action_keys(&mut self, w: &World, input: &Input) -> Option<Command> {

		if input.is_fresh_pressed(Key::Return) {
			if let Some(ref action) = self.action {
				match action.kind {
					ViewActionKind::Move => return Some(Command::Move { from: self.marked_tile, to: action.to }),
					ViewActionKind::Fight => return Some(Command::Fight { from: self.marked_tile, to: action.to }),
				}
			}
		}

		if let Some(unit) = w.unitmap.get(self.marked_tile) {
			if unit.owner == w.active_player {
				if input.is_fresh_pressed(Key::M) {
					self.action = Some(ViewAction { to: self.marked_tile.clone(), kind: ViewActionKind::Move })
				}

				if input.is_fresh_pressed(Key::F) {
					self.action = Some(ViewAction { to: self.marked_tile.clone(), kind: ViewActionKind::Fight })
				}
			}
		}

		if input.is_fresh_pressed(Key::N) {
			*self = View::new();
			return Some(Command::NextTurn);
		}

		None
	}

	pub fn handle_basic_keys(&mut self, input: &Input) {
		if input.is_pressed(Key::Escape) {
			self.action = None;
		}

		let x = input.is_pressed(Key::D) as i32 - input.is_pressed(Key::A) as i32;
		let y = input.is_pressed(Key::S) as i32 - input.is_pressed(Key::W) as i32;

		if input.is_pressed(Key::LControl) || input.is_pressed(Key::RControl) {
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
