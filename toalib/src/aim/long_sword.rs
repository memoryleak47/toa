use crate::aim::{Aim, AimTrait};
use crate::damage::Damage;
use crate::vec::{Pos, Vec2i, Direction};
use crate::world::World;

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct LongSwordAim {
	direction: Direction,
	damage: Damage,
}

impl AimTrait for LongSwordAim {
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
		let other = if d.x.abs() > d.y.abs() {
			Vec2i::new(0, 1)
		} else {
			Vec2i::new(1, 0)
		};
		vec![d, d + other, d - other]
	}
}

pub fn new_long_sword_aim(damage: Damage) -> Aim {
	Aim::LongSwordAim(LongSwordAim { damage, direction: Direction::Up })
}
