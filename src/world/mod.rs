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

use misc::Direction;
use item::Inventory;

pub const TILESIZE: f32 = 20.;
pub const MAP_SIZE_X: usize = 42;
pub const MAP_SIZE_Y: usize = 54;

pub const REQUIRED_UNREFINED_WORK_STAMINA: u32 = 50;

#[allow(non_snake_case)]
pub fn TILESIZE_VEC() -> Vector2f {
	Vector2f::new(TILESIZE, TILESIZE)
}

// represents the current world situation
pub struct World {
	pub terrainmap: [[Terrain; MAP_SIZE_Y]; MAP_SIZE_X],
	pub buildingmap: [[Option<Box<Building>>; MAP_SIZE_Y]; MAP_SIZE_X],
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

	fn get_height(&self, pos: Vector2u) -> u32 {
		self.get_building(pos)
			.map(|x| x.get_class().get_height())
			.unwrap_or(0)
	}

	// TODO use mass in calculation
	fn required_walk_stamina(&self, pos: Vector2u, direction: Direction) -> u32 {
		let to = direction.plus_vector(pos);
		let terrain_summand = (self.get_terrain(pos).get_stamina_cost() + self.get_terrain(to).get_stamina_cost()) / 2;
		let height_summand = 10 * (self.get_height(pos) as i32 - self.get_height(to) as i32).abs() as u32;
		terrain_summand + height_summand
	}

	fn required_attack_stamina(&self, pos: Vector2u, to: Vector2u) -> u32 {
		10
	}
}
