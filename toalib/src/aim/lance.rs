use crate::aim::{Aim, AimTrait};
use crate::damage::Damage;
use crate::vec::{Vec2i, Direction};
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

	fn get_relative_tiles(&self) -> Vec<Vec2i> {
		let d = *self.direction;
		vec![d, d * 2]
	}

	fn get_damage(&self) -> Damage { self.damage }
}

pub fn new_lance_aim(damage: Damage) -> Aim {
	Aim::LanceAim(LanceAim { damage, direction: Direction::Up })
}
