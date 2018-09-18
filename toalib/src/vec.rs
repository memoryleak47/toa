use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Vec2t<T: Add<Output=T>> {
	pub x: T,
	pub y: T,
}

pub type Vec2f = Vec2t<f32>;
pub type Vec2u = Vec2t<u32>;
pub type Vec2i = Vec2t<i32>;

impl<T: Add<Output=T>> Vec2t<T> {
	pub fn new(x: T, y: T) -> Vec2t<T> {
		Vec2t { x, y }
	}
}

impl<T: Add<Output=T>> Add<Vec2t<T>> for Vec2t<T> {
	type Output = Vec2t<T>;

	fn add(self, rhs: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new(self.x + rhs.x, self.y + rhs.y)
	}
}
