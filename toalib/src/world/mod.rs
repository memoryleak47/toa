pub mod command_check;
pub mod command_exec;
pub mod terrainmap;
pub mod buildingmap;
pub mod unitmap;
pub mod itemmap;
pub mod aim;
pub mod damage;

pub use self::command_check::*;
pub use self::command_exec::*;
pub use self::terrainmap::*;
pub use self::buildingmap::*;
pub use self::unitmap::*;
pub use self::itemmap::*;

use crate::vec::Vec2u;
use crate::config::{MAP_SIZE_X, MAP_SIZE_Y};
use crate::world::buildingmap::Building;
use crate::item::Inventory;
use crate::world::damage::Damage;
use crate::team::{PlayerPool, PlayerID};

// represents the current world situation
#[derive(Serialize, Deserialize)]
pub struct World {
	pub terrainmap: Vec<Terrain>,
	pub buildingmap: Vec<Option<Building>>,
	pub unitmap: Vec<Option<Unit>>,
	pub itemmap: Vec<Inventory>,
	pub pool: PlayerPool,
	pub active_player_ids: Vec<PlayerID>,
}

impl World {
	pub fn gen(pool: PlayerPool) -> World {
		let ids = pool.get_ids_for_team(pool.get_starting_team());
		World {
			terrainmap: new_terrainmap(),
			buildingmap: new_buildingmap(),
			unitmap: new_unitmap(),
			itemmap: new_itemmap(),
			pool,
			active_player_ids: ids,
		}
	}

	fn on_turn_start(&mut self) {
		self.tick_unitmap();
		self.tick_itemmap();
	}

	fn reset_turn(&mut self) {
		self.refill_stamina();
	}

	pub fn get_height(&self, pos: Vec2u) -> u32 {
		self.get_building(pos)
			.map(|x| x.get_class().get_height())
			.unwrap_or(0)
	}

	pub fn damage(&mut self, p: Vec2u, damage: Damage) {
		if let Some(x) = self.get_building_mut(p) {
			if x.damage(damage) {
				self.set_building(p, None);
			}
			return;
		}
		if let Some(x) = self.get_unit_mut(p) {
			if x.damage(damage) {
				self.set_unit(p, None);
			}
			return;
		}
		self.get_inventory_mut(p).damage(damage);
	}
}
