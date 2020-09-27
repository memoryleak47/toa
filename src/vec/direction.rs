use std::ops::Deref;

use crate::*;

static UP: Vec2i = Vec2i::new(0, -1);
static LEFT: Vec2i = Vec2i::new(-1, 0);
static DOWN: Vec2i = Vec2i::new(0, 1);
static RIGHT: Vec2i = Vec2i::new(1, 0);

#[derive(PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Direction {
	Up, Left, Down, Right
}

impl Deref for Direction {
	type Target = Vec2i;

	fn deref(&self) -> &Vec2i {
		match self {
			Direction::Up => &UP,
			Direction::Left => &LEFT,
			Direction::Down => &DOWN,
			Direction::Right => &RIGHT,
		}
	}
}


