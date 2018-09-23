use sfml::system::{Vector2f, Vector2u};

use toalib::vec::{Vec2f, Vec2u};

pub fn vec2f_to_sfml(v: Vec2f) -> Vector2f {
	Vector2f::new(v.x, v.y)
}

pub fn vec2u_to_sfml(v: Vec2u) -> Vector2u {
	Vector2u::new(v.x, v.y)
}

pub fn vector2f_to_toa(v: Vector2f) -> Vec2f {
	Vec2f::new(v.x, v.y)
}

pub fn vector2u_to_toa(v: Vector2u) -> Vec2u {
	Vec2u::new(v.x, v.y)
}
