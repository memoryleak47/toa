use std::cmp::min;

use crate::vec::Vec2u;
use crate::world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use crate::aim::{Aim, new_meelee_aim};
use crate::damage::Damage;
use crate::item::{Inventory, Item};
use crate::team::PlayerID;

const FULL_STAMINA: u32 = 100;
const FULL_HEALTH: u32 = 100;
const FULL_FOOD: u32 = 100;
const FOOD_PER_TURN: u32 = 4;
const HUNGER_DAMAGE: u32 = 10;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Unit {
	pub owner: PlayerID,
	pub stamina: i32,
	pub health: u32,
	pub food: u32,
	pub inventory: Inventory,
	pub main_item: Option<Item>,
}

impl Unit {
	pub fn new(owner: PlayerID) -> Unit {
		Unit {
			owner,
			stamina: FULL_STAMINA as i32,
			health: FULL_HEALTH,
			food: FULL_FOOD,
			inventory: Inventory::new(),
			main_item: None,
		}
	}

	pub fn get_info_string(&self) -> String {
		format!("Unit( owner: {}, stamina: {}, health: {}, food: {}, main_item: {}, inventory: {})",
			self.owner, self.stamina, self.health, self.food, self.main_item.as_ref().map(|x| x.get_class().get_name()).unwrap_or("None"), &self.inventory.get_info_string()
		)
	}

	pub fn get_weight(&self) -> u32 {
		5 + self.inventory.get_weight() + self.main_item.as_ref().map(|x| x.get_class().get_weight()).unwrap_or(0)
	}

	pub fn aim(&self) -> Aim {
		self.main_item
			.as_ref()
			.map(|x| x.aim())
			.unwrap_or_else(|| new_meelee_aim(Damage(1)))
	}

	pub fn damage(&mut self, damage: Damage) -> bool { // returns whether the unit died
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}
}

pub fn new_unitmap(spawns: &[(PlayerID, Vec2u)]) -> Vec<Option<Unit>> {
	let mut unitmap = init2d!(None, MAP_SIZE_X, MAP_SIZE_Y);

	for (player_id, spawn) in spawns {
		unitmap[index2d!(spawn.x, spawn.y)] = Some(Unit::new(*player_id));
	}

	unitmap
}


impl World {
	pub fn tick_unitmap(&mut self) {
		self.reduce_food();
		self.apply_hunger_consequences();
	}

	fn reduce_food(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(ref mut unit) = self.unitmap[index2d!(x, y)].as_mut() {
					unit.food = unit.food.saturating_sub(FOOD_PER_TURN);
				}
			}
		}
	}

	fn apply_hunger_consequences(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let u: &mut Option<Unit> = &mut self.unitmap[index2d!(x, y)];
				if let Some(ref mut unit) = u {
					if unit.food == 0 {
						unit.health = unit.health.saturating_sub(HUNGER_DAMAGE);
					}
				}
				if u.as_mut()
						.filter(|x| x.health == 0)
						.is_some() {
					*u = None;
				}
			}
		}
	}


	pub fn refill_stamina(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(ref mut unit) = self.unitmap[index2d!(x, y)].as_mut() {
					unit.stamina = min(FULL_STAMINA as i32, unit.stamina + FULL_STAMINA as i32);
				}
			}
		}
	}

	fn next_tile(&self, tile: Vec2u) -> Vec2u {
		if tile.x < MAP_SIZE_X as u32 - 1 {
			Vec2u::new(tile.x + 1, tile.y)
		} else if tile.y < MAP_SIZE_Y as u32 - 1 {
			Vec2u::new(0, tile.y + 1)
		} else {
			Vec2u::new(0, 0)
		}
	}

	pub fn find_next_unit_tile(&self, start: Vec2u, player: PlayerID) -> Option<Vec2u> {
		let mut i = start;

		for _ in 0..(MAP_SIZE_X * MAP_SIZE_Y) {
			i = self.next_tile(i);
			if let Some(unit) = self.get_unit(i) {
				if unit.owner == player {
					return Some(i);
				}
			}
		}

		None
	}

	pub fn get_unit(&self, p: Vec2u) -> Option<&Unit> {
		self.unitmap[index2d!(p.x, p.y)].as_ref()
	}

	pub fn get_unit_mut(&mut self, p: Vec2u) -> Option<&mut Unit> {
		self.unitmap[index2d!(p.x, p.y)].as_mut()
	}

	pub fn set_unit(&mut self, p: Vec2u, unit: Option<Unit>) {
		self.unitmap[index2d!(p.x, p.y)] = unit;
	}
}
