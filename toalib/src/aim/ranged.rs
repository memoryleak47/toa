use crate::aim::{Aim, AimTrait};
use crate::damage::Damage;
use crate::vec::{Pos, Vec2i};
use crate::world::World;
use crate::vec::Direction;

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

	fn exec(&self, owner_pos: Pos, w: &mut World) {
		if let Some(pos) = owner_pos.map(|x| x + self.target) {
			w.damage(pos, self.damage);
		}
	}

	fn get_relative_tiles(&self) -> Vec<Vec2i> {
		vec![self.target]
	}
}

pub fn new_ranged_aim(range: u32, damage: Damage) -> Aim {
	Aim::RangedAim(RangedAim { range, damage, target: Vec2i::with(0) })
}

fn magnitude(v: Vec2i) -> u32 {
	let v = v.map(|x| x as f32);
	(v.x * v.x + v.y * v.y).sqrt().ceil() as u32
}
