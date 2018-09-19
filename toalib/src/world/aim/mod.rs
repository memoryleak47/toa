mod meelee;

pub use self::meelee::new_meelee_aim;
use self::meelee::MeeleeAim;

use crate::vec::{Vec2u, Vec2i};
use crate::misc::Direction;
use crate::world::World;
use crate::world::damage::Damage;

trait AimTrait {
	fn apply_direction(&mut self, _d: Direction, _w: &World);
	fn exec(&self, owner_pos: Vec2u, _w: &mut World);
	fn get_relative_tiles(&self) -> Vec<Vec2i>;
}

macro_rules! setup {
	($($x:ident),*) => {

		// Aims are relative to it's owner
		#[derive(Clone)]
		pub enum Aim {
			$(  $x($x)  ),*
		}

		impl Aim {
			pub fn apply_direction(&mut self, d: Direction, w: &World)	{ match self { $( Aim::$x(a) => a.apply_direction(d, w) ),* } }
			pub fn exec(&self, owner_pos: Vec2u, w: &mut World)			{ match self { $( Aim::$x(a) => a.exec(owner_pos, w) ),* } }
			pub fn get_relative_tiles(&self) -> Vec<Vec2i>				{ match self { $( Aim::$x(a) => a.get_relative_tiles() ),* } }
		}
	};
}

setup!(MeeleeAim);
