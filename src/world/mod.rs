mod tilemap;

pub use world::tilemap::{TILESIZE, MAP_SIZE};

use sfml::graphics::RenderWindow;

use world::tilemap::TileMap;
use player::Player;
use view::View;

// represents the current world situation
pub struct World {
	tilemap: TileMap,
	active_player: u8,
}

impl World {
	pub fn gen() -> World {
		World {
			tilemap: TileMap::gen(),
			active_player: 0,
		}
	}

	pub fn render(&self, w: &mut RenderWindow, view: &View) {
		self.tilemap.render(w, view);
	}

	pub fn tick(&self, players: &[Box<Player>; 2], view: &mut View) {
		view.handle_keys();
	}
}
