use sfml::graphics::Color;
use sfml::system::Vector2u;

use world::{World, MAP_SIZE_X, MAP_SIZE_Y};
use item::Inventory;

const FULL_STAMINA: u32 = 100;
const FULL_HEALTH: u32 = 100;
const FULL_FOOD: u32 = 100;
const FOOD_PER_TURN: u32 = 4;
const HUNGER_DAMAGE: u32 = 10;

#[derive(Clone, Debug)]
pub struct Unit {
	pub owner: u32,
	pub stamina: u32,
	pub health: u32,
	pub food: u32,
	pub inventory: Inventory,
}

impl Unit {
	fn new(owner: u32) -> Unit {
		Unit {
			owner,
			stamina: FULL_STAMINA,
			health: FULL_HEALTH,
			food: FULL_FOOD,
			inventory: Inventory::new(),
		}
	}

	pub fn get_color(&self) -> Color {
		if self.owner == 0 {
			Color::rgb(255, 0, 0)
		} else {
			Color::rgb(0, 0, 255)
		}
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
				let apply_hunger = |mut x: Unit| -> Unit {
					if x.food == 0 {
						x.health = x.health.saturating_sub(HUNGER_DAMAGE);
					}
					x
				};
				self.unitmap[x][y] = self.unitmap[x][y].clone()
					.map(apply_hunger)
					.filter(|x| x.health > 0);
			}
		}
	}

	pub fn refill_stamina(&mut self) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(ref mut unit) = self.unitmap[x][y].as_mut() {
					unit.stamina = FULL_STAMINA;
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
}
