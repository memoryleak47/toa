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
	pub terrainmap: TileMap<Terrain>,
	pub buildingmap: OptTileMap<Building>,
	pub unitmap: OptTileMap<Unit>,
	pub itemmap: TileMap<Inventory>,
	pub pool: PlayerPool,
	pub active_player_ids: Vec<PlayerID>,
}

impl World {
	pub fn gen(pool: PlayerPool) -> World {
		let ids = pool.get_ids_for_team(pool.get_starting_team());

		let spawns = World::gen_spawns(&pool);

		World {
			terrainmap: new_terrainmap(),
			buildingmap: OptTileMap::new(),
			unitmap: new_unitmap(&spawns[..]),
			itemmap: TileMap::new(Inventory::new()),
			pool,
			active_player_ids: ids,
		}
	}

	fn reset_turn(&mut self) {
		self.refill_stamina();
		self.tick_spawners();

		self.tick_unitmap();
		self.tick_itemmap();
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

	fn tick_spawners(&mut self) {
		for p in Pos::iter_all() {
			if let Some(Building::Spawner(s)) = self.buildingmap.get(p) {
				let player = s.get_player_id();
				if self.unitmap.get(p).is_some() { continue; }
				if self.itemmap.get(p).contains_all(&SPAWN_FOOD_VEC[..]) {
					self.itemmap.get_mut(p).reduce(&SPAWN_FOOD_VEC[..]);
					let new_unit = Unit::new(player);
					self.unitmap.set(p, Some(new_unit));
				}
			}
		}
	}
}
