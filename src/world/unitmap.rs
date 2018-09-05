use std::cmp::min;

use sfml::system::Vector2u;

use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use item::Inventory;

const FULL_STAMINA: u32 = 100;
const FULL_HEALTH: u32 = 100;
const FULL_FOOD: u32 = 100;
const FOOD_PER_TURN: u32 = 4;
const HUNGER_DAMAGE: u32 = 10;

#[derive(Clone)]
pub struct Unit {
	pub owner: u32,
	pub stamina: i32,
	pub health: u32,
	pub food: u32,
	pub inventory: Inventory,
}

impl Unit {
	pub fn new(owner: u32) -> Unit {
		Unit {
			owner,
			stamina: FULL_STAMINA as i32,
			health: FULL_HEALTH,
			food: FULL_FOOD,
			inventory: Inventory::new(),
		}
	}

	pub fn get_info_string(&self) -> String {
		format!("Unit( owner: {}, stamina: {}, health: {}, food: {}, inventory: {})", self.owner, self.stamina, self.health, self.food, &self.inventory.get_info_string())
	}

	pub fn get_weight(&self) -> u32 {
		5 + self.inventory.get_weight()
	}
}

pub fn new_unitmap() -> [[Option<Unit>; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut unitmap = init2d!(None, MAP_SIZE_X, MAP_SIZE_Y);

	unitmap[MAP_SIZE_X / 2][0] = Some(Unit::new(0));
	unitmap[MAP_SIZE_X / 2][MAP_SIZE_Y - 1] = Some(Unit::new(1));

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
				if let Some(ref mut unit) = self.unitmap[x][y].as_mut() {
					unit.food = unit.food.saturating_sub(FOOD_PER_TURN);
				}
			}
		}
	}

	fn apply_hunger_consequences(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				let u: &mut Option<Unit> = &mut self.unitmap[x][y];
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
				if let Some(ref mut unit) = self.unitmap[x][y].as_mut() {
					unit.stamina = min(FULL_STAMINA as i32, unit.stamina + FULL_STAMINA as i32);
				}
			}
		}
	}

	fn next_tile(&self, tile: Vector2u) -> Vector2u {
		if tile.x < MAP_SIZE_X as u32 - 1 {
			Vector2u::new(tile.x + 1, tile.y)
		} else if tile.y < MAP_SIZE_Y as u32 - 1 {
			Vector2u::new(0, tile.y + 1)
		} else {
			Vector2u::new(0, 0)
		}
	}

	pub fn find_next_unit_tile(&self, start: Vector2u, player: u32) -> Option<Vector2u> {
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

	pub fn get_unit(&self, p: Vector2u) -> Option<&Unit> {
		self.unitmap[p.x as usize][p.y as usize].as_ref()
	}

	pub fn get_unit_mut(&mut self, p: Vector2u) -> Option<&mut Unit> {
		self.unitmap[p.x as usize][p.y as usize].as_mut()
	}

	pub fn set_unit(&mut self, p: Vector2u, unit: Option<Unit>) {
		self.unitmap[p.x as usize][p.y as usize] = unit;
	}
}
