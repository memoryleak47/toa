mod meelee;
mod ranged;
mod long_sword;
mod lance;

pub use self::meelee::new_meelee_aim;
pub use self::ranged::new_ranged_aim;
pub use self::long_sword::new_long_sword_aim;
pub use self::lance::new_lance_aim;
use self::meelee::MeeleeAim;
use self::ranged::RangedAim;
use self::long_sword::LongSwordAim;
use self::lance::LanceAim;

use crate::vec::{Pos, Vec2i};
use crate::vec::Direction;
use crate::world::World;

trait AimTrait {
	fn apply_direction(&mut self, _d: Direction, _w: &World);
	fn exec(&self, owner_pos: Pos, _w: &mut World);
	fn get_relative_tiles(&self) -> Vec<Vec2i>;
}

macro_rules! setup {
	($($x:ident),*) => {

		// Aims are relative to it's owner
		#[derive(Clone)]
		#[derive(Serialize, Deserialize)]
		pub enum Aim {
			$(  $x($x)  ),*
		}

		impl Aim {
			pub fn apply_direction(&mut self, d: Direction, w: &World)	{ match self { $( Aim::$x(a) => a.apply_direction(d, w) ),* } }
			pub fn exec(&self, owner_pos: Pos, w: &mut World)			{ match self { $( Aim::$x(a) => a.exec(owner_pos, w) ),* } }
			pub fn get_relative_tiles(&self) -> Vec<Vec2i>				{ match self { $( Aim::$x(a) => a.get_relative_tiles() ),* } }
		}
	};
}

setup!(MeeleeAim, RangedAim, LongSwordAim, LanceAim);
