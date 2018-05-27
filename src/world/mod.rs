mod tilemap;
mod buildingmap;
mod unitmap;

use sfml::graphics::RenderWindow;
use sfml::system::Vector2u;

pub use world::tilemap::{TILESIZE, MAP_SIZE, TILESIZE_VEC};

use world::tilemap::TileMap;
use world::buildingmap::BuildingMap;
use world::unitmap::UnitMap;

use player::Player;
use view::View;

struct Unit {
	owner: u8,
}

pub enum Command {
	Move { from: Vector2u, to: Vector2u },
	Fight { from: Vector2u, to: Vector2u },
	NextTurn,
}

// represents the current world situation
pub struct World {
	tilemap: TileMap,
	buildingmap: BuildingMap,
	unitmap: UnitMap,
	active_player: u8,
}

impl World {
	pub fn gen() -> World {
		World {
			tilemap: TileMap::gen(),
			buildingmap: BuildingMap::gen(),
			unitmap: UnitMap::gen(),
			active_player: 0,
		}
	}

	pub fn render(&self, w: &mut RenderWindow, view: &View) {
		self.tilemap.render(w, view);
		self.buildingmap.render(w, view);
		self.unitmap.render(w, view);
	}

	pub fn tick(&mut self, players: &[Box<Player>; 2], view: &mut View) {
		view.handle_basic_keys();

		if let Some(command) = players[self.active_player as usize].fetch_command(view) {
			self.exec(command, view);
		}
	}

	fn exec(&mut self, command: Command, view: &mut View) {
		match command {
			Command::Move { from, to } => {
				if self.unitmap.try_move(from, to, self.active_player) {
					view.action = None;
					view.marked_tile = to;
				}
			},
			Command::Fight { from, to } => {
				// TODO
			},
			Command::NextTurn => {
				self.active_player = 1 - self.active_player;

				if self.active_player == 0 {
					self.reset_turn();
				}
			},
		}
	}

	fn reset_turn(&mut self) {
		self.unitmap.refill_stamina();
	}
}
