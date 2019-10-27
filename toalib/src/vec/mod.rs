mod vec2t;
pub use vec2t::*;

mod pos;
pub use pos::*;

mod direction;
pub use direction::*;

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

