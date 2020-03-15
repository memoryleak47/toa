use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug, Error, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize)]
pub struct Vec2t<T> {
	pub x: T,
	pub y: T,
}

#[allow(dead_code)]
pub type Vec2f = Vec2t<f32>;
#[allow(dead_code)]
pub type Vec2i = Vec2t<i32>;

#[allow(dead_code)]
impl Vec2f {
	pub fn to_i(self) -> Vec2i {
		self.map(|x| x as i32)
	}
}

#[allow(dead_code)]
impl Vec2i {
	pub fn to_f(self) -> Vec2f {
		self.map(|x| x as f32)
	}
}


impl<T> Vec2t<T> {
	pub const fn new(x: T, y: T) -> Vec2t<T> {
		Vec2t { x, y }
	}
}

impl<T: Copy> Vec2t<T> {
	pub fn with(a: T) -> Vec2t<T> {
		Vec2t {
			x: a,
			y: a,
		}
	}
}

impl<T> Add<Vec2t<T>> for Vec2t<T> where T: Add<Output=T> {
	type Output = Vec2t<T>;

	fn add(self, other: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new (
			self.x + other.x,
			self.y + other.y,
		)
	}
}

impl<T> Add<T> for Vec2t<T> where T: Add<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn add(self, other: T) -> Vec2t<T> {
		Vec2t::new (
			self.x + other,
			self.y + other,
		)
	}
}

impl<T> Sub<Vec2t<T>> for Vec2t<T> where T: Sub<Output=T> {
	type Output = Vec2t<T>;

	fn sub(self, other: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl<T> Sub<T> for Vec2t<T> where T: Sub<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn sub(self, other: T) -> Vec2t<T> {
		Vec2t::new (
			self.x - other,
			self.y - other,
		)
	}
}

impl<T> Mul<Vec2t<T>> for Vec2t<T> where T: Mul<Output=T> {
	type Output = Vec2t<T>;

	fn mul(self, other: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl<T> Mul<T> for Vec2t<T> where T: Mul<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn mul(self, other: T) -> Vec2t<T> {
		Vec2t::new (
			self.x * other,
			self.y * other,
		)
	}
}

impl<T> Div<Vec2t<T>> for Vec2t<T> where T: Div<Output=T> {
	type Output = Vec2t<T>;

	fn div(self, other: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl<T> Div<T> for Vec2t<T> where T: Div<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn div(self, other: T) -> Vec2t<T> {
		Vec2t::new (
			self.x / other,
			self.y / other,
		)
	}
}

impl<T> Hash for Vec2t<T> where T: Hash {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.x.hash(h);
		self.y.hash(h);
		h.finish();
	}
}

impl<T: PartialEq> PartialEq for Vec2t<T> {
	fn eq(&self, rhs: &Self) -> bool {
		(self.x == rhs.x) && (self.y == rhs.y)
	}
}

impl<T: Eq> Eq for Vec2t<T> {}

impl<T> Vec2t<T> {
	pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec2t<U> {
		Vec2t::new(
			f(self.x),
			f(self.y),
		)
	}
}

impl<T: Clone> Clone for Vec2t<T> {
	fn clone(&self) -> Self {
		Vec2t::new(
			self.x.clone(),
			self.y.clone(),
		)
	}
}

impl<T: Copy> Copy for Vec2t<T> { }

impl<T: Display> Display for Vec2t<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({}, {})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl<T: Debug> Debug for Vec2t<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({:?}, {:?})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl Vec2f {
	pub fn magnitude(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}
