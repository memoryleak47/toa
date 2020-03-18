use crate::aim::{Aim, AimTrait};
use crate::damage::Damage;
use crate::vec::{Vec2i, Direction};
use crate::world::World;

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct RangedAim {
	range: u32,
	target: Vec2i,
	damage: Damage,
}

impl AimTrait for RangedAim {
	fn apply_direction(&mut self, d: Direction, _w: &World) {
		let tmp = self.target + *d;
		if magnitude(tmp) <= self.range {
			self.target = tmp;
		}
	}

	fn get_relative_tiles(&self) -> Vec<Vec2i> {
		vec![self.target]
	}

	fn get_damage(&self) -> Damage { self.damage }
}

pub fn new_ranged_aim(range: u32, damage: Damage) -> Aim {
	Aim::RangedAim(RangedAim { range, damage, target: 0.into() })
}

fn magnitude(v: Vec2i) -> u32 {
	let v = v.map(|x| x as f32);
	(v.x * v.x + v.y * v.y).sqrt().ceil() as u32
}
