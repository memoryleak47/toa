use sfml::system::{Vector2u, Vector2i};

use crate::misc::Direction;
use crate::world::World;
use crate::world::damage::Damage;

// Aims are relative to it's owner
pub trait Aim {
	fn apply_direction(&mut self, _d: Direction, _w: &World);
	fn exec(&self, owner_pos: Vector2u, _w: &mut World);
	fn get_relative_tiles(&self) -> Vec<Vector2i>;
	fn clone_box(&self) -> Box<dyn Aim>;
}

#[derive(Clone, Copy)]
pub struct MeeleeAim {
	dir: Direction,
	damage: Damage,
}



impl MeeleeAim {
	pub fn new(damage: Damage) -> MeeleeAim {
		MeeleeAim { damage, dir: Direction::Up }
	}
}

impl Aim for MeeleeAim {
	fn apply_direction(&mut self, d: Direction, _w: &World) {
		self.dir = d;
	}

	fn exec(&self, owner_pos: Vector2u, w: &mut World) {
		let p = self.dir.plus_vector(owner_pos);
		w.damage(p, self.damage);
	}

	fn get_relative_tiles(&self) -> Vec<Vector2i> {
		vec![self.dir.to_vector()]
	}

	fn clone_box(&self) -> Box<dyn Aim> {
		Box::new(self.clone())
	}
}
