use crate::aim::{Aim, AimTrait};
use crate::damage::Damage;
use crate::vec::{Pos, Vec2i, Direction};
use crate::world::World;

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct LanceAim {
	direction: Direction,
	damage: Damage,
}

impl AimTrait for LanceAim {
	fn apply_direction(&mut self, d: Direction, _w: &World) {
		self.direction = d;
	}

	fn exec(&self, owner_pos: Pos, w: &mut World) {
		for t in self.get_relative_tiles()
				.into_iter()
				.filter_map(|x| owner_pos.map(|p| p + x)) {
			w.damage(t, self.damage);
		}
	}

	fn get_relative_tiles(&self) -> Vec<Vec2i> {
		let d = *self.direction;
		vec![d, d * 2]
	}
}

pub fn new_lance_aim(damage: Damage) -> Aim {
	Aim::LanceAim(LanceAim { damage, direction: Direction::Up })
}
