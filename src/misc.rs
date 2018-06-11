use std::cmp::{min, max};
use sfml::system::{Vector2u, Vector2i, Vector2f};

use world::{MAP_SIZE_X, MAP_SIZE_Y};

#[derive(Copy, Clone)]
pub enum Direction {
	Up, Left, Down, Right
}

pub fn vector_iu(v: Vector2i) -> Vector2u {
	Vector2u::new(
		max(0, min(MAP_SIZE_X as i32 - 1, v.x)) as u32,
		max(0, min(MAP_SIZE_Y as i32 - 1, v.y)) as u32,
	)
}

pub fn vector_ui(v: Vector2u) -> Vector2i {
	Vector2i::new(v.x as i32, v.y as i32)
}

pub fn vector_uf(v: Vector2u) -> Vector2f {
	Vector2f::new(v.x as f32, v.y as f32)
}


impl Direction {
	pub fn to_vector(&self) -> Vector2i {
		match self {
			Direction::Up => Vector2i::new(0, -1),
			Direction::Left => Vector2i::new(-1, 0),
			Direction::Down => Vector2i::new(0, 1),
			Direction::Right => Vector2i::new(1, 0),
		}
	}

	pub fn plus_vector(&self, p: Vector2u) -> Vector2u {
		vector_iu(self.to_vector() + vector_ui(p))
	}
}
