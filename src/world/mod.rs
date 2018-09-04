pub mod command_check;
pub mod command_exec;
pub mod terrainmap;
pub mod buildingmap;
pub mod unitmap;
pub mod itemmap;

pub use self::command_check::*;
pub use self::command_exec::*;
pub use self::terrainmap::*;
pub use self::buildingmap::*;
pub use self::unitmap::*;
pub use self::itemmap::*;

use self::buildingmap::Building;

use sfml::system::{Vector2f, Vector2u};

use item::Inventory;

pub const TILESIZE: f32 = 40.;
pub const MAP_SIZE_X: usize = 42;
pub const MAP_SIZE_Y: usize = 54;

#[allow(non_snake_case)]
pub fn TILESIZE_VEC() -> Vector2f {
	Vector2f::new(TILESIZE, TILESIZE)
}

// represents the current world situation
pub struct World {
	pub terrainmap: [[Terrain; MAP_SIZE_Y]; MAP_SIZE_X],
	pub buildingmap: [[Option<Box<dyn Building>>; MAP_SIZE_Y]; MAP_SIZE_X],
	pub unitmap: [[Option<Unit>; MAP_SIZE_Y]; MAP_SIZE_X],
	pub itemmap: [[Inventory; MAP_SIZE_Y]; MAP_SIZE_X],
	pub active_player: u32,
}

impl World {
	pub fn gen() -> World {
		World {
			terrainmap: new_terrainmap(),
			buildingmap: new_buildingmap(),
			unitmap: new_unitmap(),
			itemmap: new_itemmap(),
			active_player: 0,
		}
	}

	fn on_turn_start(&mut self) {
		self.tick_unitmap();
		self.tick_itemmap();
	}

	fn reset_turn(&mut self) {
		self.refill_stamina();
	}

	pub fn get_height(&self, pos: Vector2u) -> u32 {
		self.get_building(pos)
			.map(|x| x.get_class().get_height())
			.unwrap_or(0)
	}
}
