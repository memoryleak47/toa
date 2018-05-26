use sfml::system::{Vector2u, Vector2f};

pub struct View {
	pub focus_position: Vector2f, // the tile in the center of the screen, in map coordinates
	pub marked_tile: Vector2u,
}

impl View {
	pub fn new() -> View {
		View {
			focus_position: Vector2f::new(0., 0.),
			marked_tile: Vector2u::new(0, 0)
		}
	}
}
