use sfml::system::Vector2u;

use misc::Direction;
use world::World;

pub trait Aim {
	fn apply_direction(&mut self, Direction, &World);
	fn exec(&self, owner_pos: Vector2u, &mut World);
}

pub struct MeeleeAim {
	dir: Direction,
}

impl MeeleeAim {
	pub fn new() -> MeeleeAim {
		MeeleeAim { dir: Direction::Up }
	}
}

impl Aim for MeeleeAim {
	fn apply_direction(&mut self, d: Direction, _w: &World) {
		self.dir = d;
	}

	fn exec(&self, owner_pos: Vector2u, w: &mut World) {
		let p = self.dir.plus_vector(owner_pos);
		w.damage(p);
	}
}
