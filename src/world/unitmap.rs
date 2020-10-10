use std::cmp::min;

use crate::*;

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
}

impl Unit {
	pub fn new(owner: PlayerID) -> Unit {
		Unit {
			owner,
			stamina: FULL_STAMINA as i32,
			health: FULL_HEALTH,
			food: FULL_FOOD,
			inventory: Inventory::new(),
		}
	}

	pub fn get_weight(&self) -> u32 {
		self.inventory.get_weight()
	}

	pub fn damage(&mut self, damage: Damage) -> bool { // returns whether the unit died
		self.health = self.health.saturating_sub(damage.0);
		self.health == 0
	}

	// returns the heaviest item
	pub fn equipped_item(&self) -> Option<&'_ Item> {
		self.inventory.iter()
			.max_by_key(|item| item.get_class().get_weight())
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
				let food_reduct = FOOD_PER_TURN + 2 * unit.get_weight() / 10;
				unit.food = unit.food.saturating_sub(food_reduct);
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

	pub fn find_next_usable_unit_tile(&self, start: Pos, player: PlayerID) -> Option<Pos> {
		let mut i = start.next_repeat();
		while i != start {
			if let Some(unit) = self.unitmap.get(i) {
				if unit.owner == player && unit.stamina > 0 {
					return Some(i);
				}
			}
			i = i.next_repeat();
		}

		None
	}

	pub fn kill_unit(&mut self, p: Pos) {
		let mut unit = None;
		mem::swap(&mut unit, self.unitmap.get_mut_raw(p));
		if let Some(mut u) = unit {
			let inv: Vec<Item> = u.inventory.get_item_vec().clone();
			self.itemmap.get_mut(p).get_item_vec()
				.extend(inv);
		}
	}
}
