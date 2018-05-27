use std::cmp::{min, max};
use sfml::system::{Vector2u, Vector2i, Vector2f};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Shape, Color, Transformable};
use sfml::window::Key;

use input::Input;
use world::{World, Direction, MAP_SIZE, TILESIZE, Command};

const MARKED_TILE_BORDER_SIZE: u8 = 5;
const ACTION_BORDER_SIZE: u8 = 3;
const MOVE_WAIT_TIME: u32 = 7;

#[allow(non_snake_case)]
fn MARKED_TILE_COLOR() -> Color { Color::rgb(150, 150, 0) }

pub struct View {
	pub focus_position: Vector2f, // the tile in the center of the screen, in map coordinates
	pub marked_tile: Vector2u,
	pub marking_unit: bool,
}

impl View {
	pub fn new() -> View {
		View {
			focus_position: Vector2f::new(MAP_SIZE as f32 / 2., MAP_SIZE as f32  / 2.),
			marked_tile: Vector2u::new(MAP_SIZE as u32 / 2, MAP_SIZE as u32 / 2),
			marking_unit: false,
		}
	}

	pub fn move_cursor(&mut self, direction: Direction) {
		self.marked_tile = direction.plus_vector(self.marked_tile);
	}

	pub fn handle_action_keys(&mut self, w: &World, input: &Input) -> Option<Command> {
		if input.is_fresh_pressed(Key::Return) {
			if let Some(unit) = w.unitmap.get(self.marked_tile) {
				if unit.owner == w.active_player {
					self.marking_unit = true;
				}
			}
		}

		if input.is_fresh_pressed(Key::N) {
			*self = View::new();
			return Some(Command::NextTurn);
		}

		if self.marking_unit {
			if let Some(direction) = move_direction(input) {
				return Some(Command::Move { from: self.marked_tile, direction });
			}
		}

		None
	}

	pub fn handle_basic_keys(&mut self, input: &Input) {
		if input.is_pressed(Key::Escape) {
			self.marking_unit = false;
		}

		if let Some(direction) = move_direction(input) {
			if input.is_pressed(Key::LControl) || input.is_pressed(Key::RControl) {
				let v = direction.to_vector();
				self.focus_position += Vector2f::new(v.x as f32, v.y as f32);
			} else if !self.marking_unit {
				self.move_cursor(direction);
			}
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
	}
}

fn move_direction(input: &Input) -> Option<Direction> {
	if input.is_pressed_mod(Key::W, MOVE_WAIT_TIME) { Some(Direction::Up) }
	else if input.is_pressed_mod(Key::A, MOVE_WAIT_TIME) { Some(Direction::Left) }
	else if input.is_pressed_mod(Key::S, MOVE_WAIT_TIME) { Some(Direction::Down) }
	else if input.is_pressed_mod(Key::D, MOVE_WAIT_TIME) { Some(Direction::Right) }
	else { None }
}
