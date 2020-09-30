use crate::*;

// this will possibly become an enum later.
#[derive(Clone)]
pub struct Animation { remaining_ticks: u32 }

impl Animation {
	pub fn new() -> Animation {
		Animation { remaining_ticks: 10 }
	}
}

impl App {
	pub fn tick_animationmap(&mut self)  {
		for t in Pos::iter_all() {
			if let Some(x) = self.animationmap.get(t).cloned() {
				if x.remaining_ticks == 0 {
					self.animationmap.set(t, None);
				} else {
					self.animationmap.set(t, Some(Animation { remaining_ticks: x.remaining_ticks - 1 }));
				}
			}
		}
	}

	pub fn animate(&mut self, c: &Command) {
		if let Command::UnitCommand { command: UnitCommand::Attack(a,b), pos } = c {
			let unit = self.world.unitmap.get(*pos).unwrap();
			// TODO extract this into some function
			let rel_tiles = if let Some(i) = a {
				unit.inventory.iter().nth(*i).unwrap().aim(*b)
			} else {
				melee_aim(*b)
			};
			for t in rel_tiles {
				let p = if let Some(x) = pos.map(|p| p + t) { x } else { continue; };
				self.animationmap.set(p, Some(Animation::new()));
			}

		}
	}
}