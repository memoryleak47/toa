use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Shape, Color, Transformable};
use sfml::system::{Vector2f, Vector2u};

use view::View;

use world::{World, TILESIZE, MAP_SIZE};
use misc::{vector_uf};

const FULL_STAMINA: u32 = 100;
const FULL_HEALTH: u32 = 100;

#[derive(Copy, Clone, Debug)]
pub struct Unit {
	pub owner: u32,
	pub stamina: u32,
	pub health: u32,
}

impl Unit {
	fn get_color(&self) -> Color {
		if self.owner == 0 {
			Color::rgb(255, 0, 0)
		} else {
			Color::rgb(0, 0, 255)
		}
	}
}

pub fn new_unitmap() -> [[Option<Unit>; MAP_SIZE]; MAP_SIZE] {
	let mut unitmap = [[None; MAP_SIZE]; MAP_SIZE];

	unitmap[MAP_SIZE / 2][0] = Some(Unit { owner: 0, stamina: FULL_STAMINA, health: FULL_HEALTH });
	unitmap[MAP_SIZE / 2][MAP_SIZE - 1] = Some(Unit { owner: 1, stamina: FULL_STAMINA, health: FULL_HEALTH });

	unitmap
}


impl World {
	pub fn render_unitmap(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
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

	pub fn refill_stamina(&mut self) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				if let Some(mut unit) = self.unitmap[x][y] {
					unit.stamina = FULL_STAMINA;
				}
			}
		}
	}

	pub fn get_unit(&self, p: Vector2u) -> Option<&Unit> {
		self.unitmap[p.x as usize][p.y as usize].as_ref()
	}

	fn next_tile(&self, tile: Vector2u) -> Vector2u {
		if tile.x < MAP_SIZE as u32- 1 {
			Vector2u::new(tile.x + 1, tile.y)
		} else if tile.y < MAP_SIZE as u32 - 1 {
			Vector2u::new(tile.x, tile.y + 1)
		} else {
			Vector2u::new(0, 0)
		}
	}

	pub fn find_next_unit_tile(&self, start: Vector2u, player: u32) -> Option<Vector2u> {
		let mut i = start;

		for _ in 0..(MAP_SIZE * MAP_SIZE) {
			i = self.next_tile(i);
			if let Some(unit) = self.get_unit(i) {
				if unit.owner == player {
					return Some(i);
				}
			}
		}

		None
	}
}
