use crate::*;

mod command_check;
pub use command_check::*;

mod command_exec;
pub use command_exec::*;

mod terrainmap;
pub use terrainmap::*;

mod buildingmap;
pub use buildingmap::*;

mod unitmap;
pub use unitmap::*;

mod itemmap;
pub use itemmap::*;

// represents the current world situation
#[derive(Serialize, Deserialize, Clone)]
pub struct World {
	pub terrainmap: TileMap<Terrain>,
	pub buildingmap: OptTileMap<Building>,
	pub unitmap: OptTileMap<Unit>,
	pub itemmap: TileMap<Inventory>,
	pub created_unit_counter: Vec<u32>,
	pub invested_food_counter: Vec<u32>,
	pub pool: PlayerPool,
	pub active_player_ids: Vec<PlayerID>,
}

impl World {
	pub fn gen(pool: PlayerPool) -> World {
		let ids = pool.get_ids_for_team(pool.get_starting_team());

		let player_count = pool.get_player_ids().len();

		let spawns = World::gen_spawns(&pool);

		World {
			terrainmap: new_terrainmap(),
			buildingmap: OptTileMap::new(),
			unitmap: new_unitmap(&spawns[..]),
			itemmap: TileMap::new(Inventory::new()),
			created_unit_counter: vec![0; player_count],
			invested_food_counter: vec![0; player_count],
			pool,
			active_player_ids: ids,
		}
	}

	pub fn damage(&mut self, p: Pos, damage: Damage) {
		if let Some(x) = self.buildingmap.get_mut(p) {
			if x.damage(damage) {
				self.buildingmap.set(p, None);
			}
			return;
		}
		if let Some(x) = self.unitmap.get_mut(p) {
			if x.damage(damage) {
				self.kill_unit(p);
			}
			return;
		}
		self.itemmap.get_mut(p).damage(damage);
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

	pub fn unit_cost_fn(created_unit_count: u32) -> u32 {
		2u32.pow(created_unit_count.min(7))
	}

	// returns true, iff a new unit is spawned with the next invested food
	pub fn will_spawn(&self, PlayerID(pidu): PlayerID) -> bool {
		let cost = Self::unit_cost_fn(self.created_unit_counter[pidu]);
		self.invested_food_counter[pidu] == cost
	}
}
