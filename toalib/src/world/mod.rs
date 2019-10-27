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

use std::iter;

use crate::vec::Pos;
use crate::config::{MAP_SIZE_X, MAP_SIZE_Y};
use crate::world::buildingmap::Building;
use crate::item::{ItemClass, Inventory};
use crate::damage::Damage;
use crate::team::{PlayerPool, PlayerID};

const REQUIRED_FOOD: u32 = 10;
lazy_static! {
	static ref SPAWN_FOOD_VEC: Vec<ItemClass> = {
		iter::repeat(ItemClass::Food)
			.take(REQUIRED_FOOD as usize)
			.collect()
	};
}

// represents the current world situation
#[derive(Serialize, Deserialize, Clone)]
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

		let spawns = World::gen_spawns(&pool);

		World {
			terrainmap: new_terrainmap(),
			buildingmap: new_buildingmap(),
			unitmap: new_unitmap(&spawns[..]),
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
		self.tick_spawners();
	}

	pub fn damage(&mut self, p: Pos, damage: Damage) {
		if let Some(x) = self.get_building_mut(p) {
			if x.damage(damage) {
				self.set_building(p, None);
			}
			return;
		}
		if let Some(x) = self.get_unit_mut(p) {
			if x.damage(damage) {
				self.kill_unit(p);
			}
			return;
		}
		self.get_inventory_mut(p).damage(damage);
	}

	fn gen_spawns(pool: &PlayerPool) -> Vec<(PlayerID, Pos)> {
		let v = vec![
			Pos::build((MAP_SIZE_X/2) as i32, 0).unwrap(),
			Pos::build((MAP_SIZE_X/2) as i32, (MAP_SIZE_Y-1) as i32).unwrap(),
			Pos::build(0, (MAP_SIZE_Y/2) as i32).unwrap(),
			Pos::build((MAP_SIZE_X-1) as i32, (MAP_SIZE_Y/2) as i32).unwrap()
		];

		assert!(pool.get_player_ids().len() <= v.len());

		pool.get_player_ids()
				.into_iter()
				.zip(v.into_iter())
				.collect()
	}

	fn tick_spawners(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let p = Pos::build(x as i32, y as i32).unwrap();
				if let Some(Building::Spawner(s)) = self.get_building(p) {
					let player = s.get_player_id();
					if self.get_unit(p).is_some() { continue; }
					if self.get_inventory(p).contains_all(&SPAWN_FOOD_VEC[..]) {
						self.get_inventory_mut(p).reduce(&SPAWN_FOOD_VEC[..]);
						let new_unit = Unit::new(player);
						self.set_unit(p, Some(new_unit));
					}
				}
			}
		}
	}
}
