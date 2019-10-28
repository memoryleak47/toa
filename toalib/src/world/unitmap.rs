use std::cmp::min;
use std::mem::swap;

use crate::vec::Pos;
use crate::tilemap::OptTileMap;
use crate::world::World;
use crate::aim::{Aim, new_meelee_aim};
use crate::damage::Damage;
use crate::item::{Inventory, Item, ItemClass};
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
		self.inventory.get_weight() + self.main_item.as_ref().map(|x| x.get_class().get_weight()).unwrap_or(0)
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

pub fn new_unitmap(spawns: &[(PlayerID, Pos)]) -> OptTileMap<Unit> {
	let mut unitmap = <OptTileMap<Unit>>::new();

	for (player_id, spawn) in spawns {
		let mut u = Unit::new(*player_id);
		u.inventory.push(ItemClass::SettlementKit.build());
		unitmap.set(*spawn, Some(u));
	}

	unitmap
}


impl World {
	pub fn tick_unitmap(&mut self) {
		self.reduce_food();
		self.apply_hunger_consequences();
	}

	fn reduce_food(&mut self) {
		for p in Pos::iter_all() {
			if let Some(ref mut unit) = self.unitmap.get_mut(p) {
				unit.food = unit.food.saturating_sub(FOOD_PER_TURN);
			}
		}
	}

	fn apply_hunger_consequences(&mut self) {
		for p in Pos::iter_all() {
			let u: &mut Option<Unit> = self.unitmap.get_mut_raw(p);
			if let Some(ref mut unit) = u {
				if unit.food == 0 {
					unit.health = unit.health.saturating_sub(HUNGER_DAMAGE);
				}
			}
			if u.as_ref()
					.filter(|x| x.health == 0)
					.is_some() {
				self.kill_unit(p);
			}
		}
	}


	pub fn refill_stamina(&mut self) {
		for p in Pos::iter_all() {
			if let Some(ref mut unit) = self.unitmap.get_mut(p) {
				unit.stamina = min(FULL_STAMINA as i32, unit.stamina + FULL_STAMINA as i32);
			}
		}
	}

	pub fn find_next_unit_tile(&self, start: Pos, player: PlayerID) -> Option<Pos> {
		let mut i = start.next_repeat();
		while i != start {
			if let Some(unit) = self.unitmap.get(i) {
				if unit.owner == player {
					return Some(i);
				}
			}
			i = i.next_repeat();
		}

		None
	}

	pub fn kill_unit(&mut self, p: Pos) {
		let mut unit = None;
		swap(&mut unit, self.unitmap.get_mut_raw(p));
		if let Some(mut u) = unit {
			let ground_inv = self.itemmap.get_mut(p).get_item_vec();
			if let Some(i) = u.main_item {
				ground_inv.push(i);
			}
			let v = u.inventory.get_item_vec();
			while let Some(x) = v.pop() { 
				ground_inv.push(x);
			}
		}
	}
}
