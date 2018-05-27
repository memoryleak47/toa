use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Shape, Color, Transformable};
use sfml::system::{Vector2f, Vector2u};

use view::View;

use world::{TILESIZE, MAP_SIZE};

const FULL_STAMINA: u8 = 100;
const FULL_HEALTH: u8 = 100;

#[derive(Copy, Clone, Debug)]
pub struct Unit {
	pub owner: u8,
	pub stamina: u8,
	pub health: u8,
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

pub struct UnitMap {
	units: [[Option<Unit>; MAP_SIZE]; MAP_SIZE],
}

impl UnitMap {
	pub fn gen() -> UnitMap {
		let mut units = [[None; MAP_SIZE]; MAP_SIZE];

		units[MAP_SIZE / 2][0] = Some(Unit { owner: 0, stamina: FULL_STAMINA, health: FULL_HEALTH });
		units[MAP_SIZE / 2][MAP_SIZE - 1] = Some(Unit { owner: 1, stamina: FULL_STAMINA, health: FULL_HEALTH });

		UnitMap { units }
	}

	pub fn render(&self, window: &mut RenderWindow, view: &View) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				if let Some(unit) = self.units[x][y] {
					let posf = Vector2f::new(x as f32, y as f32);
					let size = window.size();

					let mut shape = CircleShape::new(TILESIZE / 2.0, 200);
					shape.set_fill_color(&unit.get_color());
					shape.set_position((posf - view.focus_position) * TILESIZE + Vector2f::new(size.x as f32, size.y as f32) / 2.0);
					window.draw(&shape);
				}
			}
		}
	}

	pub fn refill_stamina(&mut self) {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				if let Some(mut unit) = self.units[x][y] {
					unit.stamina = FULL_STAMINA;
				}
			}
		}
	}

	pub fn try_move(&mut self, from: Vector2u, to: Vector2u, player: u8) -> bool {
		if let Some(unit) = self.units[from.x as usize][from.y as usize] {
			if unit.owner == player {
				self.units[to.x as usize][to.y as usize] = Some(unit);
				self.units[from.x as usize][from.y as usize] = None;
				return true;
			}
		}
		return false;
	}

	pub fn get(&self, p: Vector2u) -> Option<&Unit> {
		self.units[p.x as usize][p.y as usize].as_ref()
	}
}
