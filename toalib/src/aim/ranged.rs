use crate::aim::{Aim, AimTrait};
use crate::damage::Damage;
use crate::vec::{Vec2u, Vec2i};
use crate::world::World;
use crate::misc::Direction;

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct RangedAim {
	range: u32,
	target: Vec2i,
	damage: Damage,
}

impl AimTrait for RangedAim {
	fn apply_direction(&mut self, d: Direction, _w: &World) {
		let tmp = self.target + d.to_vector();
		if magnitude(tmp) <= self.range {
			self.target = tmp;
		}
	}

	fn exec(&self, owner_pos: Vec2u, w: &mut World) {
		let owner_pos = owner_pos.map(|x| x as i32);
		w.damage((owner_pos + self.target).map(|x| x as u32), self.damage);
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
