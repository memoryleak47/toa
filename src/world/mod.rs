mod tilemap;

use sfml::graphics::RenderWindow;
use sfml::system::Vector2u;

pub use world::tilemap::{TILESIZE, MAP_SIZE};

use world::tilemap::TileMap;
use player::Player;
use view::View;

pub enum Command {
	Move { from: Vector2u, to: Vector2u },
}

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

	pub fn tick(&mut self, players: &[Box<Player>; 2], view: &mut View) {
		view.handle_basic_keys();

		if let Some(command) = players[self.active_player as usize].fetch_command(view) {
			self.exec(command);
		}
	}

	fn exec(&mut self, command: Command) {
		match command {
			Command::Move { from, to } => {
			}
		}
	}
}
