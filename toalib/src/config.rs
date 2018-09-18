use crate::vec::Vec2f;

pub const TILESIZE: f32 = 40.;
pub const MAP_SIZE_X: usize = 42;
pub const MAP_SIZE_Y: usize = 54;

#[allow(non_snake_case)]
pub fn TILESIZE_VEC() -> Vec2f {
	Vec2f::new(TILESIZE, TILESIZE)
}
