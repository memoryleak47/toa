use crate::world::aim::{Aim, AimTrait};
use crate::world::damage::Damage;
use crate::vec::{Vec2u, Vec2i};
use crate::world::World;
use crate::misc::Direction;

#[derive(Clone, Copy)]
pub struct MeeleeAim {
	dir: Direction,
	damage: Damage,
}

impl AimTrait for MeeleeAim {
	fn apply_direction(&mut self, d: Direction, _w: &World) {
		self.dir = d;
	}

	fn exec(&self, owner_pos: Vec2u, w: &mut World) {
		let p = self.dir.plus_vector(owner_pos);
		w.damage(p, self.damage);
	}

	fn get_relative_tiles(&self) -> Vec<Vec2i> {
		vec![self.dir.to_vector()]
	}
}

pub fn new_meelee_aim(damage: Damage) -> Aim {
	Aim::MeeleeAim(MeeleeAim { damage, dir: Direction::Up })
}
