mod tilemap;

use sfml::graphics::RenderWindow;

use world::tilemap::TileMap;

// represents the current world situation
pub struct World {
	tilemap: TileMap,
}

impl World {
	pub fn gen() -> World {
		World {
			tilemap: TileMap::gen()
		}
	}

	pub fn render(&self, w: &mut RenderWindow) {
		self.tilemap.render(w);
	}
}
