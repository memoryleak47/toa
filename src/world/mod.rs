pub mod command_exec;
pub mod terrainmap;
pub mod buildingmap;
pub mod unitmap;
pub mod itemmap;

pub use self::command_exec::*;
pub use self::terrainmap::*;
pub use self::buildingmap::*;
pub use self::unitmap::*;
pub use self::itemmap::*;

use sfml::system::Vector2f;

use item::Item;

pub const TILESIZE: f32 = 20.;
pub const MAP_SIZE_X: usize = 42;
pub const MAP_SIZE_Y: usize = 54;

#[allow(non_snake_case)]
pub fn TILESIZE_VEC() -> Vector2f {
	Vector2f::new(TILESIZE, TILESIZE)
}

// represents the current world situation
pub struct World {
	pub terrainmap: [[Terrain; MAP_SIZE_Y]; MAP_SIZE_X],
	pub buildingmap: [[Option<Building>; MAP_SIZE_Y]; MAP_SIZE_X],
	pub unitmap: [[Option<Unit>; MAP_SIZE_Y]; MAP_SIZE_X],
	pub itemmap: [[Option<Item>; MAP_SIZE_Y]; MAP_SIZE_X],
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
	}

	fn reset_turn(&mut self) {
		self.refill_stamina();
	}
}
