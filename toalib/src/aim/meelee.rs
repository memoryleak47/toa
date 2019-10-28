use crate::aim::{Aim, AimTrait};
use crate::damage::Damage;
use crate::vec::Vec2i;
use crate::world::World;
use crate::vec::Direction;

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct MeeleeAim {
	dir: Direction,
	damage: Damage,
}

impl AimTrait for MeeleeAim {
	fn apply_direction(&mut self, d: Direction, _w: &World) {
		self.dir = d;
	}

	fn get_relative_tiles(&self) -> Vec<Vec2i> {
		vec![*self.dir]
	}

	fn get_damage(&self) -> Damage { self.damage }
}

pub fn new_meelee_aim(damage: Damage) -> Aim {
	Aim::MeeleeAim(MeeleeAim { damage, dir: Direction::Up })
}
