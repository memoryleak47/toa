use sfml::system::Vector2f;

pub const TILESIZE: f32 = 40.;
pub const MAP_SIZE_X: usize = 42;
pub const MAP_SIZE_Y: usize = 54;

#[allow(non_snake_case)]
pub fn TILESIZE_VEC() -> Vector2f {
	Vector2f::new(TILESIZE, TILESIZE)
}
