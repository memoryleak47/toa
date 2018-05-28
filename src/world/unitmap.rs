use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Shape, Color, Transformable};
use sfml::system::{Vector2f, Vector2u};

use view::View;

use world::{World, TILESIZE, MAP_SIZE_X, MAP_SIZE_Y};
use misc::{vector_uf};

const FULL_STAMINA: u32 = 100;
const FULL_HEALTH: u32 = 100;
const FULL_FOOD: u32 = 100;
const FOOD_PER_TURN: u32 = 4;
const HUNGER_DAMAGE: u32 = 10;

#[derive(Copy, Clone, Debug)]
pub struct Unit {
	pub owner: u32,
	pub stamina: u32,
	pub health: u32,
	pub food: u32,
}

impl Unit {
	fn new(owner: u32) -> Unit {
		Unit {
			owner,
			stamina: FULL_STAMINA,
			health: FULL_HEALTH,
			food: FULL_FOOD,
		}
	}

	fn get_color(&self) -> Color {
		if self.owner == 0 {
			Color::rgb(255, 0, 0)
		} else {
			Color::rgb(0, 0, 255)
		}
	}
}

pub fn new_unitmap() -> [[Option<Unit>; MAP_SIZE_Y]; MAP_SIZE_X] {
	let mut unitmap = [[None; MAP_SIZE_Y]; MAP_SIZE_X];

	unitmap[MAP_SIZE_X / 2][0] = Some(Unit::new(0));
	unitmap[MAP_SIZE_X / 2][MAP_SIZE_Y - 1] = Some(Unit::new(1));

	unitmap
}


impl World {
	pub fn render_unitmap(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE_X {
			for y in 0..MAP_SIZE_Y {
				if let Some(unit) = self.unitmap[x][y] {
					let posf = Vector2f::new(x as f32, y as f32);

					let mut shape = CircleShape::new(TILESIZE / 2.0, 200);
					shape.set_fill_color(&unit.get_color());
					shape.set_position((posf - view.focus_position) * TILESIZE + vector_uf(window.size()) / 2.0);
					window.draw(&shape);
				}
			}
		}
	}

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
				let opt = &mut self.unitmap[x][y];
				if let Some(ref mut unit) = opt {
					if unit.food == 0 {
						unit.health = unit.health.saturating_sub(HUNGER_DAMAGE);
						if unit.health == 0 {
							*opt = None;
						}
					}
				}
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
