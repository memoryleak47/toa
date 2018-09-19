use std::cmp::{min, max};
use std::path::PathBuf;

use crate::vec::{Vec2u, Vec2i, Vec2f};

use crate::config::{MAP_SIZE_X, MAP_SIZE_Y};

#[derive(PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Direction {
	Up, Left, Down, Right
}

pub fn vector_iu(v: Vec2i) -> Vec2u {
	Vec2u::new(
		max(0, min(MAP_SIZE_X as i32 - 1, v.x)) as u32,
		max(0, min(MAP_SIZE_Y as i32 - 1, v.y)) as u32,
	)
}

pub fn vector_ui(v: Vec2u) -> Vec2i {
	Vec2i::new(v.x as i32, v.y as i32)
}

pub fn vector_uf(v: Vec2u) -> Vec2f {
	Vec2f::new(v.x as f32, v.y as f32)
}

pub fn vector_if(v: Vec2i) -> Vec2f {
	Vec2f::new(v.x as f32, v.y as f32)
}


impl Direction {
	pub fn to_vector(&self) -> Vec2i {
		match self {
			Direction::Up => Vec2i::new(0, -1),
			Direction::Left => Vec2i::new(-1, 0),
			Direction::Down => Vec2i::new(0, 1),
			Direction::Right => Vec2i::new(1, 0),
		}
	}

	pub fn plus_vector(&self, p: Vec2u) -> Vec2u {
		vector_iu(self.to_vector() + vector_ui(p))
	}
}

#[macro_export]
macro_rules! init2d {
	($value: expr, $width: expr, $height: expr) => {{
		use std::iter;

		iter::repeat(|| $value)
			.map(|x| x())
			.take($width * $height)
			.collect::<Vec<_>>()
	}}
}

#[macro_export]
macro_rules! index2d {
	($width: expr, $height: expr) => {{
		use crate::config::MAP_SIZE_X;

		($width as usize) + ($height as usize) * MAP_SIZE_X
	}}
}

pub fn res_dir() -> PathBuf {
	use std::env;

	let s = env::args()
		.next()
		.unwrap();

	let mut p = PathBuf::from(s);
	p.pop();
	p.pop();
	p.pop();
	p.push("res");
	p
}
